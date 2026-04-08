import type { ClipboardItem, PinnedSearch } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { computed, nextTick, ref, type Ref } from "vue";
import {
  parseSearchQuery,
  type ParsedQuery,
} from "@/composables/useSmartSearch";

export interface SearchOptions {
  activeTab: Ref<string>;
  searchQuery: Ref<string>;
  pinnedSearches: Ref<PinnedSearch[]>;
  history: Ref<ClipboardItem[]>;
  onSearchComplete?: (resultCount: number) => void;
  scrollerRef?: Ref<{ scrollToItem: (index: number, position: string) => void } | null>;
}

export interface SearchState {
  filteredHistory: Ref<ClipboardItem[]>;
  isSearching: Ref<boolean>;
  searchHasMore: Ref<boolean>;
  parsedQuery: Ref<ParsedQuery>;
  performSearch: (isLoadMore?: boolean) => Promise<void>;
  loadMoreResults: () => Promise<void>;
  handleSmartSearch: (shouldScrollToTop?: boolean) => Promise<void>;
}

const ITEMS_PER_PAGE = 50;

export function useSearch(options: SearchOptions): SearchState {
  const { activeTab, searchQuery, pinnedSearches, history, onSearchComplete, scrollerRef } =
    options;

  const filteredHistory = ref<ClipboardItem[]>([]);
  const isSearching = ref(false);
  const searchOffset = ref(0);
  const searchHasMore = ref(true);

  const parsedQuery = computed<ParsedQuery>(() =>
    parseSearchQuery(searchQuery.value),
  );

  /**
   * 执行异步搜索（调用 Rust 后端）
   * 支持标签过滤、类型过滤和关键词搜索
   * @param isLoadMore 是否为加载更多（累加数据）
   */
  const performSearch = async (isLoadMore = false): Promise<void> => {
    const query = searchQuery.value;
    const parsed = parseSearchQuery(query);

    // 如果不是加载更多，重置分页状态
    if (!isLoadMore) {
      searchOffset.value = 0;
      searchHasMore.value = true;
    }

    // 如果有钉住搜索，使用钉住的查询
    const pinnedSearch = pinnedSearches.value.find(
      (ps) => ps.id === activeTab.value,
    );

    let searchRequest: {
      keywords: string[];
      tags: string[];
      types: string[];
      limit: number;
      offset: number;
    };

    if (pinnedSearch) {
      const pinnedParsed = parseSearchQuery(pinnedSearch.query);
      if (pinnedParsed.isValid) {
        searchRequest = {
          keywords: pinnedParsed.keywords,
          tags: pinnedParsed.tags,
          types: pinnedParsed.types.map((t) => {
            const typeMap: Record<string, string> = {
              text: "text",
              html: "html",
              rtf: "rtf",
              image: "image",
              file: "file",
              folder: "folder",
              files: "files",
            };
            return typeMap[t] || "text";
          }),
          limit: ITEMS_PER_PAGE,
          offset: searchOffset.value,
        };
      } else {
        searchRequest = {
          keywords: [],
          tags: [],
          types: [],
          limit: ITEMS_PER_PAGE,
          offset: 0,
        };
      }
    } else if (activeTab.value !== "all" && !parsed.isValid) {
      // Tab 过滤（非搜索状态）
      const typeMap: Record<string, string[]> = {
        file: ["file", "folder", "files"],
        text: ["text", "html", "rtf"],
        image: ["image"],
      };
      const types = typeMap[activeTab.value] || [activeTab.value.toLowerCase()];

      searchRequest = {
        keywords: [],
        tags: [],
        types,
        limit: ITEMS_PER_PAGE,
        offset: searchOffset.value,
      };
    } else if (parsed.isValid) {
      // 普通搜索
      searchRequest = {
        keywords: parsed.keywords,
        tags: parsed.tags,
        types: parsed.types.map((t) => {
          const typeMap: Record<string, string> = {
            text: "text",
            html: "html",
            rtf: "rtf",
            image: "image",
            file: "file",
            folder: "folder",
            files: "files",
          };
          return typeMap[t] || "text";
        }),
        limit: ITEMS_PER_PAGE,
        offset: searchOffset.value,
      };
    } else {
      // 无搜索条件，显示全部
      if (!isLoadMore) {
        filteredHistory.value = history.value.slice(0, ITEMS_PER_PAGE);
        searchHasMore.value = history.value.length > ITEMS_PER_PAGE;
        onSearchComplete?.(filteredHistory.value.length);
      }
      return;
    }

    // 执行搜索：只有新搜索才显示加载状态，加载更多时不显示
    if (!isLoadMore) {
      isSearching.value = true;
    }
    try {
      const results = await invoke<ClipboardItem[]>(
        "search_clipboard_advanced",
        {
          request: searchRequest,
        },
      );

      if (isLoadMore) {
        // 加载更多：追加数据
        filteredHistory.value = [...filteredHistory.value, ...results];
      } else {
        // 新搜索：替换数据
        filteredHistory.value = results;
      }

      // 更新分页状态
      searchOffset.value += ITEMS_PER_PAGE;
      searchHasMore.value = results.length === ITEMS_PER_PAGE;

      onSearchComplete?.(filteredHistory.value.length);
    } finally {
      isSearching.value = false;
    }
  };

  /**
   * 加载更多搜索结果（滚动到底部触发）
   */
  const loadMoreResults = async (): Promise<void> => {
    if (isSearching.value) return;

    // 如果有搜索条件或标签过滤，调用后端加载更多
    if (searchQuery.value || activeTab.value !== "all") {
      if (!searchHasMore.value) return;
      await performSearch(true);
    } else {
      // 全部模式：从历史记录中加载更多
      const currentLength = filteredHistory.value.length;
      const moreItems = history.value.slice(
        currentLength,
        currentLength + ITEMS_PER_PAGE,
      );
      if (moreItems.length > 0) {
        filteredHistory.value = [...filteredHistory.value, ...moreItems];
      }
      // 更新是否有更多标志
      searchHasMore.value =
        currentLength + moreItems.length < history.value.length;
    }
  };

  /**
   * 搜索处理：执行搜索并可选滚动到顶部
   */
  const handleSmartSearch = async (shouldScrollToTop = true): Promise<void> => {
    // 立即执行搜索（不防抖）
    await performSearch();

    nextTick(() => {
      // 只有需要时才滚动到顶部（新搜索时滚动，加载更多时不滚动）
      if (shouldScrollToTop && scrollerRef?.value) {
        scrollerRef.value.scrollToItem(0, "start");
      }
    });
  };

  return {
    filteredHistory,
    isSearching,
    searchHasMore,
    parsedQuery,
    performSearch,
    loadMoreResults,
    handleSmartSearch,
  };
}
