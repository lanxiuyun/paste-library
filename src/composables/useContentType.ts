import type { ClipboardContentType } from '@/types';

export interface ContentTypeInfo {
  label: string;
  icon: string;
  color: string;
}

const CONTENT_TYPE_MAP: Record<ClipboardContentType, ContentTypeInfo> = {
  text: { label: '文本', icon: '📝', color: '#52c41a' },
  html: { label: 'HTML', icon: '🌐', color: '#1890ff' },
  rtf: { label: '富文本', icon: '📄', color: '#722ed1' },
  image: { label: '图片', icon: '🖼️', color: '#fa8c16' },
  file: { label: '文件', icon: '📎', color: '#595959' },
  folder: { label: '文件夹', icon: '📁', color: '#faad14' },
  files: { label: '多文件', icon: '📦', color: '#13c2c2' },
};

export function useContentType() {
  /**
   * 获取内容类型的标签
   * @param type 内容类型
   * @returns 类型标签
   */
  const getTypeLabel = (type: ClipboardContentType): string => {
    return CONTENT_TYPE_MAP[type]?.label || '未知';
  };

  /**
   * 获取内容类型的图标
   * @param type 内容类型
   * @returns 类型图标
   */
  const getTypeIcon = (type: ClipboardContentType): string => {
    return CONTENT_TYPE_MAP[type]?.icon || '📋';
  };

  /**
   * 获取内容类型的颜色
   * @param type 内容类型
   * @returns 类型颜色
   */
  const getTypeColor = (type: ClipboardContentType): string => {
    return CONTENT_TYPE_MAP[type]?.color || '#8c8c8c';
  };

  /**
   * 获取内容类型的完整信息
   * @param type 内容类型
   * @returns 类型信息对象
   */
  const getTypeInfo = (type: ClipboardContentType): ContentTypeInfo => {
    return CONTENT_TYPE_MAP[type] || { label: '未知', icon: '📋', color: '#8c8c8c' };
  };

  /**
   * 判断是否为文本类型
   * @param type 内容类型
   * @returns 是否为文本类型
   */
  const isTextType = (type: ClipboardContentType): boolean => {
    return type === 'text' || type === 'html' || type === 'rtf';
  };

  /**
   * 判断是否为文件类型
   * @param type 内容类型
   * @returns 是否为文件类型
   */
  const isFileType = (type: ClipboardContentType): boolean => {
    return type === 'file' || type === 'files' || type === 'folder';
  };

  /**
   * 判断是否为图片类型
   * @param type 内容类型
   * @returns 是否为图片类型
   */
  const isImageType = (type: ClipboardContentType): boolean => {
    return type === 'image';
  };

  return {
    getTypeLabel,
    getTypeIcon,
    getTypeColor,
    getTypeInfo,
    isTextType,
    isFileType,
    isImageType,
  };
}
