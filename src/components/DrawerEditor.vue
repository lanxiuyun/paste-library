<template>
  <Teleport to="body">
    <Transition name="drawer">
      <div v-if="visible" class="drawer-overlay" @click="handleOverlayClick">
        <div class="drawer-panel" @click.stop>
          <!-- Header -->
          <div class="drawer-header">
            <div class="header-info">
              <span class="type-badge" :class="item?.content_type">
                {{ typeLabel }}
              </span>
              <span class="created-time">{{ formattedTime }}</span>
            </div>
            <div class="header-actions">
              <button class="action-btn" title="复制" @click="handleCopy">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                  <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
                </svg>
              </button>
              <button class="action-btn" title="粘贴" @click="handlePaste">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/>
                  <rect x="8" y="2" width="8" height="4" rx="1"/>
                </svg>
              </button>
              <button class="action-btn" title="保存为新项" @click="handleSaveAsNew">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
                  <polyline points="17 21 17 13 7 13 7 21"/>
                  <polyline points="7 3 7 8 15 8"/>
                </svg>
              </button>
              <button class="action-btn close-btn" title="关闭" @click="close">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <line x1="18" y1="6" x2="6" y2="18"/>
                  <line x1="6" y1="6" x2="18" y2="18"/>
                </svg>
              </button>
            </div>
          </div>

          <!-- Content -->
          <div class="drawer-content">
            <!-- Text Editor -->
            <template v-if="isTextContent">
              <div class="editor-toolbar">
                <button 
                  class="toolbar-btn" 
                  :class="{ active: !isPreview }"
                  @click="isPreview = false"
                >
                  编辑
                </button>
                <button 
                  class="toolbar-btn" 
                  :class="{ active: isPreview }"
                  @click="isPreview = true"
                >
                  预览
                </button>
              </div>
              
              <textarea
                v-if="!isPreview"
                v-model="editedContent"
                class="text-editor"
                placeholder="输入内容..."
                @keydown.stop
              />
              <div v-else class="text-preview" v-html="previewContent" />
            </template>

            <!-- Image Preview -->
            <template v-else-if="item?.content_type === 'image'">
              <div class="image-viewer">
                <img 
                  v-if="item.thumbnail_path"
                  :src="drawerImageSrc"
                  :alt="'图片 ' + (item.metadata?.width || 0) + 'x' + (item.metadata?.height || 0)"
                  class="preview-image"
                />
                <div class="image-info">
                  <div class="info-item">
                    <span class="label">尺寸:</span>
                    <span class="value">{{ item.metadata?.width || '?' }} × {{ item.metadata?.height || '?' }}</span>
                  </div>
                  <div class="info-item">
                    <span class="label">格式:</span>
                    <span class="value">{{ item.metadata?.format || 'PNG' }}</span>
                  </div>
                  <div class="info-item">
                    <span class="label">大小:</span>
                    <span class="value">{{ imageFileSize }}</span>
                  </div>
                </div>
              </div>
            </template>

            <!-- File Info -->
            <template v-else-if="item?.content_type === 'file' || item?.content_type === 'folder'">
              <div class="file-viewer">
                <div class="file-icon-large">
                  {{ item.content_type === 'folder' ? '📁' : '📄' }}
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
                    <button class="path-action-btn" @click="copyFilePath">
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

            <!-- Multiple Files Info -->
            <template v-else-if="item?.content_type === 'files'">
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
                  <button class="path-action-btn" @click="copyFilePath">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/>
                      <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>
                    </svg>
                    复制所有路径
                  </button>
                </div>
              </div>
            </template>
          </div>

          <!-- Stats Footer (仅文本类型显示统计) -->
          <div v-if="isTextContent" class="drawer-footer">
            <div class="stats">
              <span class="stat-item">
                <span class="stat-label">字符:</span>
                <span class="stat-value">{{ charCount }}</span>
              </span>
              <span class="stat-item">
                <span class="stat-label">单词:</span>
                <span class="stat-value">{{ wordCount }}</span>
              </span>
              <span class="stat-item">
                <span class="stat-label">行数:</span>
                <span class="stat-value">{{ lineCount }}</span>
              </span>
              <span class="stat-item">
                <span class="stat-label">大小:</span>
                <span class="stat-value">{{ byteSize }}</span>
              </span>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { convertFileSrc, invoke } from '@tauri-apps/api/core';
import { writeText } from 'tauri-plugin-clipboard-x-api';
import type { ClipboardItem } from '@/types';
import { decodeHtmlEntities } from '@/utils/htmlUtils';

interface Props {
  visible: boolean;
  item: ClipboardItem | null;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  'update:visible': [value: boolean];
  copy: [item: ClipboardItem];
  paste: [item: ClipboardItem];
  saveAsNew: [content: string, type: string];
}>();

const editedContent = ref('');
const isPreview = ref(false);
const actualFileSize = ref<number>(0);

const isTextContent = computed(() => {
  return props.item?.content_type === 'text' || props.item?.content_type === 'html' || props.item?.content_type === 'rtf';
});

const typeLabel = computed(() => {
  if (!props.item) return '';
  switch (props.item.content_type) {
    case 'text': return '文本';
    case 'html': return 'HTML';
    case 'rtf': return 'RTF';
    case 'image': return '图片';
    case 'file': return '文件';
    case 'folder': return '文件夹';
    case 'files': return '多文件';
    default: return '文本';
  }
});

const formattedTime = computed(() => {
  if (!props.item) return '';
  const date = new Date(props.item.created_at);
  return date.toLocaleString('zh-CN');
});

const previewContent = computed(() => {
  if (!props.item) return '';
  if (props.item.content_type === 'html') {
    // 解码 HTML 实体后显示，避免显示 &#160; 等实体编码
    return decodeHtmlEntities(editedContent.value);
  }
  return editedContent.value.replace(/\n/g, '<br>');
});

const charCount = computed(() => {
  return editedContent.value.length;
});

const wordCount = computed(() => {
  return editedContent.value.trim().split(/\s+/).filter(w => w.length > 0).length;
});

const lineCount = computed(() => {
  return editedContent.value.split('\n').length;
});

const byteSize = computed(() => {
  const bytes = new Blob([editedContent.value]).size;
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
});

// 处理图片路径 - 使用 convertFileSrc 转换
const drawerImageSrc = computed(() => {
  if (!props.item?.thumbnail_path) return '';
  return convertFileSrc(props.item.thumbnail_path);
});

// 图片文件大小（通过API获取）
const imageFileSize = computed(() => {
  const bytes = actualFileSize.value;
  if (bytes === 0) return '未知';
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
});

// 异步获取文件大小（通过Rust后端）
const loadFileSize = async (filePath: string | null | undefined) => {
  if (!filePath) {
    actualFileSize.value = 0;
    return;
  }
  try {
    const size = await invoke<number>('get_file_size', { path: filePath });
    actualFileSize.value = size || 0;
  } catch (error) {
    console.error('Failed to get file size:', error);
    actualFileSize.value = 0;
  }
};

watch(() => props.item, async (newItem) => {
  if (newItem) {
    editedContent.value = newItem.content;
    isPreview.value = false;
    // 异步获取文件大小（图片/文件类型）
    if (newItem.content_type === 'image') {
      await loadFileSize(newItem.thumbnail_path);
    } else if (newItem.content_type === 'file' || newItem.content_type === 'folder') {
      await loadFileSize(newItem.file_paths?.[0]);
    } else {
      actualFileSize.value = 0;
    }
  }
}, { immediate: true });

const close = () => {
  emit('update:visible', false);
};

const handleOverlayClick = () => {
  close();
};

const handleCopy = () => {
  if (props.item) {
    emit('copy', { ...props.item, content: editedContent.value });
  }
};

const handlePaste = () => {
  if (props.item) {
    emit('paste', { ...props.item, content: editedContent.value });
  }
};

const handleSaveAsNew = () => {
  if (props.item) {
    emit('saveAsNew', editedContent.value, props.item.content_type);
  }
};

const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '-';
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
};

// 判断是否为图片文件
const isImageFile = (path: string): boolean => {
  const ext = path.split('.').pop()?.toLowerCase();
  return ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'svg', 'ico'].includes(ext || '');
};

// 复制文件路径到剪贴板
const copyFilePath = async (): Promise<void> => {
  if (!props.item) return;

  try {
    let pathToCopy = '';

    if (props.item.file_paths && props.item.file_paths.length > 0) {
      // 多文件时复制所有路径，用换行符分隔（Windows风格 \r\n）
      pathToCopy = props.item.file_paths.join('\r\n') + '\r\n';
    } else if (props.item.content) {
      // 使用 content 字段（文件夹类型），末尾添加换行符
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
.drawer-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.3);
  z-index: 9999;
  display: flex;
  justify-content: flex-end;
}

.drawer-panel {
  width: 500px;
  max-width: 90vw;
  height: 100%;
  background: #fff;
  box-shadow: -4px 0 20px rgba(0, 0, 0, 0.15);
  display: flex;
  flex-direction: column;
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.drawer-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid #e8e8e8;
}

.header-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.type-badge {
  padding: 3px 10px;
  font-size: 12px;
  font-weight: 500;
  border-radius: 4px;
}

.type-badge.text {
  background: #fff2e8;
  color: #fa8c16;
}

.type-badge.html {
  background: #e6f7ff;
  color: #1890ff;
}

.type-badge.image {
  background: #f6ffed;
  color: #52c41a;
}

.type-badge.file,
.type-badge.folder {
  background: #f9f0ff;
  color: #722ed1;
}

.created-time {
  font-size: 12px;
  color: #8c8c8c;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}

.action-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 4px;
  color: #595959;
  transition: all 0.15s ease;
}

.action-btn:hover {
  background: #f5f5f5;
  color: #262626;
}

.action-btn.close-btn:hover {
  background: #ff4d4f;
  color: #fff;
}

.action-btn svg {
  width: 16px;
  height: 16px;
}

.drawer-content {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
}

.editor-toolbar {
  display: flex;
  gap: 4px;
  margin-bottom: 12px;
  padding-bottom: 12px;
  border-bottom: 1px solid #e8e8e8;
}

.toolbar-btn {
  padding: 6px 16px;
  font-size: 13px;
  border: 1px solid #d9d9d9;
  background: #fff;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s ease;
  color: #595959;
}

.toolbar-btn:hover {
  border-color: #262626;
  color: #262626;
}

.toolbar-btn.active {
  background: #262626;
  border-color: #262626;
  color: #fff;
}

.text-editor {
  width: 100%;
  height: calc(100% - 60px);
  padding: 12px;
  font-size: 14px;
  line-height: 1.6;
  border: 1px solid #d9d9d9;
  border-radius: 6px;
  resize: none;
  outline: none;
  font-family: inherit;
}

.text-editor:focus {
  border-color: #262626;
}

.text-preview {
  padding: 12px;
  font-size: 14px;
  line-height: 1.6;
  border: 1px solid #e8e8e8;
  border-radius: 6px;
  background: #fafafa;
  min-height: 200px;
  overflow-wrap: break-word;
}

.image-viewer {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

.preview-image {
  max-width: 100%;
  max-height: 400px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.image-info {
  display: flex;
  gap: 24px;
  padding: 12px 16px;
  background: #f5f5f5;
  border-radius: 6px;
}

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

/* 多文件视图 */
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

.drawer-footer {
  padding: 12px 16px;
  border-top: 1px solid #e8e8e8;
  background: #fafafa;
}

.stats {
  display: flex;
  gap: 20px;
}

.stat-item {
  display: flex;
  gap: 4px;
  font-size: 12px;
}

.stat-label {
  color: #8c8c8c;
}

.stat-value {
  color: #262626;
  font-weight: 500;
}

/* Transition animations */
.drawer-enter-active,
.drawer-leave-active {
  transition: all 0.3s ease;
}

.drawer-enter-from,
.drawer-leave-to {
  opacity: 0;
}

.drawer-enter-from .drawer-panel,
.drawer-leave-to .drawer-panel {
  transform: translateX(100%);
}

.drawer-enter-active .drawer-panel,
.drawer-leave-active .drawer-panel {
  transition: transform 0.3s ease;
}

/* Scrollbar */
.drawer-content::-webkit-scrollbar {
  width: 6px;
}

.drawer-content::-webkit-scrollbar-track {
  background: transparent;
}

.drawer-content::-webkit-scrollbar-thumb {
  background: #c0c0c0;
  border-radius: 3px;
}

.drawer-content::-webkit-scrollbar-thumb:hover {
  background: #a0a0a0;
}
</style>
