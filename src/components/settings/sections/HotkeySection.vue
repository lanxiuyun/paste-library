<template>
  <div class="settings-section">
    <h2 class="section-title">快捷键</h2>

    <div class="shortcut-card">
      <div class="shortcut-row">
        <span class="shortcut-label">打开剪贴板窗口</span>
        <KeyRecorder
          v-model="form.hotkey"
          @record="handleHotkeyRecord"
        />
      </div>
      <div v-if="hotkeyError" class="shortcut-error">{{ hotkeyError }}</div>

      <div class="shortcut-row">
        <div>
          <span class="shortcut-label">数字键快捷粘贴</span>
          <p class="shortcut-desc">显示简体版时，使用修饰键 + 数字键快速粘贴前九项</p>
        </div>
        <KeyRecorder
          v-model="form.number_key_shortcut"
          is-modifier-only
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
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

let currentRegisteredHotkey = '';
let unlistenHotkeyError: UnlistenFn | null = null;

onMounted(async () => {
  currentRegisteredHotkey = props.form.hotkey;

  unlistenHotkeyError = await listen<string>('shortcut-registration-failed', (event) => {
    hotkeyError.value = `快捷键 "${event.payload}" 已被其他程序占用`;
  });
});

onUnmounted(() => {
  unlistenHotkeyError?.();
});

const handleHotkeyRecord = async (value: string) => {
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

.shortcut-card {
  background: #fff;
  border-radius: 10px;
  padding: 0 20px;
}

.shortcut-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 0;
  border-bottom: 1px solid #f0f0f0;
}

.shortcut-row:last-child {
  border-bottom: none;
}

.shortcut-label {
  font-size: 14px;
  font-weight: 500;
  color: #262626;
}

.shortcut-desc {
  font-size: 12px;
  color: #8c8c8c;
  margin: 4px 0 0;
}

.shortcut-error {
  font-size: 12px;
  color: #ff4d4f;
  padding: 0 0 12px;
  margin-top: -8px;
}
</style>
