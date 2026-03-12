<template>
  <div class="setting-item" :class="{ 'full-width': fullWidth }">
    <div class="setting-info">
      <div class="setting-title">{{ title }}</div>
      <div v-if="description" class="setting-desc">{{ description }}</div>
      <div v-if="note" class="setting-note">{{ note }}</div>
      <div v-if="warning" class="setting-warning">{{ warning }}</div>
      <div v-if="error" class="error-message">{{ error }}</div>
    </div>
    <div class="setting-control">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  title: string;
  description?: string;
  note?: string;
  warning?: string;
  error?: string;
  fullWidth?: boolean;
}

defineProps<Props>();
</script>

<style scoped>
.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 0;
  border-bottom: 1px solid #f0f0f0;
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-item.full-width {
  flex-direction: column;
  align-items: flex-start;
  gap: 12px;
}

.setting-item.full-width .setting-control {
  width: 100%;
}

.setting-info {
  flex: 1;
}

.setting-title {
  font-size: 14px;
  font-weight: 500;
  color: #262626;
  margin-bottom: 4px;
}

.setting-desc {
  font-size: 12px;
  color: #8c8c8c;
}

.setting-note {
  font-size: 11px;
  color: #faad14;
  margin-top: 4px;
}

.setting-warning {
  font-size: 11px;
  color: #ff4d4f;
  margin-top: 4px;
}

.error-message {
  font-size: 11px;
  color: #ff4d4f;
  margin-top: 4px;
  padding: 4px 8px;
  background: #fff2f0;
  border: 1px solid #ffccc7;
  border-radius: 4px;
}

.setting-control {
  display: flex;
  align-items: center;
  gap: 12px;
}
</style>
