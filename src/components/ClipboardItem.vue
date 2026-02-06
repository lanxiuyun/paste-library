<template>
  <div 
    class="clipboard-item" 
    :class="{ 'is-hovered': isHovered }"
    @click="handleClick"
    @dblclick="handleDoubleClick"
    @contextmenu.prevent="handleContextMenu"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
  >
    <div class="item-row">
      <!-- 类型标签 -->
      <span class="type-badge" :class="item.content_type">
        {{ typeLabel }}
      </span>

      <!-- 内容区域 -->
      <div class="content-area">
        <!-- 图片预览 -->
        <div v-if="item.content_type === 'image'" class="image-preview">
          <img 
            v-if="item.thumbnail_path" 
            :src="'asset://localhost/' + item.thumbnail_path" 
            :alt="'图片 ' + (item.metadata?.width || 0) + 'x' + (item.metadata?.height || 0)"
          />
          <span class="image-size">
            {{ item.metadata?.width || '?' }}×{{ item.metadata?.height || '?' }}
          </span>
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
        
        <div class="meta-info">
          <span class="meta-item">{{ metaText }}</span>
          <span class="meta-separator">·</span>
          <span class="meta-item">{{ formattedTime }}</span>
          <span v-if="item.is_favorite" class="star-icon">
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
            </svg>
          </span>
        </div>
      </div>

      <!-- 序号 -->
      <span class="item-index">{{ index + 1 }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import type { ClipboardItem } from '@/types';

interface Props {
  item: ClipboardItem;
  index: number;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  click: [item: ClipboardItem];
  dblclick: [item: ClipboardItem];
  contextmenu: [event: MouseEvent, item: ClipboardItem];
  delete: [id: number];
  toggleFavorite: [id: number, isFavorite: boolean];
  copy: [item: ClipboardItem];
}>();

const isHovered = ref(false);

// 用于区分单击和双击
let clickTimer: ReturnType<typeof setTimeout> | null = null;
let clickCount = 0;
const DOUBLE_CLICK_DELAY = 250;

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

// 元信息文本
const metaText = computed(() => {
  const type = props.item.content_type;
  if (type === 'image') {
    const w = props.item.metadata?.width || 0;
    const h = props.item.metadata?.height || 0;
    return `${w}×${h}`;
  }
  if (type === 'file' || type === 'folder') {
    const size = props.item.metadata?.file_size;
    return size ? formatFileSize(size) : '文件';
  }
  if (type === 'files') {
    const count = props.item.metadata?.item_count || props.item.file_paths?.length || 0;
    return `${count} 个项目`;
  }
  return `${props.item.content.length}字符`;
});

// 格式化文件大小
const formatFileSize = (bytes: number): string => {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
  return `${(bytes / 1024 / 1024 / 1024).toFixed(1)} GB`;
};

// 从路径获取文件名
const getFileName = (path: string): string => {
  return path.split(/[/\\]/).pop() || path;
};

const formattedTime = computed(() => {
  const date = new Date(props.item.created_at);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffSec = Math.floor(diffMs / 1000);
  const diffMin = Math.floor(diffSec / 60);
  const diffHour = Math.floor(diffMin / 60);
  const diffDay = Math.floor(diffHour / 24);

  if (diffSec < 60) return '刚刚';
  if (diffMin < 60) return `${diffMin}分钟前`;
  if (diffHour < 24) return `${diffHour}小时前`;
  if (diffDay < 7) return `${diffDay}天前`;
  return date.toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' });
});

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
</script>

<style scoped>
.clipboard-item {
  padding: 10px 12px;
  border-bottom: 1px solid #f0f0f0;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.clipboard-item:hover {
  background-color: #f5f5f5;
}

.item-row {
  display: flex;
  align-items: flex-start;
  gap: 8px;
}

/* 类型标签 */
.type-badge {
  flex-shrink: 0;
  padding: 2px 6px;
  font-size: 10px;
  font-weight: 500;
  border-radius: 3px;
  line-height: 1.4;
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
}

.content-text {
  font-size: 14px;
  color: #262626;
  line-height: 1.5;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 图片预览 */
.image-preview {
  display: flex;
  align-items: center;
  gap: 8px;
}

.image-preview img {
  max-width: 48px;
  max-height: 36px;
  border-radius: 4px;
  object-fit: cover;
  background: #f5f5f5;
}

.image-size {
  font-size: 13px;
  color: #595959;
}

/* 文件预览 */
.file-preview,
.files-preview {
  display: flex;
  align-items: center;
  gap: 6px;
}

.file-icon {
  font-size: 16px;
}

.file-name {
  font-size: 14px;
  color: #262626;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.files-count {
  font-size: 14px;
  color: #595959;
}

.meta-info {
  display: flex;
  align-items: center;
  gap: 4px;
  margin-top: 2px;
}

.meta-item {
  font-size: 11px;
  color: #8c8c8c;
}

.meta-separator {
  font-size: 11px;
  color: #bfbfbf;
}

.star-icon {
  display: flex;
  align-items: center;
  color: #faad14;
}

.star-icon svg {
  width: 12px;
  height: 12px;
}

/* 序号 */
.item-index {
  flex-shrink: 0;
  width: 20px;
  text-align: right;
  font-size: 11px;
  color: #bfbfbf;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
}
</style>
