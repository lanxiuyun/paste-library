import { createApp } from "vue";
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { writeText } from 'tauri-plugin-clipboard-x-api';
import App from "./App.vue";
import ClipboardView from "./ClipboardView.vue";
import { markPendingRemoteClipboardText } from '@/composables/useLanSyncClipboardBridge';
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css';

const path = window.location.pathname;
const rootComponent = path === '/clipboard' ? ClipboardView : App;

createApp(rootComponent).mount("#app");

listen('shortcut-triggered', async () => {
  if (path !== '/clipboard') {
    try {
      await invoke('toggle_clipboard_window');
    } catch (error) {
      console.error('Failed to toggle window:', error);
    }
  }
});

listen<string>('lan-sync-copy-text', async (event) => {
  try {
    markPendingRemoteClipboardText(event.payload);

    // 仅由主窗口实际写入系统剪贴板，剪贴板窗口只负责标记本地桥接状态，
    // 避免两个窗口同时写入导致重复事件或剪贴板竞争。
    if (path !== '/clipboard') {
      await writeText(event.payload);
    }
  } catch (error) {
    console.error('Failed to write synced clipboard text:', error);
  }
});
