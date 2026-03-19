<template>
  <Teleport to="body">
    <Transition name="drawer">
      <div
        v-if="visible"
        class="drawer-overlay"
        @mousedown="handleOverlayMouseDown"
        @mouseup="handleOverlayMouseUp"
      >
        <div class="drawer-panel" @mousedown.stop @mouseup.stop>
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
                <svg
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                >
                  <rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
                  <path
                    d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"
                  />
                </svg>
              </button>
              <button class="action-btn" title="粘贴" @click="handlePaste">
                <svg
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                >
                  <path
                    d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"
                  />
                  <rect x="8" y="2" width="8" height="4" rx="1" />
                </svg>
              </button>
              <button
                class="action-btn"
                title="保存为新项"
                @click="handleSaveAsNew"
              >
                <svg
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                >
                  <path
                    d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"
                  />
                  <polyline points="17 21 17 13 7 13 7 21" />
                  <polyline points="7 3 7 8 15 8" />
                </svg>
              </button>
              <button class="action-btn close-btn" title="关闭" @click="close">
                <svg
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                >
                  <line x1="18" y1="6" x2="6" y2="18" />
                  <line x1="6" y1="6" x2="18" y2="18" />
                </svg>
              </button>
            </div>
          </div>

          <!-- Content -->
          <div class="drawer-content">
            <!-- Text Content -->
            <TextPreview
              v-if="isTextContent"
              v-model:content="editedContent"
              :content-type="item?.content_type || 'text'"
              @copy-extracted="copyExtractedInfo"
              @paste-extracted="pasteExtractedInfo"
            />

            <!-- Image Preview -->
            <ImagePreview
              v-else-if="item?.content_type === 'image'"
              :item="item"
            />

            <!-- File/Folder/Multi-files Preview -->
            <FilePreview v-else-if="item" :item="item" />
          </div>

          <!-- Footer (only for text content) -->
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
import type { ClipboardItem } from "@/types";
import { writeText } from "tauri-plugin-clipboard-x-api";
import { computed, ref, watch } from "vue";
import FilePreview from "./drawer/FilePreview.vue";
import ImagePreview from "./drawer/ImagePreview.vue";
import TextPreview from "./drawer/TextPreview.vue";

interface Props {
  visible: boolean;
  item: ClipboardItem | null;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  "update:visible": [value: boolean];
  copy: [item: ClipboardItem];
  paste: [item: ClipboardItem];
  saveAsNew: [content: string, type: string];
}>();

const editedContent = ref("");
const isMouseDownOnOverlay = ref(false);

const isTextContent = computed(() => {
  return (
    props.item?.content_type === "text" ||
    props.item?.content_type === "html" ||
    props.item?.content_type === "rtf"
  );
});

const typeLabel = computed(() => {
  if (!props.item) return "";
  const labels: Record<string, string> = {
    text: "文本",
    html: "HTML",
    rtf: "RTF",
    image: "图片",
    file: "文件",
    folder: "文件夹",
    files: "多文件",
  };
  return labels[props.item.content_type] || "文本";
});

const formattedTime = computed(() => {
  if (!props.item) return "";
  return new Date(props.item.created_at).toLocaleString("zh-CN");
});

const charCount = computed(() => editedContent.value.length);

const wordCount = computed(() => {
  return editedContent.value
    .trim()
    .split(/\s+/)
    .filter((w) => w.length > 0).length;
});

const lineCount = computed(() => editedContent.value.split("\n").length);

const byteSize = computed(() => {
  const bytes = new Blob([editedContent.value]).size;
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
});

watch(
  () => props.item,
  (newItem) => {
    if (newItem) {
      editedContent.value = newItem.content;
    }
  },
  { immediate: true }
);

const close = () => {
  emit("update:visible", false);
};

const handleOverlayMouseDown = (e: MouseEvent) => {
  if (e.target === e.currentTarget) {
    isMouseDownOnOverlay.value = true;
  }
};

const handleOverlayMouseUp = (e: MouseEvent) => {
  if (isMouseDownOnOverlay.value && e.target === e.currentTarget) {
    close();
  }
  isMouseDownOnOverlay.value = false;
};

const handleCopy = () => {
  if (props.item) {
    emit("copy", { ...props.item, content: editedContent.value });
  }
};

const handlePaste = () => {
  if (props.item) {
    emit("paste", { ...props.item, content: editedContent.value });
  }
};

const handleSaveAsNew = () => {
  if (props.item) {
    emit("saveAsNew", editedContent.value, props.item.content_type);
  }
};

const copyExtractedInfo = async (value: string): Promise<void> => {
  try {
    await writeText(value);
  } catch (error) {
    console.error("Failed to copy extracted info:", error);
  }
};

const pasteExtractedInfo = async (value: string): Promise<void> => {
  try {
    await writeText(value);
    close();
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("simulate_paste");
  } catch (error) {
    console.error("Failed to paste extracted info:", error);
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
  flex-shrink: 0;
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
.type-badge.folder,
.type-badge.files {
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
  display: flex;
  flex-direction: column;
}

.drawer-footer {
  padding: 12px 16px;
  border-top: 1px solid #e8e8e8;
  background: #fafafa;
  flex-shrink: 0;
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
