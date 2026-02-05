<template>
  <div class="drag-bar" data-tauri-drag-region>
    <div class="drag-capsule" data-tauri-drag-region>
      <div class="drag-dots">
        <span></span>
        <span></span>
        <span></span>
      </div>
    </div>
    <div class="window-controls">
      <button class="win-btn minimize" @click="minimizeWindow" title="最小化">
        <svg viewBox="0 0 24 24" fill="currentColor">
          <rect x="4" y="11" width="16" height="2"/>
        </svg>
      </button>
      <button class="win-btn close" @click="closeWindow" title="关闭">
        <svg viewBox="0 0 24 24" fill="currentColor">
          <path d="M18.3 5.7a1 1 0 00-1.4 0L12 10.6 7.1 5.7a1 1 0 00-1.4 1.4L10.6 12l-4.9 4.9a1 1 0 001.4 1.4L12 13.4l4.9 4.9a1 1 0 001.4-1.4L13.4 12l4.9-4.9a1 1 0 000-1.4z"/>
        </svg>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

const appWindow = getCurrentWebviewWindow();

const minimizeWindow = async () => {
  await appWindow.minimize();
};

const closeWindow = async () => {
  await appWindow.hide();
};
</script>

<style scoped>
.drag-bar {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  height: 44px;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  background: linear-gradient(to bottom, rgba(255,255,255,0.95), rgba(255,255,255,0.8));
  border-bottom: 1px solid rgba(0,0,0,0.05);
  -webkit-app-region: drag;
  app-region: drag;
}

.drag-capsule {
  background: rgba(0, 0, 0, 0.06);
  border-radius: 20px;
  padding: 6px 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: grab;
  transition: all 0.2s ease;
}

.drag-capsule:hover {
  background: rgba(0, 0, 0, 0.1);
}

.drag-capsule:active {
  cursor: grabbing;
}

.drag-dots {
  display: flex;
  gap: 5px;
  align-items: center;
}

.drag-dots span {
  width: 5px;
  height: 5px;
  background: #bFBFBF;
  border-radius: 50%;
  transition: all 0.2s;
}

.drag-capsule:hover .drag-dots span {
  background: #999;
}

.window-controls {
  position: absolute;
  right: 12px;
  display: flex;
  gap: 8px;
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.win-btn {
  width: 28px;
  height: 28px;
  border: none;
  border-radius: 6px;
  background: transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #666;
  transition: all 0.2s;
}

.win-btn:hover {
  background: rgba(0,0,0,0.08);
}

.win-btn.minimize:hover {
  color: #1890ff;
}

.win-btn.close:hover {
  background: #ff4d4f;
  color: #fff;
}

.win-btn svg {
  width: 14px;
  height: 14px;
}
</style>