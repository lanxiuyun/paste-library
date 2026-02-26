<template>
  <div class="settings-section">
    <h2 class="section-title">通用设置</h2>
    
    <div class="setting-group">
      <SettingItem
        title="开机自启"
        description="系统启动时自动运行 Paste Library"
      >
        <label class="switch">
          <input type="checkbox" v-model="form.auto_start" />
          <span class="slider"></span>
        </label>
      </SettingItem>
    </div>
  </div>
</template>

<script setup lang="ts">
import SettingItem from '../components/SettingItem.vue';
import type { AppSettings } from '@/types';

interface Props {
  form: AppSettings;
}

defineProps<Props>();
</script>

<style scoped>
.settings-section {
  flex: 1;
  padding: 24px 32px;
  overflow-y: auto;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: #262626;
  margin: 0 0 12px 0;
}

.setting-group {
  background: #fff;
  border-radius: 8px;
  padding: 0 20px;
}

/* Switch toggle */
.switch {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 24px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #d9d9d9;
  transition: 0.2s;
  border-radius: 24px;
}

.slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: 0.2s;
  border-radius: 50%;
}

input:checked + .slider {
  background-color: #262626;
}

input:checked + .slider:before {
  transform: translateX(20px);
}
</style>
