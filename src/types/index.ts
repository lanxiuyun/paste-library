/**
 * 剪贴板内容类型
 */
export type ClipboardContentType =
  | 'text'
  | 'html'
  | 'rtf'
  | 'image'
  | 'file'
  | 'folder'
  | 'files';

/**
 * 剪贴板元数据
 */
export interface ClipboardMetadata {
  // 图片相关
  width?: number;
  height?: number;
  format?: string;

  // 文件相关
  file_name?: string;
  file_size?: number;
  mime_type?: string;

  // 文件夹相关
  folder_name?: string;
  item_count?: number;
}

/**
 * 剪贴板历史记录项
 */
export interface ClipboardItem {
  id: number;
  content_type: ClipboardContentType;
  content: string;
  created_at: string;
  content_hash: string;
  // 新增字段
  metadata?: ClipboardMetadata;
  file_paths?: string[];
  thumbnail_path?: string;
  is_favorite?: boolean;
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
  window_position: 'remember' | 'center' | 'cursor';
  window_width: number;
  window_height: number;
  scroll_to_top_on_activate: boolean;
  switch_to_all_on_activate: boolean;

  // 音效设置
  copy_sound: boolean;

  // 搜索设置
  search_position: 'top' | 'bottom';
  auto_focus_search: boolean;
  clear_search_on_activate: boolean;

  // 内容设置
  auto_paste: 'off' | 'single' | 'double';
  image_ocr: boolean;
  copy_as_plain_text: boolean;
  paste_as_plain_text: boolean;
  auto_favorite: boolean;
  confirm_delete: boolean;
  auto_sort: boolean;
  left_click_action: 'copy' | 'paste';

  // 通用设置
  hotkey: string;
  auto_start: boolean;
  blacklist_apps: string[];
}
