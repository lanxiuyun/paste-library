<script setup lang="ts">
import ClipboardList from '@/components/ClipboardList.vue';
import DragHandle from '@/components/DragHandle.vue';
</script>

<template>
  <div class="app">
    <DragHandle />
    <div class="app-content">
      <ClipboardList />
    </div>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto,
    'Helvetica Neue', Arial, 'Noto Sans SC', sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

.app {
  height: 100vh;
  background-color: #f5f5f5;
  display: flex;
  flex-direction: column;
}

.app-content {
  flex: 1;
  overflow: hidden;
  padding-top: 40px;
}
</style>