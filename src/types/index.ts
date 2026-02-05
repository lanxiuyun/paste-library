/**
 * 剪贴板内容类型
 */
export type ClipboardContentType = 'text' | 'html' | 'rtf' | 'image' | 'file';

/**
 * 剪贴板历史记录项
 */
export interface ClipboardItem {
  id: number;
  content_type: ClipboardContentType;
  content: string;
  created_at: string;
  content_hash: string;
}

/**
 * 创建剪贴板项请求
 */
export interface CreateClipboardItemRequest {
  content_type: ClipboardContentType;
  content: string;
}

/**
 * 搜索请求
 */
export interface SearchRequest {
  query: string;
  limit?: number;
}

/**
 * 获取历史记录请求
 */
export interface GetHistoryRequest {
  limit?: number;
  offset?: number;
}

/**
 * 清空历史请求
 */
export interface ClearHistoryRequest {
  keep_count?: number;
  keep_days?: number;
}

/**
 * 应用设置
 */
export interface AppSettings {
  max_history_count: number;
  hotkey: string;
  auto_start: boolean;
  auto_cleanup_days: number;
  window_width: number;
  window_height: number;
  blacklist_apps: string[];
}