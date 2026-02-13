import { ref, computed } from 'vue';
import type { ClipboardItem, ClipboardContentType } from '@/types';

/**
 * 搜索查询解析结果
 */
export interface ParsedQuery {
  /** 关键词（去除@语法后的纯文本搜索词） */
  keywords: string[];
  /** 标签过滤器 */
  tags: string[];
  /** 类型过滤器 */
  types: ClipboardContentType[];
  /** 原始查询字符串 */
  raw: string;
  /** 是否为有效查询 */
  isValid: boolean;
}

/**
 * 支持的类型别名映射
 */
const TYPE_ALIASES: Record<string, ClipboardContentType[]> = {
  '文本': ['text', 'html', 'rtf'],
  'text': ['text', 'html', 'rtf'],
  '文字': ['text', 'html', 'rtf'],
  'html': ['html'],
  'rtf': ['rtf'],
  '图片': ['image'],
  'image': ['image'],
  '照片': ['image'],
  '文件': ['file', 'files', 'folder'],
  'file': ['file', 'files', 'folder'],
  '文件夹': ['folder'],
  'folder': ['folder'],
};

/**
 * 解析搜索查询字符串
 * 
 * 支持的语法：
 * - @标签名 - 按标签搜索
 * - @类型 - 按类型搜索（如@文本、@html、@图片、@文件）
 * - 关键词 @标签 - 组合搜索
 * - 关键词 @类型 - 组合搜索
 * 
 * @param query 原始查询字符串
 * @returns 解析后的查询对象
 */
export function parseSearchQuery(query: string): ParsedQuery {
  const trimmed = query.trim();
  
  if (!trimmed) {
    return {
      keywords: [],
      tags: [],
      types: [],
      raw: '',
      isValid: false,
    };
  }

  const keywords: string[] = [];
  const tags: string[] = [];
  const types: ClipboardContentType[] = [];

  // 使用正则匹配 @语法
  // @xxx 匹配标签或类型
  const atPattern = /@([^\s@]+)/g;
  let match: RegExpExecArray | null;
  const atMatches: { index: number; length: number; value: string }[] = [];

  while ((match = atPattern.exec(trimmed)) !== null) {
    const value = match[1];
    atMatches.push({
      index: match.index,
      length: match[0].length,
      value,
    });

    // 判断是类型还是标签
    const lowerValue = value.toLowerCase();
    if (TYPE_ALIASES[lowerValue]) {
      types.push(...TYPE_ALIASES[lowerValue]);
    } else if (TYPE_ALIASES[value]) {
      types.push(...TYPE_ALIASES[value]);
    } else {
      // 不是已知类型，则认为是标签
      tags.push(value);
    }
  }

  // 提取关键词（去除@语法后的剩余文本）
  let remainingText = trimmed;
  // 从后往前替换，避免索引变化
  for (let i = atMatches.length - 1; i >= 0; i--) {
    const { index, length } = atMatches[i];
    remainingText = remainingText.slice(0, index) + remainingText.slice(index + length);
  }

  // 清理并分割关键词
  const cleanedKeywords = remainingText
    .trim()
    .split(/\s+/)
    .filter(k => k.length > 0);

  keywords.push(...cleanedKeywords);

  // 去重
  const uniqueTags = [...new Set(tags)];
  const uniqueTypes = [...new Set(types)];

  return {
    keywords,
    tags: uniqueTags,
    types: uniqueTypes,
    raw: trimmed,
    isValid: keywords.length > 0 || uniqueTags.length > 0 || uniqueTypes.length > 0,
  };
}

/**
 * 检查Item是否匹配解析后的查询
 * 
 * @param item 剪贴板项
 * @param query 解析后的查询
 * @param fuzzyMatch 模糊匹配函数
 * @returns 是否匹配
 */
export function matchItemWithQuery(
  item: ClipboardItem,
  query: ParsedQuery,
  fuzzyMatch: (query: string, text: string) => boolean
): boolean {
  if (!query.isValid) {
    return true;
  }

  // 类型过滤
  if (query.types.length > 0) {
    if (!query.types.includes(item.content_type)) {
      return false;
    }
  }

  // 标签过滤
  if (query.tags.length > 0) {
    const itemTags = item.tags || [];
    const hasMatchingTag = query.tags.some(tag => 
      itemTags.some(itemTag => 
        itemTag.toLowerCase() === tag.toLowerCase()
      )
    );
    if (!hasMatchingTag) {
      return false;
    }
  }

  // 关键词过滤
  if (query.keywords.length > 0) {
    // 获取用于搜索的文本内容
    let searchText = item.content;
    
    // 对于HTML类型，去除HTML标签后再搜索
    if (item.content_type === 'html') {
      searchText = item.content.replace(/<[^>]*>/g, ' ').replace(/\s+/g, ' ').trim();
    }

    // 所有关键词都必须匹配
    const allKeywordsMatch = query.keywords.every(keyword => {
      // 在内容中搜索
      if (fuzzyMatch(keyword, searchText)) {
        return true;
      }
      // 在文件路径中搜索
      if (item.file_paths) {
        const pathText = item.file_paths.join(' ');
        if (fuzzyMatch(keyword, pathText)) {
          return true;
        }
      }
      return false;
    });

    if (!allKeywordsMatch) {
      return false;
    }
  }

  return true;
}

/**
 * 搜索历史记录管理
 */
const MAX_HISTORY_ITEMS = 20;
const STORAGE_KEY = 'paste_library_search_history';

/**
 * 获取搜索历史记录
 */
export function getSearchHistory(): string[] {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      return JSON.parse(stored);
    }
  } catch {
    // 忽略解析错误
  }
  return [];
}

/**
 * 添加搜索历史记录
 */
export function addSearchHistory(query: string): void {
  if (!query.trim()) return;
  
  try {
    const history = getSearchHistory();
    // 去重并将新查询移到最前面
    const filtered = history.filter(h => h !== query);
    filtered.unshift(query);
    
    // 限制数量
    const limited = filtered.slice(0, MAX_HISTORY_ITEMS);
    localStorage.setItem(STORAGE_KEY, JSON.stringify(limited));
  } catch {
    // 忽略存储错误
  }
}

/**
 * 删除单条搜索历史
 */
export function removeSearchHistory(query: string): void {
  try {
    const history = getSearchHistory();
    const filtered = history.filter(h => h !== query);
    localStorage.setItem(STORAGE_KEY, JSON.stringify(filtered));
  } catch {
    // 忽略存储错误
  }
}

/**
 * 清空搜索历史
 */
export function clearSearchHistory(): void {
  try {
    localStorage.removeItem(STORAGE_KEY);
  } catch {
    // 忽略存储错误
  }
}

/**
 * 计算热门标签
 * 
 * @param items 剪贴板历史记录
 * @param limit 返回数量限制
 * @returns 热门标签列表（按使用次数排序）
 */
export function getPopularTags(items: ClipboardItem[], limit = 10): { tag: string; count: number }[] {
  const tagCount = new Map<string, number>();
  
  items.forEach(item => {
    (item.tags || []).forEach(tag => {
      tagCount.set(tag, (tagCount.get(tag) || 0) + 1);
    });
  });

  return Array.from(tagCount.entries())
    .map(([tag, count]) => ({ tag, count }))
    .sort((a, b) => b.count - a.count)
    .slice(0, limit);
}

/**
 * 高亮文本中的匹配关键词
 * 
 * @param text 原始文本
 * @param keywords 关键词列表
 * @returns 高亮后的HTML字符串
 */
export function highlightMatches(text: string, keywords: string[]): string {
  if (!keywords.length) return text;

  let highlighted = text;
  
  // 按长度降序排序，优先匹配长词
  const sortedKeywords = [...keywords].sort((a, b) => b.length - a.length);
  
  for (const keyword of sortedKeywords) {
    if (!keyword.trim()) continue;
    
    // 转义正则特殊字符
    const escaped = keyword.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const regex = new RegExp(`(${escaped})`, 'gi');
    
    highlighted = highlighted.replace(regex, '<mark class="search-highlight">$1</mark>');
  }

  return highlighted;
}

/**
 * 创建智能搜索 composable
 */
export function useSmartSearch() {
  const searchQuery = ref('');
  const parsedQuery = computed(() => parseSearchQuery(searchQuery.value));
  const searchHistory = ref<string[]>([]);
  
  // 初始化历史记录
  searchHistory.value = getSearchHistory();

  /**
   * 执行搜索
   */
  const executeSearch = (query: string) => {
    searchQuery.value = query;
    addSearchHistory(query);
    searchHistory.value = getSearchHistory();
  };

  /**
   * 清除搜索
   */
  const clearSearch = () => {
    searchQuery.value = '';
  };

  /**
   * 移除历史记录
   */
  const removeHistory = (query: string) => {
    removeSearchHistory(query);
    searchHistory.value = getSearchHistory();
  };

  /**
   * 添加标签到搜索
   */
  const addTagToSearch = (tag: string) => {
    const current = searchQuery.value.trim();
    const tagQuery = `@${tag}`;
    
    // 检查是否已存在该标签
    if (current.includes(tagQuery)) {
      return;
    }
    
    if (current) {
      searchQuery.value = `${current} ${tagQuery}`;
    } else {
      searchQuery.value = tagQuery;
    }
  };

  /**
   * 添加类型到搜索
   */
  const addTypeToSearch = (type: string) => {
    const current = searchQuery.value.trim();
    const typeQuery = `@${type}`;
    
    // 检查是否已存在该类型
    if (current.includes(typeQuery)) {
      return;
    }
    
    if (current) {
      searchQuery.value = `${current} ${typeQuery}`;
    } else {
      searchQuery.value = typeQuery;
    }
  };

  return {
    searchQuery,
    parsedQuery,
    searchHistory,
    executeSearch,
    clearSearch,
    removeHistory,
    addTagToSearch,
    addTypeToSearch,
  };
}
