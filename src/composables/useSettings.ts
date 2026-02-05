import { invoke } from '@tauri-apps/api/core';
import { ref, onMounted } from 'vue';
import type { AppSettings } from '@/types';

const settings = ref<AppSettings>({
  max_history_count: 5000,
  hotkey: 'Alt+V',
  auto_start: false,
  auto_cleanup_days: 30,
  window_width: 800,
  window_height: 600,
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