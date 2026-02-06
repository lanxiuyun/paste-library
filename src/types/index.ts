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
  // 历史记录设置
  max_history_count: number;
  auto_cleanup_days: number;

  // 窗口设置
  window_position: string;
  window_width: number;
  window_height: number;
  scroll_to_top_on_activate: boolean;
  switch_to_all_on_activate: boolean;

  // 音效设置
  copy_sound: boolean;

  // 搜索设置
  search_position: string;
  auto_focus_search: boolean;
  clear_search_on_activate: boolean;

  // 内容设置
  auto_paste: string;
  image_ocr: boolean;
  copy_as_plain_text: boolean;
  paste_as_plain_text: boolean;
  auto_favorite: boolean;
  confirm_delete: boolean;
  auto_sort: boolean;

  // 通用设置
  hotkey: string;
  auto_start: boolean;
  blacklist_apps: string[];
}
