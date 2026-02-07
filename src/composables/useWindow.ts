import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { ref, onMounted, onUnmounted } from 'vue';

const isClipboardVisible = ref(false);

export function useWindow() {
  const toggleClipboardWindow = async (): Promise<void> => {
    try {
      const result = await invoke<boolean>('toggle_clipboard_window');
      isClipboardVisible.value = result;
    } catch (error) {
      console.error('Failed to toggle clipboard window:', error);
    }
  };

  const hideClipboardWindow = async (): Promise<void> => {
    try {
      await invoke('hide_clipboard_window');
      isClipboardVisible.value = false;
    } catch (error) {
      console.error('Failed to hide clipboard window:', error);
    }
  };

  const openClipboardWindow = async (): Promise<void> => {
    try {
      const result = await invoke<boolean>('toggle_clipboard_window');
      isClipboardVisible.value = result;
    } catch (error) {
      console.error('Failed to open clipboard window:', error);
    }
  };

  let unlistenBlur: (() => void) | null = null;

  onMounted(async () => {
    unlistenBlur = await listen('clipboard-window-blur', () => {
      isClipboardVisible.value = false;
    });
  });

  onUnmounted(() => {
    if (unlistenBlur) {
      unlistenBlur();
    }
  });

  return {
    isClipboardVisible,
    toggleClipboardWindow,
    hideClipboardWindow,
    openClipboardWindow,
  };
}
