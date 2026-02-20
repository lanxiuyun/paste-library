<script setup lang="ts">
import { computed } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/core';
import type { ClipboardItem } from '@/types';

interface Props {
  item: ClipboardItem;
  index: number;
  isSelected?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  isSelected: false,
});

const emit = defineEmits<{
  click: [item: ClipboardItem];
  doubleClick: [item: ClipboardItem];
  rightClick: [event: MouseEvent, item: ClipboardItem];
}>();

const contentPreview = computed(() => {
  let text = props.item.content;
  text = text.replace(/<[^>]*>/g, '');
  text = text.trim();
  if (text.length > 100) {
    text = text.substring(0, 100) + '...';
  }
  return text;
});

const typeLabel = computed(() => {
  switch (props.item.content_type) {
    case 'text': return '文本';
    case 'html': return 'HTML';
    case 'rtf': return 'RTF';
    case 'image': return '图片';
    case 'file': return '文件';
    case 'folder': return '文件夹';
    case 'files': return '多文件';
    default: return '未知';
  }
});

const typeColor = computed(() => {
  switch (props.item.content_type) {
    case 'text': return { bg: '#fff2e8', color: '#fa8c16' };
    case 'html': return { bg: '#e6f7ff', color: '#1890ff' };
    case 'rtf': return { bg: '#f6ffed', color: '#52c41a' };
    case 'image': return { bg: '#fff0f6', color: '#eb2f96' };
    case 'file': return { bg: '#e6f7ff', color: '#1890ff' };
    case 'folder': return { bg: '#fff7e6', color: '#fa8c16' };
    case 'files': return { bg: '#f9f0ff', color: '#722ed1' };
    default: return { bg: '#f5f5f5', color: '#8c8c8c' };
  }
});

const imageSrc = computed(() => {
  if (props.item.content_type === 'image' && props.item.thumbnail_path) {
    return convertFileSrc(props.item.thumbnail_path);
  }
  return '';
});

const formatFileSize = (bytes?: number): string => {
  if (!bytes) return '';
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
};

const fileInfo = computed(() => {
  if (props.item.content_type === 'file' && props.item.metadata?.file_name) {
    return props.item.metadata.file_name;
  }
  if (props.item.content_type === 'files' && props.item.metadata?.item_count) {
    return `${props.item.metadata.item_count} 个文件`;
  }
  return '';
});

const imageDimensions = computed(() => {
  if (props.item.content_type === 'image' && props.item.metadata) {
    const { width, height } = props.item.metadata;
    if (width && height) {
      return `${width} × ${height}`;
    }
  }
  return '';
});

const formatTime = (dateString: string): string => {
  const date = new Date(dateString);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffSec = Math.floor(diffMs / 1000);
  const diffMin = Math.floor(diffSec / 60);
  const diffHour = Math.floor(diffMin / 60);
  const diffDay = Math.floor(diffHour / 24);

  if (diffSec < 60) return '刚刚';
  if (diffMin < 60) return `${diffMin}分钟前`;
  if (diffHour < 24) return `${diffHour}小时前`;
  if (diffDay < 30) return `${diffDay}天前`;
  return date.toLocaleDateString('zh-CN');
};

const handleClick = () => {
  emit('click', props.item);
};

const handleDoubleClick = () => {
  emit('doubleClick', props.item);
};

const handleContextMenu = (event: MouseEvent) => {
  event.preventDefault();
  emit('rightClick', event, props.item);
};
</script>

<template>
  <div
    class="clipboard-item"
    :class="{ selected: isSelected }"
    @click="handleClick"
    @dblclick="handleDoubleClick"
    @contextmenu="handleContextMenu"
  >
    <!-- Type Badge -->
    <span class="type-badge" :style="{ backgroundColor: typeColor.bg, color: typeColor.color }">
      {{ typeLabel }}
    </span>

    <!-- Content Preview -->
    <div class="content-area">
      <!-- Text Content -->
      <template v-if="item.content_type === 'text' || item.content_type === 'html' || item.content_type === 'rtf'">
        <div class="text-preview">{{ contentPreview }}</div>
      </template>

      <!-- Image Content -->
      <template v-else-if="item.content_type === 'image'">
        <div class="image-preview">
          <img v-if="imageSrc" :src="imageSrc" alt="thumbnail" />
          <span v-if="imageDimensions" class="image-dimensions">{{ imageDimensions }}</span>
        </div>
      </template>

      <!-- File/Folder Content -->
      <template v-else-if="item.content_type === 'file' || item.content_type === 'folder' || item.content_type === 'files'">
        <div class="file-preview">
          <span class="file-icon">{{ item.content_type === 'folder' ? '📁' : '📄' }}</span>
          <span class="file-info">{{ fileInfo || contentPreview }}</span>
        </div>
      </template>
    </div>

    <!-- Meta Info -->
    <div class="meta-info">
      <span v-if="item.content_type === 'file'" class="file-size">{{ formatFileSize(item.metadata?.file_size) }}</span>
      <span class="time">{{ formatTime(item.created_at) }}</span>
      <span v-if="item.tags && item.tags.length > 0" class="tags">
        <span v-for="tag in item.tags" :key="tag" class="tag">{{ tag }}</span>
      </span>
    </div>

    <!-- Index Number -->
    <span class="index-number">{{ index + 1 }}</span>
  </div>
</template>

<style scoped>
.clipboard-item {
  display: flex;
  align-items: flex-start;
  padding: 12px 16px;
  border-bottom: 1px solid #f0f0f0;
  cursor: pointer;
  transition: background-color 0.2s;
  position: relative;
}

.clipboard-item:hover {
  background-color: #fafafa;
}

.clipboard-item.selected {
  background-color: #e6f7ff;
}

.type-badge {
  flex-shrink: 0;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
  margin-right: 12px;
}

.content-area {
  flex: 1;
  min-width: 0;
  overflow: hidden;
}

.text-preview {
  font-size: 14px;
  color: #333;
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
  word-break: break-all;
}

.image-preview {
  position: relative;
  display: inline-block;
}

.image-preview img {
  max-width: 200px;
  max-height: 120px;
  border-radius: 4px;
  object-fit: cover;
}

.image-dimensions {
  display: block;
  font-size: 12px;
  color: #999;
  margin-top: 4px;
}

.file-preview {
  display: flex;
  align-items: center;
  gap: 8px;
}

.file-icon {
  font-size: 20px;
}

.file-info {
  font-size: 14px;
  color: #333;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.meta-info {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: #999;
  margin-left: 12px;
  flex-shrink: 0;
}

.file-size {
  color: #666;
}

.tags {
  display: flex;
  gap: 4px;
}

.tag {
  padding: 1px 6px;
  background-color: #f0f0f0;
  border-radius: 3px;
  font-size: 11px;
}

.index-number {
  position: absolute;
  right: 12px;
  bottom: 8px;
  font-size: 18px;
  font-weight: 600;
  color: #e0e0e0;
}
</style>
