import { createApp } from "vue";
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import App from "./App.vue";
import ClipboardView from "./ClipboardView.vue";

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
