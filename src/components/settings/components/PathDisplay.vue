<template>
  <div class="path-display">
    <span class="path-text">{{ path }}</span>
    <button class="icon-btn" :title="copyTitle" @click="handleCopy">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
        <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
      </svg>
    </button>
    <button class="icon-btn" :title="openTitle" @click="handleOpen">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
      </svg>
    </button>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

interface Props {
  path: string;
  copyTitle?: string;
  openTitle?: string;
}

const props = withDefaults(defineProps<Props>(), {
  copyTitle: '复制路径',
  openTitle: '打开文件夹',
});

const emit = defineEmits<{
  'copy': [path: string];
  'open': [path: string];
}>();

const handleCopy = async () => {
  try {
    await navigator.clipboard.writeText(props.path);
    emit('copy', props.path);
  } catch (error) {
    console.error('复制失败:', error);
    alert('复制失败，请重试');
  }
};

const handleOpen = async () => {
  try {
    await invoke('show_in_folder', { path: props.path });
    emit('open', props.path);
  } catch (error) {
    console.error('打开文件夹失败:', error);
    alert('打开文件夹失败，请重试');
  }
};
</script>

<style scoped>
.path-display {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 10px 12px;
  background: #fafafa;
  border: 1px solid #e8e8e8;
  border-radius: 4px;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
  font-size: 12px;
}

.path-text {
  flex: 1;
  color: #595959;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.icon-btn {
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #8c8c8c;
  transition: all 0.2s;
  flex-shrink: 0;
}

.icon-btn:hover {
  background: #f5f5f5;
  color: #262626;
}

.icon-btn svg {
  width: 16px;
  height: 16px;
}
</style>
