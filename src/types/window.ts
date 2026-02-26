import type { UnlistenFn } from '@tauri-apps/api/event';

declare global {
  interface Window {
    /** 窗口聚焦事件清理函数 */
    __unlistenFocus?: UnlistenFn;
    /** 窗口失焦事件清理函数 */
    __unlistenBlur?: UnlistenFn;
    /** 剪贴板窗口引用 */
    __clipboardWindow?: unknown;
  }
}

export {};
