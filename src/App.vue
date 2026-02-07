<script setup lang="ts">
import SettingsPanel from '@/components/SettingsPanel.vue';
</script>

<template>
  <div class="app">
    <SettingsPanel />
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
  background-color: #fff;
  display: flex;
  flex-direction: column;
}
</style>
