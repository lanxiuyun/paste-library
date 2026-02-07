import { invoke } from '@tauri-apps/api/core';
import { ref, onMounted, onUnmounted } from 'vue';
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
      unlisten = await onClipboardChange(async (result) => {
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
      // 优先级: files > image > html > rtf > text
      if (result.files) {
        // 文件类型
        const paths = result.files.value;
        const contentType: ClipboardContentType = paths.length === 1 
          ? (await isDirectory(paths[0]) ? 'folder' : 'file')
          : 'files';
        
        const metadata: ClipboardMetadata = paths.length === 1
          ? { file_name: getFileName(paths[0]), file_size: result.files.count }
          : { item_count: paths.length };

        await invoke('add_clipboard_item_extended', {
          contentType,
          content: paths.join('\n'),
          filePaths: paths,
          metadata,
        });
      } else if (result.image) {
        // 图片类型
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
        });
      } else if (result.html) {
        // HTML 类型
        await invoke('add_clipboard_item', {
          text: result.text?.value || '',
          html: result.html.value,
        });
      } else if (result.text) {
        // 纯文本类型
        await invoke('add_clipboard_item', {
          text: result.text.value,
          html: null,
        });
      }

      // 记录上次复制时间
      lastCopyTime.value = Date.now();
      
      await loadHistory();
    } catch (error) {
      console.error('Failed to handle clipboard change:', error);
    }
  };

  // 辅助函数：检查路径是否为目录
  const isDirectory = async (path: string): Promise<boolean> => {
    try {
      // 简单判断：如果路径没有扩展名，可能是目录
      const lastPart = path.split(/[/\\]/).pop() || '';
      return !lastPart.includes('.');
    } catch {
      return false;
    }
  };

  // 辅助函数：获取文件名
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

  const restoreToClipboard = async (item: ClipboardItem): Promise<void> => {
    try {
      switch (item.content_type) {
        case 'html':
          await writeHTML(item.content, '');
          break;
        case 'image':
          // 图片类型：使用缩略图路径或内容路径
          if (item.thumbnail_path) {
            await writeImage(item.thumbnail_path);
          }
          break;
        case 'file':
        case 'folder':
        case 'files':
          // 文件类型：使用文件路径列表
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
    getTypeIcon,
  };
}