<template>
  <div class="settings-section">
    <h2 class="section-title">历史记录设置</h2>
    
    <div class="setting-group">
      <SettingItem
        title="最大历史记录数"
        description="达到上限后自动删除最早的数据"
      >
        <input 
          type="number" 
          v-model.number="form.max_history_count"
          min="100"
          max="10000"
          step="100"
          class="number-input"
        />
      </SettingItem>

      <SettingItem title="自动清理" description="自动删除超过指定天数的历史记录">
        <select v-model.number="form.auto_cleanup_days" class="select-input">
          <option :value="0">不自动清理</option>
          <option :value="7">7天</option>
          <option :value="30">30天</option>
          <option :value="90">90天</option>
        </select>
      </SettingItem>

      <SettingItem
        title="删除历史记录"
        description="永久删除所有剪贴板历史记录（此操作不可撤销）"
      >
        <button class="btn-danger" @click="handleClearAll">
          删除全部
        </button>
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

const emit = defineEmits<{
  'clear-all': [];
}>();

const handleClearAll = () => {
  emit('clear-all');
};
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

.number-input {
  width: 80px;
  padding: 6px 10px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-size: 13px;
  text-align: center;
  outline: none;
}

.number-input:focus {
  border-color: #262626;
}

.select-input {
  padding: 6px 28px 6px 10px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-size: 13px;
  background: #fff;
  cursor: pointer;
  outline: none;
  min-width: 120px;
}

.select-input:focus {
  border-color: #262626;
}

.btn-danger {
  padding: 8px 16px;
  background: #ff4d4f;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  color: #fff;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-danger:hover {
  background: #ff7875;
}

.btn-danger:active {
  background: #d9363e;
}
</style>
