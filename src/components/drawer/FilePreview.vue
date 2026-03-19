<template>
  <!-- Single File / Folder -->
  <template v-if="contentType === 'file' || contentType === 'folder'">
    <div class="file-viewer">
      <div class="file-icon-large">
        {{ contentType === 'folder' ? '📁' : '📄' }}
      </div>
      <div class="file-details">
        <div class="info-item">
          <span class="label">名称:</span>
          <span class="value">{{ item.metadata?.file_name || item.metadata?.folder_name || '-' }}</span>
        </div>
        <div class="info-item">
          <span class="label">大小:</span>
          <span class="value">{{ formatFileSize(item.metadata?.file_size || 0) }}</span>
        </div>
        <div class="info-item">
          <span class="label">路径:</span>
          <span class="value path">{{ item.content }}</span>
        </div>
        <div class="info-item actions-row">
          <button class="path-action-btn" @click="copyPath">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/>
              <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>
            </svg>
            复制路径
          </button>
        </div>
      </div>
    </div>
  </template>

  <!-- Multiple Files -->
  <template v-else-if="contentType === 'files'">
    <div class="files-viewer">
      <div class="files-header">
        <span class="files-icon-large">📚</span>
        <span class="files-count">{{ item.file_paths?.length || 0 }} 个文件</span>
      </div>
      <div class="files-list">
        <div
          v-for="(path, idx) in item.file_paths"
          :key="idx"
          class="file-list-item"
        >
          <span class="file-list-icon">{{ isImageFile(path) ? '🖼️' : '📄' }}</span>
          <span class="file-list-path">{{ path }}</span>
        </div>
      </div>
      <div class="files-actions">
        <button class="path-action-btn" @click="copyPath">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/>
            <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>
          </svg>
          复制所有路径
        </button>
      </div>
    </div>
  </template>
</template>

<script setup lang="ts">
import { writeText } from 'tauri-plugin-clipboard-x-api';
import type { ClipboardItem } from '@/types';

interface Props {
  item: ClipboardItem;
}

const props = defineProps<Props>();

const contentType = props.item.content_type;

const isImageFile = (path: string): boolean => {
  const ext = path.split('.').pop()?.toLowerCase();
  return ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'svg', 'ico'].includes(ext || '');
};

const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '-';
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
};

const copyPath = async (): Promise<void> => {
  try {
    let pathToCopy = '';

    if (props.item.file_paths && props.item.file_paths.length > 0) {
      pathToCopy = props.item.file_paths.join('\r\n') + '\r\n';
    } else if (props.item.content) {
      pathToCopy = props.item.content + '\r\n';
    }

    if (pathToCopy) {
      await writeText(pathToCopy);
    }
  } catch (error) {
    console.error('Failed to copy file path:', error);
  }
};
</script>

<style scoped>
.file-viewer {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
  padding: 20px;
}

.file-icon-large {
  font-size: 64px;
}

.file-details {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.info-item {
  display: flex;
  gap: 8px;
  font-size: 13px;
}

.info-item .label {
  color: #8c8c8c;
  flex-shrink: 0;
}

.info-item .value {
  color: #262626;
  word-break: break-all;
}

.info-item .value.path {
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
  font-size: 12px;
}

.info-item.actions-row {
  margin-top: 8px;
}

.path-action-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  font-size: 12px;
  color: #595959;
  background: #f5f5f5;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.path-action-btn:hover {
  color: #262626;
  background: #e8e8e8;
  border-color: #bfbfbf;
}

.path-action-btn svg {
  width: 14px;
  height: 14px;
}

.files-viewer {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.files-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px;
  background: #f5f5f5;
  border-radius: 8px;
}

.files-icon-large {
  font-size: 48px;
}

.files-count {
  font-size: 16px;
  font-weight: 500;
  color: #262626;
}

.files-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 300px;
  overflow-y: auto;
  padding: 12px;
  background: #fafafa;
  border-radius: 8px;
  border: 1px solid #e8e8e8;
}

.file-list-item {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  padding: 6px 8px;
  border-radius: 4px;
  transition: background-color 0.15s;
}

.file-list-item:hover {
  background: #f0f0f0;
}

.file-list-icon {
  font-size: 16px;
  flex-shrink: 0;
  margin-top: 2px;
}

.file-list-path {
  font-size: 12px;
  color: #262626;
  line-height: 1.4;
  word-break: break-all;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
}

.files-actions {
  display: flex;
  justify-content: flex-start;
}
</style>
