<template>
  <div class="settings-section">
    <h2 class="section-title">快捷键设置</h2>
    
    <div class="setting-group">
      <SettingItem
        title="唤醒快捷键"
        description="按下此快捷键可快速打开或关闭剪贴板窗口"
        note="点击右边按钮开始录制，然后按下想要的快捷键组合"
        :warning="shortcutError ? '修改快捷键后需要重启应用才能生效' : undefined"
        :error="shortcutError"
      >
        <KeyRecorder
          v-model="form.hotkey"
          @record="handleHotkeyRecord"
        />
      </SettingItem>

      <SettingItem
        title="数字键快捷粘贴"
        description="按下 1-9 数字键粘贴对应位置剪贴板内容时需要同时按住的修饰键"
        note="点击右边按钮开始录制，然后按下想要的修饰键组合（如 Ctrl、Ctrl+Shift、Alt 等）"
      >
        <KeyRecorder
          v-model="form.number_key_shortcut"
          size="small"
          is-modifier-only
        />
      </SettingItem>

      <SettingItem
        title="切换钉住模式"
        description="快速切换剪贴板面板的钉住/默认模式"
        note="点击右边按钮开始录制，然后按下想要的快捷键组合"
        :warning="pinShortcutError ? '快捷键冲突，请更换其他组合' : undefined"
        :error="pinShortcutError"
      >
        <KeyRecorder
          v-model="form.pin_shortcut"
          @record="handlePinShortcutRecord"
        />
      </SettingItem>
    </div>
  </div>
</template>

<script setup lang="ts">
import SettingItem from '../components/SettingItem.vue';
import KeyRecorder from '../components/KeyRecorder.vue';
import type { AppSettings } from '@/types';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface Props {
  form: AppSettings;
  shortcutError: string;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  'hotkey-record': [value: string];
}>();

const pinShortcutError = ref('');

const handleHotkeyRecord = (value: string) => {
  emit('hotkey-record', value);
};

const handlePinShortcutRecord = async (value: string) => {
  // 检查快捷键冲突
  if (value === props.form.hotkey) {
    pinShortcutError.value = '与唤醒快捷键冲突';
    return;
  }
  if (value === props.form.number_key_shortcut) {
    pinShortcutError.value = '与数字键快捷粘贴冲突';
    return;
  }
  pinShortcutError.value = '';
  
  // 调用后端更新快捷键
  try {
    await invoke('update_pin_shortcut', { shortcut: value });
  } catch (error) {
    console.error('Failed to update pin shortcut:', error);
    pinShortcutError.value = '更新快捷键失败';
  }
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
</style>
