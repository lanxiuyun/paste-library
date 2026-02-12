<template>
  <div 
    class="clipboard-item" 
    :class="{ 'is-hovered': isHovered, 'is-selected': isSelected }"
    @click="handleClick"
    @dblclick="handleDoubleClick"
    @contextmenu.prevent="handleContextMenu"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
  >
    <div class="item-row" :class="{ 'has-tags': item.tags && item.tags.length > 0 }">
      <!-- 内容包装器 -->
      <div class="content-wrapper">
        <span class="type-badge" :class="item.content_type">
          {{ typeLabel }}
        </span>

        <div class="content-area">
          <!-- 图片预览 -->
          <div v-if="item.content_type === 'image'" class="image-preview">
            <div v-if="imageLoading && !imageLoadError" class="image-loading">
              <svg class="loading-spinner" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10" stroke-dasharray="60" stroke-dashoffset="20"/>
              </svg>
              <span>加载中...</span>
            </div>
            <img 
              v-if="item.thumbnail_path && !imageLoadError" 
              :key="imageSrcKey"
              :src="imageSrc" 
              :alt="'图片 ' + (item.metadata?.width || 0) + 'x' + (item.metadata?.height || 0)"
              @error="handleImageError"
              @load="handleImageLoad"
              :class="{ 'image-loading-hidden': imageLoading }"
            />
            <div v-if="imageLoadError" class="image-error">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
                <circle cx="8.5" cy="8.5" r="1.5"/>
                <polyline points="21 15 16 10 5 21"/>
              </svg>
              <span>图片加载失败</span>
            </div>
          </div>
          
          <!-- 单个图片文件预览（显示预览图+路径） -->
          <div v-else-if="isSingleImageFile" class="single-image-file-preview">
            <div class="image-file-row">
              <div v-if="fileImageLoading && !fileImageLoadError" class="file-thumbnail-loading">
                <svg class="loading-spinner-small" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="12" r="10" stroke-dasharray="60" stroke-dashoffset="20"/>
                </svg>
              </div>
              <img 
                v-if="singleImageSrc && !fileImageLoadError" 
                :key="fileImageSrcKey"
                :src="singleImageSrc" 
                :alt="getFileName(item.file_paths![0])"
                class="file-thumbnail"
                @error="handleFileImageError"
                @load="handleFileImageLoad"
                :class="{ 'image-loading-hidden': fileImageLoading }"
              />
              <div v-if="fileImageLoadError" class="file-thumbnail-error">📄</div>
              <span class="file-path">{{ item.file_paths![0] }}</span>
            </div>
          </div>

          <!-- 文件预览（非图片单个文件） -->
          <div v-else-if="item.content_type === 'file'" class="file-preview">
            <span class="file-icon">📄</span>
            <span class="file-path">{{ item.file_paths?.[0] || item.content }}</span>
          </div>

          <!-- 文件夹预览（显示路径） -->
          <div v-else-if="item.content_type === 'folder'" class="file-preview">
            <span class="file-icon">📁</span>
            <span class="file-path">{{ item.content }}</span>
          </div>

          <!-- 多文件预览（显示多行文件路径） -->
          <div v-else-if="item.content_type === 'files'" class="files-preview-list">
            <div 
              v-for="(path, idx) in (item.file_paths?.slice(0, 3) || [])" 
              :key="idx"
              class="file-path-row"
            >
              <span class="file-icon-small">{{ isImageFile(path) ? '🖼️' : (path.endsWith('/') ? '📁' : '📄') }}</span>
              <span class="file-path-text">{{ path }}</span>
            </div>
            <div v-if="(item.file_paths?.length || 0) > 3" class="more-files-hint">
              +{{ (item.file_paths?.length || 0) - 3 }} 个文件...
            </div>
          </div>
          
          <!-- 文本预览 -->
          <p v-else class="content-text">{{ contentPreview }}</p>
        </div>

        <!-- Hover 快捷按钮 - 绝对定位，不影响布局 -->
        <transition name="fade">
          <div v-if="isHovered" class="quick-actions" @click.stop>
            <button 
              class="action-btn" 
              title="详情" 
              @click="handleQuickAction('detail')"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"/>
                <line x1="12" y1="16" x2="12" y2="12"/>
                <line x1="12" y1="8" x2="12.01" y2="8"/>
              </svg>
            </button>
            <button 
              class="action-btn danger" 
              title="删除" 
              @click="handleQuickAction('delete')"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="3 6 5 6 21 6"/>
                <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
              </svg>
            </button>
          </div>
        </transition>
      </div>

      <!-- 序号/快捷数字 -->
      <span v-if="index < 9" class="item-index">{{ index + 1 }}</span>
      <span v-else class="item-index subdued">{{ index + 1 }}</span>
    </div>

    <!-- 标签区域（暂时隐藏） -->
    <!-- <div v-if="item.tags && item.tags.length > 0" class="tags-row">
      <span 
        v-for="tag in item.tags.slice(0, 3)" 
        :key="tag"
        class="tag-item"
        :style="{ backgroundColor: getTagColor(tag) + '20', color: getTagColor(tag) }"
      >
        {{ tag }}
      </span>
      <span v-if="item.tags.length > 3" class="tag-more">+{{ item.tags.length - 3 }}</span>
    </div> -->
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/core';
import type { ClipboardItem } from '@/types';

interface Props {
  item: ClipboardItem;
  index: number;
  isSelected?: boolean;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  click: [item: ClipboardItem, index: number];
  dblclick: [item: ClipboardItem, index: number];
  contextmenu: [event: MouseEvent, item: ClipboardItem];
  delete: [id: number];
  copy: [item: ClipboardItem];
  tag: [item: ClipboardItem];
  quickAction: [action: string, item: ClipboardItem];
}>();

const isHovered = ref(false);
const imageLoadError = ref(false);
const imageLoading = ref(true);
const retryCount = ref(0);
const MAX_RETRY = 5;
const RETRY_DELAY = 200; // 毫秒

// 处理图片加载错误 - 添加重试机制
const handleImageError = () => {
  if (retryCount.value < MAX_RETRY) {
    retryCount.value++;
    imageLoading.value = true;
    console.log(`Image load failed, retrying... (${retryCount.value}/${MAX_RETRY})`);
    
    // 延迟后重置 loading 状态，触发重新加载
    setTimeout(() => {
      imageLoadError.value = false;
      // 强制刷新 computed 属性
      imageSrcKey.value++;
    }, RETRY_DELAY);
  } else {
    imageLoadError.value = true;
    imageLoading.value = false;
    console.error('Failed to load image after retries:', props.item.thumbnail_path);
  }
};

// 处理图片加载成功
const handleImageLoad = () => {
  imageLoading.value = false;
  imageLoadError.value = false;
  retryCount.value = 0;
};

// 用于强制刷新图片 src
const imageSrcKey = ref(0);

// 监听 item 变化，重置图片状态
watch(() => props.item.id, () => {
  retryCount.value = 0;
  imageLoadError.value = false;
  imageLoading.value = true;
  imageSrcKey.value++;
  
  // 重置文件图片状态
  fileRetryCount.value = 0;
  fileImageLoadError.value = false;
  fileImageLoading.value = true;
  fileImageSrcKey.value++;
});

// 文件图片加载状态
const fileImageLoading = ref(true);
const fileImageLoadError = ref(false);
const fileRetryCount = ref(0);
const fileImageSrcKey = ref(0);

// 处理文件图片加载错误
const handleFileImageError = () => {
  if (fileRetryCount.value < MAX_RETRY) {
    fileRetryCount.value++;
    fileImageLoading.value = true;
    console.log(`File image load failed, retrying... (${fileRetryCount.value}/${MAX_RETRY})`);
    
    setTimeout(() => {
      fileImageLoadError.value = false;
      fileImageSrcKey.value++;
    }, RETRY_DELAY);
  } else {
    fileImageLoadError.value = true;
    fileImageLoading.value = false;
    console.error('Failed to load file image after retries:', props.item.file_paths?.[0]);
  }
};

// 处理文件图片加载成功
const handleFileImageLoad = () => {
  fileImageLoading.value = false;
  fileImageLoadError.value = false;
  fileRetryCount.value = 0;
};

// 用于区分单击和双击
let clickTimer: ReturnType<typeof setTimeout> | null = null;
let clickCount = 0;
const DOUBLE_CLICK_DELAY = 250;

// 预设标签颜色（暂时隐藏）
// const tagColors: Record<string, string> = {
//   '收藏': '#faad14',
//   '工作': '#1890ff',
//   '个人': '#52c41a',
//   '待办': '#ff4d4f',
//   '灵感': '#722ed1',
//   '重要': '#ff4d4f',
//   '稍后': '#8c8c8c',
// };

// const getTagColor = (tag: string): string => {
//   return tagColors[tag] || '#595959';
// };

const typeLabel = computed(() => {
  switch (props.item.content_type) {
    case 'text':
      return '纯文本';
    case 'html':
      return 'HTML';
    case 'image':
      return '图片';
    case 'file':
      return '文件';
    case 'folder':
      return '文件夹';
    case 'files':
      return '多文件';
    case 'rtf':
      return '富文本';
    default:
      return '文本';
  }
});

const contentPreview = computed(() => {
  let text = props.item.content;
  text = text.replace(/<[^>]*>/g, '');
  text = text.trim();
  return text || '(空内容)';
});

// 处理图片路径 - 使用 convertFileSrc 转换
const imageSrc = computed(() => {
  if (!props.item.thumbnail_path) return '';
  return convertFileSrc(props.item.thumbnail_path);
});

// 从路径获取文件名
const getFileName = (path: string): string => {
  return path.split(/[/\\]/).pop() || path;
};

// 判断是否为图片文件
const isImageFile = (path: string): boolean => {
  const ext = path.split('.').pop()?.toLowerCase();
  return ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'svg', 'ico'].includes(ext || '');
};

// 判断文件项是否为单个图片文件
const isSingleImageFile = computed(() => {
  if (props.item.content_type !== 'file' || !props.item.file_paths || props.item.file_paths.length !== 1) {
    return false;
  }
  return isImageFile(props.item.file_paths[0]);
});

// 单个图片文件的缩略图路径
const singleImageSrc = computed(() => {
  if (!isSingleImageFile.value || !props.item.file_paths || props.item.file_paths.length === 0) {
    return '';
  }
  return convertFileSrc(props.item.file_paths[0]);
});

const handleClick = () => {
  clickCount++;

  if (clickCount === 1) {
    clickTimer = setTimeout(() => {
      // 单击逻辑
      emit('click', props.item, props.index);
      clickCount = 0;
    }, DOUBLE_CLICK_DELAY);
  }
};

const handleDoubleClick = () => {
  // 清除单击计时器
  if (clickTimer) {
    clearTimeout(clickTimer);
    clickTimer = null;
  }
  clickCount = 0;
  // 双击逻辑
  emit('dblclick', props.item, props.index);
};

const handleContextMenu = (event: MouseEvent) => {
  emit('contextmenu', event, props.item);
};

const handleQuickAction = (action: string) => {
  emit('quickAction', action, props.item);
};

// 拖拽功能暂时禁用（与 Tauri 窗口拖拽冲突）
// const handleDragStart = (event: DragEvent) => {
//   if (!event.dataTransfer) return;
//   
//   // 设置拖拽效果
//   event.dataTransfer.effectAllowed = 'copy';
//   
//   // 根据内容类型设置不同的拖拽数据
//   switch (props.item.content_type) {
//     case 'text':
//     case 'html':
//       event.dataTransfer.setData('text/plain', props.item.content);
//       if (props.item.content_type === 'html') {
//         event.dataTransfer.setData('text/html', props.item.content);
//       }
//       break;
//     case 'image':
//       // 图片拖拽：设置图片 URL
//       if (props.item.thumbnail_path) {
//         event.dataTransfer.setData('text/uri-list', props.item.thumbnail_path);
//       }
//       break;
//     case 'file':
//     case 'folder':
//     case 'files':
//       // 文件拖拽：设置文件路径
//       if (props.item.file_paths && props.item.file_paths.length > 0) {
//         event.dataTransfer.setData('text/uri-list', props.item.file_paths.join('\n'));
//       } else {
//         event.dataTransfer.setData('text/plain', props.item.content);
//       }
//       break;
//     default:
//       event.dataTransfer.setData('text/plain', props.item.content);
//   }
//   
//   // 设置拖拽图像
//   const dragImage = new Image();
//   dragImage.src = 'data:image/svg+xml,%3Csvg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"%3E%3Crect x="9" y="9" width="13" height="13" rx="2" ry="2"/%3E%3Cpath d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/%3E%3C/svg%3E';
//   event.dataTransfer.setDragImage(dragImage, 12, 12);
// };

// const handleDragEnd = (event: DragEvent) => {
//   if (!event.dataTransfer) return;
//   event.dataTransfer.dropEffect = 'none';
// };
</script>

<style scoped>
.clipboard-item {
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-light, #e8e8e8);
  cursor: pointer;
  transition: background-color 0.15s ease;
  background: var(--bg-card, #ffffff);
}

.clipboard-item:hover {
  background-color: var(--bg-hover, #f5f5f5);
}

.clipboard-item.is-selected {
  background-color: var(--primary-light, #e6f7ff);
  box-shadow: inset 3px 0 0 0 var(--primary, #1890ff);
}

.item-row {
  display: flex;
  align-items: flex-start;
  gap: 8px;
}

.item-row.has-tags {
  margin-bottom: 8px;
}

/* 内容包装器 */
.content-wrapper {
  flex: 1;
  display: flex;
  align-items: flex-start;
  gap: 10px;
  min-width: 0;
  position: relative;
  padding-right: 80px; /* 为按钮预留空间 */
}

/* 类型标签 */
.type-badge {
  flex-shrink: 0;
  padding: 3px 8px;
  font-size: 11px;
  font-weight: 500;
  border-radius: 4px;
  line-height: 1.4;
  margin-top: 2px;
}

.type-badge.text {
  background: #fff2e8;
  color: #fa8c16;
}

.type-badge.html {
  background: #e6f7ff;
  color: #1890ff;
}

.type-badge.rtf {
  background: #f6ffed;
  color: #52c41a;
}

.type-badge.image {
  background: #f6ffed;
  color: #52c41a;
}

.type-badge.file {
  background: #f9f0ff;
  color: #722ed1;
}

.type-badge.folder {
  background: #f9f0ff;
  color: #722ed1;
}

.type-badge.files {
  background: #fff0f6;
  color: #eb2f96;
}

/* 内容区域 */
.content-area {
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: center;
  overflow: hidden;
}

/* 文本预览 - 最多3行 */
.content-text {
  font-size: 14px;
  color: var(--text-primary, #262626);
  line-height: 1.6;
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  text-overflow: ellipsis;
  margin: 0;
}

/* 图片预览 */
.image-preview {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  min-height: 60px;
}

.image-preview img {
  max-height: 100px;
  max-width: 120px;
  width: auto;
  height: auto;
  border-radius: 6px;
  object-fit: cover;
  background: var(--bg-hover, #f5f5f5);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.image-loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 16px 24px;
  color: #8c8c8c;
  font-size: 12px;
  gap: 8px;
  background: #f5f5f5;
  border-radius: 6px;
  min-width: 80px;
  min-height: 80px;
}

.loading-spinner {
  width: 24px;
  height: 24px;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.image-loading-hidden {
  opacity: 0;
  position: absolute;
}

.image-error {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 12px 16px;
  font-size: 12px;
  color: #ff4d4f;
  background: #fff2f0;
  border: 1px solid #ffccc7;
  border-radius: 6px;
  gap: 4px;
  min-width: 80px;
  min-height: 60px;
}

.image-error svg {
  width: 24px;
  height: 24px;
}

/* 单个图片文件预览 */
.single-image-file-preview {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.image-file-row {
  display: flex;
  align-items: flex-start;
  gap: 10px;
}

.file-thumbnail {
  width: 60px;
  height: 60px;
  object-fit: cover;
  border-radius: 4px;
  background: var(--bg-hover, #f5f5f5);
  flex-shrink: 0;
}

.file-thumbnail-loading {
  width: 60px;
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-hover, #f5f5f5);
  border-radius: 4px;
  flex-shrink: 0;
}

.loading-spinner-small {
  width: 20px;
  height: 20px;
  animation: spin 1s linear infinite;
  color: #8c8c8c;
}

.file-thumbnail-error {
  width: 60px;
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
  background: var(--bg-hover, #f5f5f5);
  border-radius: 4px;
  flex-shrink: 0;
}

/* 文件预览 */
.file-preview {
  display: flex;
  align-items: flex-start;
  gap: 8px;
}

.file-icon {
  font-size: 18px;
  line-height: 1;
  flex-shrink: 0;
  margin-top: 2px;
}

.file-path {
  font-size: 13px;
  color: var(--text-primary, #262626);
  line-height: 1.5;
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  text-overflow: ellipsis;
  word-break: break-all;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
}

/* 多文件预览列表 */
.files-preview-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.file-path-row {
  display: flex;
  align-items: flex-start;
  gap: 6px;
}

.file-icon-small {
  font-size: 14px;
  line-height: 1.4;
  flex-shrink: 0;
}

.file-path-text {
  font-size: 12px;
  color: var(--text-primary, #262626);
  line-height: 1.4;
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  text-overflow: ellipsis;
  word-break: break-all;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
}

.more-files-hint {
  font-size: 11px;
  color: var(--text-secondary, #595959);
  padding-left: 20px;
  margin-top: 2px;
}

/* Hover 快捷按钮 - 绝对定位，不影响布局 */
.quick-actions {
  position: absolute;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 8px;
  background: rgba(255, 255, 255, 0.95);
  border-radius: 6px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  z-index: 10;
}

.action-btn {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 4px;
  background: transparent;
  cursor: pointer;
  transition: all 0.15s ease;
  color: #8c8c8c;
}

.action-btn:hover {
  background: #f5f5f5;
  color: var(--primary, #262626);
}

.action-btn.danger:hover {
  background: var(--danger-bg, #fff2f0);
  color: var(--danger, #ff4d4f);
}

.action-btn svg {
  width: 16px;
  height: 16px;
}

/* 序号 */
.item-index {
  flex-shrink: 0;
  width: 24px;
  text-align: right;
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary, #595959);
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
  line-height: 24px;
}

.item-index.subdued {
  color: var(--text-disabled, #bfbfbf);
  font-weight: 400;
}

/* 标签区域 */
.tags-row {
  display: flex;
  align-items: center;
  gap: 6px;
  padding-left: calc(10px + 8px + 8px); /* 类型标签宽度 + gap */
  flex-wrap: wrap;
}

.tag-item {
  padding: 2px 8px;
  font-size: 11px;
  font-weight: 500;
  border-radius: 10px;
  white-space: nowrap;
}

.tag-more {
  font-size: 11px;
  color: var(--text-desc, #8c8c8c);
}

/* 过渡动画 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
