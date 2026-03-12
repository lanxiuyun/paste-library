import { invoke } from '@tauri-apps/api/core';
import { writeText } from 'tauri-plugin-clipboard-x-api';

export function useFileOperations() {
  /**
   * 复制文件路径到剪贴板
   * @param filePaths 文件路径数组
   * @param singlePath 单个文件路径（当 filePaths 为空时使用）
   */
  const copyFilePath = async (filePaths?: string[], singlePath?: string): Promise<boolean> => {
    try {
      let pathToCopy = '';

      if (filePaths && filePaths.length > 0) {
        // 多文件时复制所有路径，用换行符分隔（Windows风格 \r\n）
        pathToCopy = filePaths.join('\r\n') + '\r\n';
      } else if (singlePath) {
        // 使用单个路径，末尾添加换行符
        pathToCopy = singlePath + '\r\n';
      }

      if (pathToCopy) {
        await writeText(pathToCopy);
        return true;
      }
      return false;
    } catch (error) {
      console.error('Failed to copy file path:', error);
      return false;
    }
  };

  /**
   * 格式化文件大小
   * @param bytes 字节数
   * @returns 格式化后的字符串
   */
  const formatFileSize = (bytes: number): string => {
    if (bytes === 0) return '0 B';
    
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    const k = 1024;
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    const size = parseFloat((bytes / Math.pow(k, i)).toFixed(2));
    
    return `${size} ${units[i]}`;
  };

  /**
   * 判断是否为图片文件
   * @param filename 文件名
   * @returns 是否为图片
   */
  const isImageFile = (filename: string): boolean => {
    const imageExtensions = ['.jpg', '.jpeg', '.png', '.gif', '.bmp', '.webp', '.svg', '.ico'];
    const lowerFilename = filename.toLowerCase();
    return imageExtensions.some(ext => lowerFilename.endsWith(ext));
  };

  /**
   * 打开文件
   * @param path 文件路径
   */
  const openFile = async (path: string): Promise<boolean> => {
    try {
      await invoke('open_file', { path });
      return true;
    } catch (error) {
      console.error('Failed to open file:', error);
      return false;
    }
  };

  /**
   * 在文件夹中显示文件
   * @param path 文件路径
   */
  const showInFolder = async (path: string): Promise<boolean> => {
    try {
      await invoke('show_in_folder', { path });
      return true;
    } catch (error) {
      console.error('Failed to show in folder:', error);
      return false;
    }
  };

  /**
   * 获取文件名（从路径中提取）
   * @param path 文件路径
   * @returns 文件名
   */
  const getFileName = (path: string): string => {
    return path.split(/[/\\]/).pop() || path;
  };

  /**
   * 检查路径是否为目录（简单判断）
   * @param path 路径
   * @returns 是否为目录
   */
  const isDirectory = (path: string): boolean => {
    try {
      const lastPart = path.split(/[/\\]/).pop() || '';
      return !lastPart.includes('.');
    } catch {
      return false;
    }
  };

  return {
    copyFilePath,
    formatFileSize,
    isImageFile,
    openFile,
    showInFolder,
    getFileName,
    isDirectory,
  };
}
