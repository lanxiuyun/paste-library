<script setup lang="ts">
import ClipboardList from '@/components/ClipboardList.vue';
</script>

<template>
  <div class="clipboard-window">
    <div class="drag-handle" data-tauri-drag-region>
      <span class="title">剪贴板历史</span>
    </div>
    <div class="clipboard-content">
      <ClipboardList />
    </div>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
  outline: none;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto,
    'Helvetica Neue', Arial, 'Noto Sans SC', sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

.clipboard-window {
  height: 100vh;
  background-color: #fff;
  display: flex;
  flex-direction: column;
}

.drag-handle {
  height: 36px;
  background-color: #f5f5f5;
  display: flex;
  align-items: center;
  padding: 0 16px;
  -webkit-app-region: drag;
  user-select: none;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 100;
}

.drag-handle .title {
  font-size: 14px;
  font-weight: 500;
  color: #333;
}

.clipboard-content {
  flex: 1;
  overflow: hidden;
  padding-top: 36px;
}
</style>
