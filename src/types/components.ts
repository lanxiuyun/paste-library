/**
 * 导航项类型
 */
export interface MenuItem {
  key: string;
  label: string;
  icon?: string;
}

/**
 * 标签项类型
 */
export interface TabItem {
  key: string;
  label: string;
  isPinned?: boolean;
  query?: string;
}

/**
 * 设置项类型
 */
export interface SettingOption {
  label: string;
  value: string | number | boolean;
  description?: string;
}

/**
 * 上下文菜单项类型
 */
export interface ContextMenuItem {
  action: string;
  label: string;
  icon?: string;
  shortcut?: string;
  divider?: boolean;
  danger?: boolean;
}

/**
 * 文件信息类型
 */
export interface FileInfo {
  name: string;
  path: string;
  size?: number;
  isDirectory: boolean;
}

/**
 * 图片加载状态
 */
export interface ImageLoadState {
  status: 'idle' | 'loading' | 'success' | 'error';
  retries: number;
  error?: string;
}
