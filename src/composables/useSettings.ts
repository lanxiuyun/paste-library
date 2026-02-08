import { invoke } from '@tauri-apps/api/core';
import { emit, listen } from '@tauri-apps/api/event';
import { ref, onMounted } from 'vue';
import type { AppSettings } from '@/types';

const settings = ref<AppSettings>({
  // 历史记录设置
  max_history_count: 5000,
  auto_cleanup_days: 30,

  // 窗口设置
  window_position: 'remember',

  // 智能激活设置 (新增)
  smart_activate: true,

  // 音效设置
  copy_sound: false,

  // 搜索设置
  search_position: 'bottom',
  auto_focus_search: true,

  // 内容设置
  auto_paste: 'double',
  image_ocr: false,
  copy_as_plain_text: false,
  paste_as_plain_text: true,
  confirm_delete: true,
  auto_sort: false,
  left_click_action: 'copy',

  // 通用设置
  hotkey: 'Alt+V',
  auto_start: false,
});

// 全局监听器标记
let isListening = false;

export function useSettings() {
  const loadSettings = async (): Promise<void> => {
    try {
      const result = await invoke<AppSettings>('get_settings');
      settings.value = result;
    } catch (error) {
      console.error('Failed to load settings:', error);
    }
  };

  const saveSettings = async (newSettings: AppSettings): Promise<void> => {
    try {
      await invoke('save_settings', { settings: newSettings });
      settings.value = newSettings;
      // 发送事件通知其他窗口设置已更改
      await emit('settings-changed', newSettings);
    } catch (error) {
      console.error('Failed to save settings:', error);
    }
  };

  const updateSetting = async <K extends keyof AppSettings>(
    key: K,
    value: AppSettings[K]
  ): Promise<void> => {
    const newSettings = { ...settings.value, [key]: value };
    await saveSettings(newSettings);
  };

  onMounted(() => {
    loadSettings();
    
    // 只设置一次全局监听器
    if (!isListening) {
      isListening = true;
      listen<AppSettings>('settings-changed', (event) => {
        settings.value = event.payload;
      });
    }
  });

  return {
    settings,
    loadSettings,
    saveSettings,
    updateSetting,
  };
}