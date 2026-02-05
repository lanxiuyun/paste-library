<template>
  <div class="drag-bar" data-tauri-drag-region>
    <div class="drag-capsule" data-tauri-drag-region>
      <div class="drag-line"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
.drag-bar {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  background: #fff;
  border-bottom: 1px solid #f0f0f0;
  -webkit-app-region: drag;
  app-region: drag;
}

.drag-capsule {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 4px 0;
}

.drag-line {
  width: 36px;
  height: 3px;
  background: #d0d0d0;
  border-radius: 2px;
  transition: background 0.2s;
}

.drag-capsule:hover .drag-line {
  background: #b0b0b0;
}
</style>
