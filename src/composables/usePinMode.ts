import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { ref, onMounted } from 'vue';

const isPinned = ref(false);
const windowOpacity = ref(1.0);

// 全局监听器标记
let isListening = false;

export function usePinMode() {
  const loadPinMode = async (): Promise<void> => {
    try {
      const result = await invoke<boolean>('get_pin_mode');
      isPinned.value = result;
    } catch (error) {
      console.error('Failed to load pin mode:', error);
    }
  };

  const setPinMode = async (enabled: boolean): Promise<void> => {
    try {
      await invoke('set_pin_mode', { enabled });
      isPinned.value = enabled;
    } catch (error) {
      console.error('Failed to set pin mode:', error);
    }
  };

  const togglePinMode = async (): Promise<void> => {
    try {
      const result = await invoke<boolean>('toggle_pin_mode');
      isPinned.value = result;
    } catch (error) {
      console.error('Failed to toggle pin mode:', error);
    }
  };

  const updatePinShortcut = async (oldShortcut: string, newShortcut: string): Promise<void> => {
    try {
      await invoke('update_pin_shortcut', { oldShortcut, newShortcut });
    } catch (error) {
      console.error('Failed to update pin shortcut:', error);
    }
  };

  const setWindowOpacity = (opacity: number): void => {
    windowOpacity.value = opacity;
  };

  onMounted(() => {
    loadPinMode();

    // 只设置一次全局监听器
    if (!isListening) {
      isListening = true;

      // 监听钉住模式变化
      listen<{ pinned: boolean }>('pin-mode-changed', (event) => {
        isPinned.value = event.payload.pinned;
      });

      // 监听窗口透明度变化
      listen<{ opacity: number }>('window-opacity-change', (event) => {
        windowOpacity.value = event.payload.opacity;
      });
    }
  });

  return {
    isPinned,
    windowOpacity,
    loadPinMode,
    setPinMode,
    togglePinMode,
    updatePinShortcut,
    setWindowOpacity,
  };
}
