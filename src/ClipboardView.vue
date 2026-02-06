<script setup lang="ts">
import ClipboardList from '@/components/ClipboardList.vue';
import DragHandle from '@/components/DragHandle.vue';
</script>

<template>
  <div class="clipboard-window">
    <DragHandle />
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

.clipboard-content {
  flex: 1;
  overflow: hidden;
  padding-top: 36px;
}
</style>
