<template>
  <div v-if="visible" class="confirm-dialog-overlay" @click="handleCancel">
    <div class="confirm-dialog" @click.stop>
      <div class="confirm-dialog-header">
        <svg class="warning-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
          <line x1="12" y1="9" x2="12" y2="13"/>
          <line x1="12" y1="17" x2="12.01" y2="17"/>
        </svg>
        <h3>确认删除</h3>
      </div>
      <div class="confirm-dialog-content">
        <p>确定要删除这条剪贴板记录吗？</p>
        <p class="confirm-dialog-subtitle">此操作不可撤销</p>
      </div>
      <div class="confirm-dialog-actions">
        <button class="confirm-btn cancel" @click="handleCancel">取消</button>
        <button class="confirm-btn confirm" @click="handleConfirm">删除</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  visible: boolean;
}

defineProps<Props>();

const emit = defineEmits<{
  'confirm': [];
  'cancel': [];
}>();

const handleConfirm = () => {
  emit('confirm');
};

const handleCancel = () => {
  emit('cancel');
};
</script>

<style scoped>
.confirm-dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.confirm-dialog {
  background: #fff;
  border-radius: 12px;
  width: 360px;
  max-width: 90%;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  overflow: hidden;
}

.confirm-dialog-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 20px 20px 12px;
}

.warning-icon {
  width: 32px;
  height: 32px;
  color: #faad14;
  flex-shrink: 0;
}

.confirm-dialog-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #262626;
}

.confirm-dialog-content {
  padding: 0 20px 20px;
  padding-left: 64px;
}

.confirm-dialog-content p {
  margin: 0 0 4px;
  font-size: 14px;
  color: #595959;
  line-height: 1.5;
}

.confirm-dialog-subtitle {
  font-size: 12px !important;
  color: #8c8c8c !important;
}

.confirm-dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 12px 20px 20px;
  padding-left: 64px;
}

.confirm-btn {
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
}

.confirm-btn.cancel {
  background: #f5f5f5;
  color: #595959;
}

.confirm-btn.cancel:hover {
  background: #e8e8e8;
  color: #262626;
}

.confirm-btn.confirm {
  background: #ff4d4f;
  color: #fff;
}

.confirm-btn.confirm:hover {
  background: #ff7875;
}
</style>
