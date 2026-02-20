import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import {
  startListening,
  stopListening,
  onClipboardChange,
  writeText,
  writeHTML,
  writeImage,
  writeFiles,
  type ReadClipboard,
} from 'tauri-plugin-clipboard-x-api';
import type { ClipboardItem, ClipboardContentType, ClipboardMetadata, GetHistoryRequest, SearchRequest, ClearHistoryRequest } from '@/types';

const history = ref<ClipboardItem[]>([]);
const isListening = ref(false);
const lastCopyTime = ref<number>(Date.now());
const isInternalCopy = ref(false);

export function useClipboard() {
  const loadHistory = async (limit?: number, offset = 0): Promise<void> => {
    try {
      const request: GetHistoryRequest = limit ? { limit, offset } : { offset };
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
      unlisten = await onClipboardChange(async (result: ReadClipboard) => {
        await handleClipboardChange(result);
      });
    };

    initListener();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  };

  const handleClipboardChange = async (result: ReadClipboard): Promise<void> => {
    try {
      const wasInternalCopy = isInternalCopy.value;

      // Priority: files > image > html > text
      if (result.files) {
        const paths = result.files.value;
        const contentType: ClipboardContentType = paths.length === 1
          ? (isPathDirectory(paths[0]) ? 'folder' : 'file')
          : 'files';

        const metadata: ClipboardMetadata = paths.length === 1
          ? { file_name: getFileName(paths[0]), file_size: result.files.count }
          : { item_count: paths.length };

        await invoke('add_clipboard_item_extended', {
          contentType,
          content: paths.join('\n'),
          filePaths: paths,
          metadata,
          isInternalCopy: wasInternalCopy,
        });
      } else if (result.image) {
        const metadata: ClipboardMetadata = {
          width: result.image.width,
          height: result.image.height,
          format: 'png',
        };

        await invoke('add_clipboard_item_extended', {
          contentType: 'image',
          content: result.image.value,
          thumbnailPath: result.image.value,
          metadata,
          isInternalCopy: wasInternalCopy,
        });
      } else if (result.html) {
        await invoke('add_clipboard_item', {
          text: result.text?.value || '',
          html: result.html.value,
          isInternalCopy: wasInternalCopy,
        });
      } else if (result.text) {
        await invoke('add_clipboard_item', {
          text: result.text.value,
          html: null,
          isInternalCopy: wasInternalCopy,
        });
      }

      if (!wasInternalCopy) {
        lastCopyTime.value = Date.now();
      }

      isInternalCopy.value = false;
      await loadHistory();
    } catch (error) {
      console.error('Failed to handle clipboard change:', error);
    }
  };

  const isPathDirectory = (path: string): boolean => {
    const lastPart = path.split(/[/\\]/).pop() || '';
    return !lastPart.includes('.');
  };

  const getFileName = (path: string): string => {
    return path.split(/[/\\]/).pop() || path;
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

  const clearAllItems = async (): Promise<void> => {
    await clearHistory(0, undefined);
  };

  const restoreToClipboard = async (item: ClipboardItem, options?: { copyAsPlainText?: boolean }): Promise<void> => {
    try {
      isInternalCopy.value = true;
      
      let content = item.content;
      if (options?.copyAsPlainText && (item.content_type === 'html' || item.content_type === 'rtf')) {
        content = content.replace(/<[^>]*>/g, '');
        await writeText(content);
        return;
      }

      switch (item.content_type) {
        case 'html': {
          const plainText = item.content.replace(/<[^>]*>/g, '').trim();
          await writeHTML(plainText, item.content);
          break;
        }
        case 'image':
          if (item.thumbnail_path) {
            await writeImage(item.thumbnail_path);
          }
          break;
        case 'file':
        case 'folder':
        case 'files':
          if (item.file_paths && item.file_paths.length > 0) {
            await writeFiles(item.file_paths);
          }
          break;
        default:
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
    lastCopyTime,
    loadHistory,
    startClipboardListening,
    stopClipboardListening,
    searchHistory,
    deleteItem,
    clearHistory,
    clearAllItems,
    restoreToClipboard,
    formatRelativeTime,
    getContentPreview,
  };
}
