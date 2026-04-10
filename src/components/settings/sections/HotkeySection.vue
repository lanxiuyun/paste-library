<template>
  <div class="settings-section">
    <h2 class="section-title">快捷键设置</h2>
    
    <div class="setting-group">
      <SettingItem
        title="唤醒快捷键"
        description="按下此快捷键可快速打开或关闭剪贴板窗口"
        note="点击右边按钮开始录制，然后按下想要的快捷键组合"
        :error="hotkeyError"
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
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

interface Props {
  form: AppSettings;
}

const props = defineProps<Props>();

const hotkeyError = ref('');
const pinShortcutError = ref('');

let currentRegisteredHotkey = '';
let currentRegisteredPinShortcut = '';
let unlistenHotkeyError: UnlistenFn | null = null;
let unlistenPinError: UnlistenFn | null = null;

onMounted(async () => {
  currentRegisteredHotkey = props.form.hotkey;
  currentRegisteredPinShortcut = props.form.pin_shortcut;

  unlistenHotkeyError = await listen<string>('shortcut-registration-failed', (event) => {
    hotkeyError.value = `快捷键 "${event.payload}" 已被其他程序占用`;
  });
  unlistenPinError = await listen<string>('pin-shortcut-registration-failed', (event) => {
    pinShortcutError.value = `快捷键 "${event.payload}" 已被其他程序占用`;
  });
});

onUnmounted(() => {
  unlistenHotkeyError?.();
  unlistenPinError?.();
});

const handleHotkeyRecord = async (value: string) => {
  if (value === props.form.pin_shortcut) {
    hotkeyError.value = '与钉住模式快捷键冲突';
    props.form.hotkey = currentRegisteredHotkey;
    return;
  }

  try {
    await invoke('update_hotkey', {
      oldHotkey: currentRegisteredHotkey,
      newHotkey: value,
    });
    currentRegisteredHotkey = value;
    hotkeyError.value = '';
  } catch (error) {
    hotkeyError.value = String(error);
    props.form.hotkey = currentRegisteredHotkey;
  }
};

const handlePinShortcutRecord = async (value: string) => {
  if (value === props.form.hotkey) {
    pinShortcutError.value = '与唤醒快捷键冲突';
    props.form.pin_shortcut = currentRegisteredPinShortcut;
    return;
  }

  try {
    await invoke('update_pin_shortcut', {
      oldShortcut: currentRegisteredPinShortcut,
      newShortcut: value,
    });
    currentRegisteredPinShortcut = value;
    pinShortcutError.value = '';
  } catch (error) {
    pinShortcutError.value = String(error);
    props.form.pin_shortcut = currentRegisteredPinShortcut;
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
