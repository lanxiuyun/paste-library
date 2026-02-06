import { invoke } from '@tauri-apps/api/core';
import { ref, onMounted } from 'vue';
import type { AppSettings } from '@/types';

const settings = ref<AppSettings>({
  // 历史记录设置
  max_history_count: 5000,
  auto_cleanup_days: 30,

  // 窗口设置
  window_position: 'remember',
  window_width: 800,
  window_height: 600,
  scroll_to_top_on_activate: false,
  switch_to_all_on_activate: true,

  // 音效设置
  copy_sound: false,

  // 搜索设置
  search_position: 'bottom',
  auto_focus_search: true,
  clear_search_on_activate: false,

  // 内容设置
  auto_paste: 'double',
  image_ocr: false,
  copy_as_plain_text: false,
  paste_as_plain_text: true,
  auto_favorite: false,
  confirm_delete: true,
  auto_sort: false,

  // 通用设置
  hotkey: 'Alt+V',
  auto_start: false,
  blacklist_apps: [],
});

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
  });

  return {
    settings,
    loadSettings,
    saveSettings,
    updateSetting,
  };
}