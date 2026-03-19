<template>
  <div class="text-preview-container">
    <div class="editor-toolbar">
      <div class="toolbar-tabs">
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

      <ExtractedInfoPanel
        v-if="hasExtractedInfo"
        :content="editedContent"
        :content-type="contentType"
        @copy="handleCopyExtracted"
        @paste="handlePasteExtracted"
      />
    </div>

    <textarea
      v-if="!isPreview"
      v-model="editedContent"
      class="text-editor"
      placeholder="输入内容..."
      @keydown.stop
    />
    <div v-else class="text-preview" v-html="previewContent" />

    <div class="stats-footer">
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
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { decodeHtmlEntities } from '@/utils/htmlUtils';
import ExtractedInfoPanel from './ExtractedInfoPanel.vue';

interface Props {
  content: string;
  contentType: string;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  'update:content': [value: string];
  copyExtracted: [value: string];
  pasteExtracted: [value: string];
}>();

const isPreview = ref(false);
const editedContent = computed({
  get: () => props.content,
  set: (value) => emit('update:content', value)
});

const hasExtractedInfo = computed(() => {
  // 简单检查是否有可提取的信息
  if (!editedContent.value) return false;
  const content = props.contentType === 'html'
    ? decodeHtmlEntities(editedContent.value).replace(/<[^>]+>/g, ' ')
    : editedContent.value;
  return /\b(?:\d{17}[\dXx]|1[3-9]\d{9}|\d{4}[年/-]\d{1,2}[月/-]\d{1,2}|[A-Za-z0-9._%+-]+@)/.test(content);
});

const previewContent = computed(() => {
  if (props.contentType === 'html') {
    return decodeHtmlEntities(editedContent.value);
  }
  return editedContent.value.replace(/\n/g, '<br>');
});

const charCount = computed(() => editedContent.value.length);

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

const handleCopyExtracted = (value: string) => {
  emit('copyExtracted', value);
};

const handlePasteExtracted = (value: string) => {
  emit('pasteExtracted', value);
};
</script>

<style scoped>
.text-preview-container {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.editor-toolbar {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 12px;
  padding-bottom: 12px;
  border-bottom: 1px solid #e8e8e8;
}

.toolbar-tabs {
  display: flex;
  gap: 4px;
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
  flex: 1;
  width: 100%;
  padding: 12px;
  font-size: 14px;
  line-height: 1.6;
  border: 1px solid #d9d9d9;
  border-radius: 6px;
  resize: none;
  outline: none;
  font-family: inherit;
  min-height: 200px;
}

.text-editor:focus {
  border-color: #262626;
}

.text-preview {
  flex: 1;
  padding: 12px;
  font-size: 14px;
  line-height: 1.6;
  border: 1px solid #e8e8e8;
  border-radius: 6px;
  background: #fafafa;
  min-height: 200px;
  overflow-wrap: break-word;
  overflow-y: auto;
}

.stats-footer {
  display: flex;
  gap: 20px;
  padding-top: 12px;
  margin-top: 12px;
  border-top: 1px solid #e8e8e8;
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
</style>
