<template>
  <div class="paste-queue-panel" :class="{ 'is-expanded': isExpanded }">
    <!-- Header -->
    <div class="queue-header" @click="toggleExpand">
      <div class="queue-info">
        <span class="queue-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2L2 7l10 5 10-5-10-5z"/>
            <path d="M2 17l10 5 10-5"/>
            <path d="M2 12l10 5 10-5"/>
          </svg>
        </span>
        <span class="queue-title">粘贴队列</span>
        <span v-if="queueCount > 0" class="queue-count">{{ queueCount }}</span>
      </div>
      <div class="queue-actions">
        <button 
          v-if="queueCount > 0" 
          class="action-btn" 
          title="清空队列"
          @click.stop="handleClear"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="3 6 5 6 21 6"/>
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
          </svg>
        </button>
        <button class="expand-btn">
          <svg 
            viewBox="0 0 24 24" 
            fill="none" 
            stroke="currentColor" 
            stroke-width="2"
            :class="{ 'is-expanded': isExpanded }"
          >
            <path d="M6 9l6 6 6-6"/>
          </svg>
        </button>
      </div>
    </div>

    <!-- Queue Items (when expanded) -->
    <div v-if="isExpanded" class="queue-content">
      <div v-if="queue.length === 0" class="queue-empty">
        拖拽项目到此处，或点击项目添加到队列
      </div>
      
      <div v-else class="queue-list">
        <div
          v-for="(item, index) in queue"
          :key="item.id"
          class="queue-item"
          draggable="true"
          @dragstart="handleDragStart($event, index)"
          @dragover.prevent
          @drop="handleDrop($event, index)"
        >
          <span class="item-index">{{ index + 1 }}</span>
          <span class="item-type" :class="item.content_type">
            {{ getTypeLabel(item.content_type) }}
          </span>
          <span class="item-preview">{{ getPreview(item) }}</span>
          <button 
            class="remove-btn"
            @click="removeFromQueue(item.id)"
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18"/>
              <line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
          </button>
        </div>
      </div>

      <!-- Separator Settings -->
      <div class="queue-settings">
        <label class="setting-label">分隔符:</label>
        <div class="separator-options">
          <button
            class="sep-btn"
            :class="{ active: separator === 'newline' }"
            @click="separator = 'newline'"
          >
            换行
          </button>
          <button
            class="sep-btn"
            :class="{ active: separator === 'none' }"
            @click="separator = 'none'"
          >
            无
          </button>
          <button
            class="sep-btn"
            :class="{ active: separator === 'custom' }"
            @click="separator = 'custom'"
          >
            自定义
          </button>
        </div>
        <input
          v-if="separator === 'custom'"
          v-model="customSeparator"
          type="text"
          class="custom-sep-input"
          placeholder="输入分隔符"
        />
      </div>

      <!-- Preview -->
      <div v-if="queue.length > 0" class="queue-preview">
        <label class="setting-label">预览:</label>
        <div class="preview-content">{{ getMergedContent() }}</div>
      </div>

      <!-- Auto Clear Toggle -->
      <div class="queue-settings">
        <label class="setting-checkbox">
          <input v-model="autoClear" type="checkbox" />
          <span>粘贴后自动清空</span>
        </label>
      </div>

      <!-- Paste Button -->
      <button 
        v-if="queue.length > 0"
        class="paste-btn"
        @click="handlePaste"
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/>
          <rect x="8" y="2" width="8" height="4" rx="1"/>
        </svg>
        粘贴合并内容
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { usePasteQueue } from '@/composables/usePasteQueue';
import type { ClipboardItem } from '@/types';

const emit = defineEmits<{
  paste: [content: string];
}>();

const {
  queue,
  separator,
  customSeparator,
  autoClear,
  queueCount,
  removeFromQueue,
  moveItem,
  clearQueue,
  getMergedContent,
} = usePasteQueue();

const isExpanded = ref(false);
let dragIndex: number | null = null;

const toggleExpand = () => {
  isExpanded.value = !isExpanded.value;
};

const handleClear = () => {
  if (confirm('确定要清空粘贴队列吗？')) {
    clearQueue();
  }
};

const handleDragStart = (e: DragEvent, index: number) => {
  dragIndex = index;
  if (e.dataTransfer) {
    e.dataTransfer.effectAllowed = 'move';
  }
};

const handleDrop = (e: DragEvent, index: number) => {
  e.preventDefault();
  if (dragIndex !== null && dragIndex !== index) {
    moveItem(dragIndex, index);
  }
  dragIndex = null;
};

const getTypeLabel = (type: string) => {
  const labels: Record<string, string> = {
    text: '文本',
    html: 'HTML',
    rtf: 'RTF',
    image: '图片',
    file: '文件',
    folder: '文件夹',
    files: '多文件',
  };
  return labels[type] || '文本';
};

const getPreview = (item: ClipboardItem) => {
  if (item.content_type === 'image') {
    return '[图片]';
  }
  if (item.content_type === 'file' || item.content_type === 'folder') {
    return item.metadata?.file_name || item.metadata?.folder_name || '[文件]';
  }
  if (item.content_type === 'files') {
    return `${item.file_paths?.length || 0} 个文件`;
  }
  // Strip HTML for preview
  let text = item.content.replace(/<[^>]*>/g, '');
  return text.length > 30 ? text.substring(0, 30) + '...' : text;
};

const handlePaste = () => {
  const content = getMergedContent();
  emit('paste', content);
  if (autoClear.value) {
    clearQueue();
  }
};

defineExpose({
  addToQueue: usePasteQueue().addToQueue,
  removeFromQueue,
  isInQueue: usePasteQueue().isInQueue,
});
</script>

<style scoped>
.paste-queue-panel {
  background: #fff;
  border-top: 1px solid #e8e8e8;
  transition: all 0.3s ease;
}

.paste-queue-panel.is-expanded {
  max-height: 400px;
}

.queue-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 16px;
  cursor: pointer;
  user-select: none;
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.queue-header:hover {
  background: #f5f5f5;
}

.queue-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.queue-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  color: #595959;
}

.queue-icon svg {
  width: 16px;
  height: 16px;
}

.queue-title {
  font-size: 13px;
  font-weight: 500;
  color: #262626;
}

.queue-count {
  padding: 2px 8px;
  font-size: 11px;
  font-weight: 600;
  color: #fff;
  background: #262626;
  border-radius: 10px;
}

.queue-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}

.action-btn,
.expand-btn {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 4px;
  color: #8c8c8c;
  transition: all 0.15s ease;
}

.action-btn:hover,
.expand-btn:hover {
  background: #e8e8e8;
  color: #262626;
}

.action-btn svg,
.expand-btn svg {
  width: 14px;
  height: 14px;
  transition: transform 0.3s ease;
}

.expand-btn svg.is-expanded {
  transform: rotate(180deg);
}

.queue-content {
  padding: 0 16px 16px;
  max-height: 300px;
  overflow-y: auto;
}

.queue-empty {
  padding: 20px;
  text-align: center;
  font-size: 13px;
  color: #8c8c8c;
}

.queue-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin-bottom: 12px;
}

.queue-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: #f5f5f5;
  border-radius: 6px;
  cursor: move;
  transition: background-color 0.15s ease;
}

.queue-item:hover {
  background: #e8e8e8;
}

.queue-item .item-index {
  font-size: 11px;
  font-weight: 600;
  color: #8c8c8c;
  min-width: 18px;
}

.queue-item .item-type {
  padding: 2px 6px;
  font-size: 10px;
  font-weight: 500;
  border-radius: 3px;
  flex-shrink: 0;
}

.queue-item .item-type.text {
  background: #fff2e8;
  color: #fa8c16;
}

.queue-item .item-type.html {
  background: #e6f7ff;
  color: #1890ff;
}

.queue-item .item-type.image {
  background: #f6ffed;
  color: #52c41a;
}

.queue-item .item-type.file,
.queue-item .item-type.folder,
.queue-item .item-type.files {
  background: #f9f0ff;
  color: #722ed1;
}

.queue-item .item-preview {
  flex: 1;
  font-size: 13px;
  color: #262626;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  min-width: 0;
}

.queue-item .remove-btn {
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  cursor: pointer;
  color: #8c8c8c;
  border-radius: 3px;
  opacity: 0;
  transition: all 0.15s ease;
}

.queue-item:hover .remove-btn {
  opacity: 1;
}

.queue-item .remove-btn:hover {
  background: #ff4d4f;
  color: #fff;
}

.queue-item .remove-btn svg {
  width: 12px;
  height: 12px;
}

.queue-settings {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 0;
  border-top: 1px solid #e8e8e8;
}

.setting-label {
  font-size: 12px;
  color: #595959;
  flex-shrink: 0;
}

.separator-options {
  display: flex;
  gap: 4px;
}

.sep-btn {
  padding: 4px 12px;
  font-size: 12px;
  border: 1px solid #d9d9d9;
  background: #fff;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s ease;
  color: #595959;
}

.sep-btn:hover {
  border-color: #262626;
}

.sep-btn.active {
  background: #262626;
  border-color: #262626;
  color: #fff;
}

.custom-sep-input {
  flex: 1;
  padding: 4px 8px;
  font-size: 12px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  outline: none;
  max-width: 100px;
}

.custom-sep-input:focus {
  border-color: #262626;
}

.setting-checkbox {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: #595959;
  cursor: pointer;
}

.setting-checkbox input[type="checkbox"] {
  width: 14px;
  height: 14px;
  cursor: pointer;
}

.queue-preview {
  padding: 8px 0;
  border-top: 1px solid #e8e8e8;
}

.queue-preview .setting-label {
  display: block;
  margin-bottom: 6px;
}

.preview-content {
  padding: 8px 12px;
  font-size: 12px;
  color: #595959;
  background: #f5f5f5;
  border-radius: 4px;
  max-height: 60px;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  white-space: pre-wrap;
}

.paste-btn {
  width: 100%;
  padding: 10px;
  margin-top: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  font-size: 13px;
  font-weight: 500;
  color: #fff;
  background: #262626;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: background-color 0.15s ease;
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.paste-btn:hover {
  background: #404040;
}

.paste-btn svg {
  width: 14px;
  height: 14px;
}

/* 滚动条样式 */
.queue-content::-webkit-scrollbar {
  width: 4px;
}

.queue-content::-webkit-scrollbar-track {
  background: transparent;
}

.queue-content::-webkit-scrollbar-thumb {
  background: #d9d9d9;
  border-radius: 2px;
}

.queue-content::-webkit-scrollbar-thumb:hover {
  background: #bfbfbf;
}
</style>
