<script setup lang="ts">
// Placeholder for Settings Panel
// Will be implemented in Phase 4
</script>

<template>
  <div class="settings-panel">
    <div class="settings-header">
      <h1>设置</h1>
    </div>
    <div class="settings-content">
      <p>设置面板正在开发中...</p>
      <p class="hint">完整的19项设置将在Phase 4实现</p>
    </div>
  </div>
</template>

<style scoped>
.settings-panel {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: #fff;
}

.settings-header {
  padding: 16px 24px;
  border-bottom: 1px solid #f0f0f0;
}

.settings-header h1 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
}

.settings-content {
  flex: 1;
  padding: 24px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #666;
}

.settings-content p {
  margin: 0;
}

.settings-content .hint {
  font-size: 12px;
  color: #999;
  margin-top: 8px;
}
</style>
