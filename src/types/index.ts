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
 * 标签定义
 */
export interface Tag {
  id: string;
  name: string;
  color: string;
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
  tags?: string[];
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

  // 智能激活设置 (新增)
  // 如果激活时间与上次复制间隔<5秒，自动回到顶部、切换全部、聚焦搜索
  smart_activate: boolean;

  // 音效设置
  copy_sound: boolean;

  // 搜索设置
  search_position: 'top' | 'bottom';
  auto_focus_search: boolean;

  // 内容设置
  click_action: 'copy' | 'paste';
  double_click_action: 'copy' | 'paste';
  paste_shortcut: 'ctrl_v' | 'shift_insert';
  image_ocr: boolean;
  copy_as_plain_text: boolean;
  paste_as_plain_text: boolean;
  confirm_delete: boolean;
  auto_sort: boolean;

  // 通用设置
  hotkey: string;
  auto_start: boolean;
}
