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
import { decodeHtmlEntities, stripHtmlAndDecode } from '@/utils/htmlUtils';

const history = ref<ClipboardItem[]>([]);
const isListening = ref(false);
const lastCopyTime = ref<number>(Date.now());
// 标记是否是应用内复制（用于区分系统剪贴板变化和应用内复制）
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
      // 检查是否是应用内复制（如果是，跳过智能激活的时间记录）
      const wasInternalCopy = isInternalCopy.value;

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
          isInternalCopy: wasInternalCopy,
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
          isInternalCopy: wasInternalCopy,
        });
      } else if (result.html) {
        // HTML 类型：存储原始 HTML 内容
        // 参考 EcoPaste 处理方式：
        // - content: 原始 HTML
        // - text: 剪贴板提供的纯文本（用于纯文本粘贴）
        const htmlContent = result.html.value;
        // 对纯文本进行 HTML 实体解码（如 &#160; -> 空格）
        const plainText = decodeHtmlEntities(result.text?.value || '');

        // 存储 HTML 内容，纯文本版本通过内存缓存
        await invoke('add_clipboard_item', {
          text: plainText,  // 解码后的纯文本用于预览和纯文本粘贴
          html: htmlContent,  // 原始 HTML
          isInternalCopy: wasInternalCopy,
        });
      } else if (result.text) {
        // 纯文本类型
        await invoke('add_clipboard_item', {
          text: result.text.value,
          html: null,
          isInternalCopy: wasInternalCopy,
        });
      }

      // 只有在不是应用内复制的情况下，才更新 lastCopyTime（用于智能激活）
      if (!wasInternalCopy) {
        lastCopyTime.value = Date.now();
      }

      // 重置内部复制标志（在 restoreToClipboard 中也有延迟重置，这里是保险措施）
      isInternalCopy.value = false;

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

  const sleep = (ms: number): Promise<void> =>
    new Promise((resolve) => setTimeout(resolve, ms));

  const restoreImageToClipboard = async (imagePath: string): Promise<void> => {
    const maxAttempts = 3;

    for (let attempt = 1; attempt <= maxAttempts; attempt += 1) {
      try {
        await writeImage(imagePath);
        return;
      } catch (error) {
        if (attempt === maxAttempts) {
          throw error;
        }

        // Windows 11 下图片写入剪贴板偶发句柄竞争，短暂退避后重试。
        await sleep(50 * attempt);
      }
    }
  };

  const restoreToClipboard = async (item: ClipboardItem, options?: { copyAsPlainText?: boolean }): Promise<void> => {
    // 标记为应用内复制（这样 handleClipboardChange 就不会更新 lastCopyTime）
    isInternalCopy.value = true;

    // 辅助函数：获取纯文本内容（优先使用 text_content，否则从 content 提取）
    const getPlainText = (): string => {
      if (item.text_content && item.text_content.trim().length > 0) {
        return item.text_content;
      }
      // 备用方案：从 HTML/RTF 内容中提取纯文本（旧数据兼容）
      if (item.content_type === 'html' || item.content_type === 'rtf') {
        return stripHtmlAndDecode(item.content);
      }
      return item.content;
    };

    try {
      // 如果需要复制为纯文本，直接使用存储的纯文本内容
      if (options?.copyAsPlainText && (item.content_type === 'html' || item.content_type === 'rtf')) {
        await writeText(getPlainText());
        return;
      }

      switch (item.content_type) {
        case 'html': {
          // 写入原始 HTML 到剪贴板
          // writeHTML(纯文本版本, HTML版本)
          // 纯文本版本：item.text_content（剪贴板直接提供的纯文本，或从 HTML 提取的备用）
          // HTML版本：item.content（原始 HTML）
          await writeHTML(getPlainText(), item.content);
          break;
        }
        case 'image':
          // 图片类型：使用缩略图路径或内容路径
          if (item.thumbnail_path) {
            await restoreImageToClipboard(item.thumbnail_path);
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
    } finally {
      // 延迟重置标志，确保剪贴板事件已处理
      // 延长到 500ms 以确保剪贴板变化事件被正确处理
      setTimeout(() => {
        isInternalCopy.value = false;
      }, 500);
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

  // 初始化函数 - 由组件在 onMounted 中调用
  const init = (): (() => void) => {
    loadHistory();
    startClipboardListening();
    const cleanupListener = setupClipboardListener();

    // 返回清理函数，供组件在 onUnmounted 中调用
    return () => {
      cleanupListener();
      stopClipboardListening();
    };
  };

  return {
    history,
    isListening,
    lastCopyTime,
    init,
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