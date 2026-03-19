<template>
  <div class="image-viewer">
    <img
      v-if="item.thumbnail_path"
      :src="drawerImageSrc"
      :alt="`图片 ${item.metadata?.width || 0}x${item.metadata?.height || 0}`"
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

<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { convertFileSrc, invoke } from '@tauri-apps/api/core';
import type { ClipboardItem } from '@/types';

interface Props {
  item: ClipboardItem;
}

const props = defineProps<Props>();

const actualFileSize = ref<number>(0);

const drawerImageSrc = computed(() => {
  if (!props.item.thumbnail_path) return '';
  return convertFileSrc(props.item.thumbnail_path);
});

const imageFileSize = computed(() => {
  const bytes = actualFileSize.value;
  if (bytes === 0) return '未知';
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
});

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

watch(() => props.item.thumbnail_path, (newPath) => {
  loadFileSize(newPath);
}, { immediate: true });
</script>

<style scoped>
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
}
</style>
