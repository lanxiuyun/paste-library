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
  /** 窗口位置 X 坐标 (remember 模式使用) */
  window_pos_x?: number;
  /** 窗口位置 Y 坐标 (remember 模式使用) */
  window_pos_y?: number;

  // 智能激活设置 (新增)
  // 如果激活时间与上次复制间隔<5秒，自动回到顶部、切换全部、聚焦搜索
  smart_activate: boolean;

  // 音效设置
  copy_sound: boolean;

  // 搜索设置
  search_position: 'top' | 'bottom';
  /** 激活窗口时自动聚焦搜索框（与smart_activate独立） */
  focus_search_on_activate: boolean;

  // 内容设置
  click_action: 'copy' | 'paste' | 'none';
  double_click_action: 'copy' | 'paste' | 'none';
  paste_shortcut: 'ctrl_v' | 'shift_insert';
  hide_window_after_copy: boolean;
  image_ocr: boolean;
  copy_as_plain_text: boolean;
  paste_as_plain_text: boolean;
  confirm_delete: boolean;
  auto_sort: boolean;

  // 通用设置
  hotkey: string;
  auto_start: boolean;

  // 快捷键设置
  /** 数字键 1-9 快速粘贴修饰键组合，如 "ctrl", "ctrl+shift", "alt", "none" 等 */
  number_key_shortcut: string;
}

/**
 * 更新信息
 */
export interface UpdateInfo {
  version: string;
  notes: string;
  pub_date: string;
  platforms: Record<string, {
    signature: string;
    url: string;
  }>;
}

/**
 * 更新检查结果
 */
export interface CheckUpdateResult {
  has_update: boolean;
  current_version: string;
  latest_version?: string;
  update_info?: UpdateInfo;
}

/**
 * 固定搜索项
 */
export interface PinnedSearch {
  /** 唯一标识 */
  id: string;
  /** 显示标签 */
  label: string;
  /** 搜索查询 */
  query: string;
  /** 创建时间 */
  created_at: number;
}
