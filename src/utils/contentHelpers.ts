import type { ClipboardContentType } from '@/types';

const TYPE_LABEL_MAP: Record<ClipboardContentType, string> = {
  text: '文本',
  html: 'HTML',
  rtf: '富文本',
  image: '图片',
  file: '文件',
  folder: '文件夹',
  files: '多文件',
};

/**
 * 获取内容类型的标签
 * @param type 内容类型
 * @returns 类型标签
 */
export function getTypeLabel(type: ClipboardContentType): string {
  return TYPE_LABEL_MAP[type] || '未知';
}

/**
 * 判断是否为图片文件
 * @param filename 文件名
 * @returns 是否为图片
 */
export function isImageFile(filename: string): boolean {
  const imageExtensions = ['.jpg', '.jpeg', '.png', '.gif', '.bmp', '.webp', '.svg', '.ico'];
  const lowerFilename = filename.toLowerCase();
  return imageExtensions.some(ext => lowerFilename.endsWith(ext));
}

/**
 * 去除 HTML 标签
 * @param html HTML 字符串
 * @returns 纯文本
 */
export function stripHtmlTags(html: string): string {
  return html.replace(/<[^>]*>/g, '');
}

/**
 * 获取内容预览
 * @param content 内容
 * @param maxLength 最大长度
 * @returns 预览文本
 */
export function getContentPreview(content: string, maxLength: number = 100): string {
  let text = content;
  text = stripHtmlTags(text);
  text = text.trim();
  if (text.length > maxLength) {
    text = text.substring(0, maxLength) + '...';
  }
  return text;
}

/**
 * 获取内容类型的图标
 * @param type 内容类型
 * @returns 图标字符
 */
export function getTypeIcon(type: ClipboardContentType): string {
  switch (type) {
    case 'text': return '📝';
    case 'html': return '🌐';
    case 'rtf': return '📄';
    case 'image': return '🖼️';
    case 'file': return '📎';
    case 'folder': return '📁';
    case 'files': return '📦';
    default: return '📋';
  }
}

/**
 * 检查路径是否为目录（简单判断）
 * @param path 路径
 * @returns 是否为目录
 */
export function isDirectory(path: string): boolean {
  try {
    const lastPart = path.split(/[/\\]/).pop() || '';
    return !lastPart.includes('.');
  } catch {
    return false;
  }
}
