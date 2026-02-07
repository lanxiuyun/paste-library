<template>
  <div 
    class="clipboard-item" 
    :class="{ 'is-hovered': isHovered, 'is-selected': isSelected }"
    draggable="true"
    @click="handleClick"
    @dblclick="handleDoubleClick"
    @contextmenu.prevent="handleContextMenu"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
    @dragstart="handleDragStart"
    @dragend="handleDragEnd"
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
            <img 
              v-if="item.thumbnail_path" 
              :src="imageSrc" 
              :alt="'图片 ' + (item.metadata?.width || 0) + 'x' + (item.metadata?.height || 0)"
            />
          </div>
          
          <!-- 文件预览 -->
          <div v-else-if="item.content_type === 'file' || item.content_type === 'folder'" class="file-preview">
            <span class="file-icon">{{ item.content_type === 'folder' ? '📁' : '📄' }}</span>
            <span class="file-name">{{ item.metadata?.file_name || item.metadata?.folder_name || getFileName(item.content) }}</span>
          </div>
          
          <!-- 多文件预览 -->
          <div v-else-if="item.content_type === 'files'" class="files-preview">
            <span class="file-icon">📚</span>
            <span class="files-count">{{ item.metadata?.item_count || item.file_paths?.length || 0 }} 个项目</span>
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
              class="action-btn" 
              title="添加到队列" 
              @click="handleQuickAction('queue')"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 2L2 7l10 5 10-5-10-5z"/>
                <path d="M2 17l10 5 10-5"/>
                <path d="M2 12l10 5 10-5"/>
              </svg>
            </button>
            <button 
              class="action-btn" 
              title="复制" 
              @click="handleQuickAction('copy')"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
              </svg>
            </button>
            <button 
              class="action-btn" 
              title="标签" 
              @click="handleQuickAction('tag')"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z"/>
                <line x1="7" y1="7" x2="7.01" y2="7"/>
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

    <!-- 标签区域 -->
    <div v-if="item.tags && item.tags.length > 0" class="tags-row">
      <span 
        v-for="tag in item.tags.slice(0, 3)" 
        :key="tag"
        class="tag-item"
        :style="{ backgroundColor: getTagColor(tag) + '20', color: getTagColor(tag) }"
      >
        {{ tag }}
      </span>
      <span v-if="item.tags.length > 3" class="tag-more">+{{ item.tags.length - 3 }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/core';
import type { ClipboardItem } from '@/types';

interface Props {
  item: ClipboardItem;
  index: number;
  isSelected?: boolean;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  click: [item: ClipboardItem];
  dblclick: [item: ClipboardItem];
  contextmenu: [event: MouseEvent, item: ClipboardItem];
  delete: [id: number];
  copy: [item: ClipboardItem];
  tag: [item: ClipboardItem];
  quickAction: [action: string, item: ClipboardItem];
}>();

const isHovered = ref(false);

// 用于区分单击和双击
let clickTimer: ReturnType<typeof setTimeout> | null = null;
let clickCount = 0;
const DOUBLE_CLICK_DELAY = 250;

// 预设标签颜色
const tagColors: Record<string, string> = {
  '收藏': '#faad14',
  '工作': '#1890ff',
  '个人': '#52c41a',
  '待办': '#ff4d4f',
  '灵感': '#722ed1',
  '重要': '#ff4d4f',
  '稍后': '#8c8c8c',
};

const getTagColor = (tag: string): string => {
  return tagColors[tag] || '#595959';
};

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

const handleClick = () => {
  clickCount++;
  
  if (clickCount === 1) {
    clickTimer = setTimeout(() => {
      // 单击逻辑
      emit('click', props.item);
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
  emit('dblclick', props.item);
};

const handleContextMenu = (event: MouseEvent) => {
  emit('contextmenu', event, props.item);
};

const handleQuickAction = (action: string) => {
  emit('quickAction', action, props.item);
};

const handleDragStart = (event: DragEvent) => {
  if (!event.dataTransfer) return;
  
  // 设置拖拽效果
  event.dataTransfer.effectAllowed = 'copy';
  
  // 根据内容类型设置不同的拖拽数据
  switch (props.item.content_type) {
    case 'text':
    case 'html':
      event.dataTransfer.setData('text/plain', props.item.content);
      if (props.item.content_type === 'html') {
        event.dataTransfer.setData('text/html', props.item.content);
      }
      break;
    case 'image':
      // 图片拖拽：设置图片 URL
      if (props.item.thumbnail_path) {
        event.dataTransfer.setData('text/uri-list', props.item.thumbnail_path);
      }
      break;
    case 'file':
    case 'folder':
    case 'files':
      // 文件拖拽：设置文件路径
      if (props.item.file_paths && props.item.file_paths.length > 0) {
        event.dataTransfer.setData('text/uri-list', props.item.file_paths.join('\n'));
      } else {
        event.dataTransfer.setData('text/plain', props.item.content);
      }
      break;
    default:
      event.dataTransfer.setData('text/plain', props.item.content);
  }
  
  // 设置拖拽图像
  const dragImage = new Image();
  dragImage.src = 'data:image/svg+xml,%3Csvg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"%3E%3Crect x="9" y="9" width="13" height="13" rx="2" ry="2"/%3E%3Cpath d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/%3E%3C/svg%3E';
  event.dataTransfer.setDragImage(dragImage, 12, 12);
};

const handleDragEnd = (event: DragEvent) => {
  if (!event.dataTransfer) return;
  event.dataTransfer.dropEffect = 'none';
};
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

/* 文件预览 */
.file-preview,
.files-preview {
  display: flex;
  align-items: center;
  gap: 8px;
}

.file-icon {
  font-size: 18px;
  line-height: 1;
}

.file-name {
  font-size: 14px;
  color: var(--text-primary, #262626);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.files-count {
  font-size: 14px;
  color: var(--text-secondary, #595959);
}

/* Hover 快捷按钮 - 绝对定位，不影响布局 */
.quick-actions {
  position: absolute;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
  display: flex;
  align-items: center;
  gap: 2px;
  padding: 2px 4px;
  background: rgba(255, 255, 255, 0.95);
  border-radius: 4px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  z-index: 10;
}

.action-btn {
  width: 22px;
  height: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 3px;
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
  width: 12px;
  height: 12px;
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
