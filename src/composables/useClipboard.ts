import { invoke } from '@tauri-apps/api/core';
import { ref, onMounted, onUnmounted } from 'vue';
import {
  startListening,
  stopListening,
  onClipboardChange,
  readText,
  readHTML,
  writeText,
  writeHTML,
} from 'tauri-plugin-clipboard-x-api';
import type { ClipboardItem, GetHistoryRequest, SearchRequest, ClearHistoryRequest } from '@/types';

const history = ref<ClipboardItem[]>([]);
const isListening = ref(false);

export function useClipboard() {
  const loadHistory = async (limit = 100, offset = 0): Promise<void> => {
    try {
      const request: GetHistoryRequest = { limit, offset };
      const result = await invoke<ClipboardItem[]>('get_clipboard_history', { request });
      history.value = result;
    } catch (error) {
      console.error('Failed to load clipboard history:', error);
    }
  };

  const startClipboardListening = async (): Promise<void> => {
    try {
      await startListening();
      isListening.value = true;
      console.log('Clipboard listening started');
    } catch (error) {
      console.error('Failed to start clipboard listening:', error);
    }
  };

  const stopClipboardListening = async (): Promise<void> => {
    try {
      await stopListening();
      isListening.value = false;
      console.log('Clipboard listening stopped');
    } catch (error) {
      console.error('Failed to stop clipboard listening:', error);
    }
  };

  const setupClipboardListener = (): (() => void) => {
    let unlisten: (() => void) | null = null;

    const initListener = async () => {
      unlisten = await onClipboardChange(async () => {
        await handleClipboardChange();
      });
    };

    initListener();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  };

  const handleClipboardChange = async (): Promise<void> => {
    try {
      const text = await readText();
      let html: string | undefined;
      
      try {
        html = await readHTML();
      } catch {
      }

      await invoke('add_clipboard_item', { 
        text,
        html: html || null 
      });
      
      await loadHistory();
    } catch (error) {
      console.error('Failed to handle clipboard change:', error);
    }
  };

  const searchHistory = async (query: string, limit = 100): Promise<void> => {
    try {
      const request: SearchRequest = { query, limit };
      const result = await invoke<ClipboardItem[]>('search_clipboard_history', { request });
      history.value = result;
    } catch (error) {
      console.error('Failed to search clipboard history:', error);
    }
  };

  const deleteItem = async (id: number): Promise<void> => {
    try {
      await invoke('delete_clipboard_item', { id });
      await loadHistory();
    } catch (error) {
      console.error('Failed to delete clipboard item:', error);
    }
  };

  const clearHistory = async (keepCount?: number, keepDays?: number): Promise<number> => {
    try {
      const request: ClearHistoryRequest = { keep_count: keepCount, keep_days: keepDays };
      const result = await invoke<number>('clear_clipboard_history', { request });
      await loadHistory();
      return result;
    } catch (error) {
      console.error('Failed to clear clipboard history:', error);
      return 0;
    }
  };

  const restoreToClipboard = async (item: ClipboardItem): Promise<void> => {
    try {
      if (item.content_type === 'html') {
        await writeHTML(item.content, '');
      } else {
        await writeText(item.content);
      }
    } catch (error) {
      console.error('Failed to restore to clipboard:', error);
    }
  };

  const formatRelativeTime = (dateString: string): string => {
    const date = new Date(dateString);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffSec = Math.floor(diffMs / 1000);
    const diffMin = Math.floor(diffSec / 60);
    const diffHour = Math.floor(diffMin / 60);
    const diffDay = Math.floor(diffHour / 24);

    if (diffSec < 60) return '刚刚';
    if (diffMin < 60) return `${diffMin}分钟前`;
    if (diffHour < 24) return `${diffHour}小时前`;
    if (diffDay < 30) return `${diffDay}天前`;
    return date.toLocaleDateString('zh-CN');
  };

  const getContentPreview = (content: string, maxLength = 100): string => {
    let text = content;
    text = text.replace(/<[^>]*>/g, '');
    text = text.trim();
    if (text.length > maxLength) {
      text = text.substring(0, maxLength) + '...';
    }
    return text;
  };

  const getTypeIcon = (type: string): string => {
    switch (type) {
      case 'text':
        return '📝';
      case 'html':
        return '🌐';
      case 'rtf':
        return '📄';
      default:
        return '📋';
    }
  };

  onMounted(() => {
    loadHistory();
    startClipboardListening();
    const cleanup = setupClipboardListener();

    onUnmounted(() => {
      cleanup();
      stopClipboardListening();
    });
  });

  return {
    history,
    isListening,
    loadHistory,
    startClipboardListening,
    stopClipboardListening,
    searchHistory,
    deleteItem,
    clearHistory,
    restoreToClipboard,
    formatRelativeTime,
    getContentPreview,
    getTypeIcon,
  };
}