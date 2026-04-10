<template>
  <div class="clipboard-list">
    <!-- 搜索栏 - 固定在顶部 -->
    <div class="search-bar search-bar-top">
      <div class="search-wrapper">
        <SmartSearch
          ref="smartSearchRef"
          v-model="searchQuery"
          :history="searchHistory"
          :items="history"
          @search="handleSmartSearch"
          @search-commit="handleSearchCommit"
          @clear-history="removeSearchHistory"
          @clear-all-history="clearAllHistory"
        />
        <button
          v-if="canPinCurrentSearch"
          class="icon-btn"
          title="固定当前搜索"
          @click="pinCurrentSearch"
        >
          <BookmarkPlus :size="18" />
        </button>
        <!-- 钉住模式按钮 -->
        <button
          class="icon-btn"
          :class="{ 'is-active': isPinned }"
          :title="
            isPinned
              ? '取消钉住'
              : `钉住面板 (${settings.pin_shortcut || 'Ctrl+Shift+P'})`
          "
          @click="togglePinMode"
        >
          <PinIcon :size="18" />
        </button>
        <!-- 设置按钮 -->
        <button
          class="icon-btn"
          title="打开设置"
          @click="openSettings"
        >
          <SettingsIcon :size="18" />
        </button>
      </div>
    </div>

    <!-- 标签栏 -->
    <TabBar
      :active-fixed-tab="activeFixedTab"
      :active-pinned-ids="activePinnedIds"
      :fixed-tabs="FIXED_TABS"
      :pinned-searches="pinnedSearches"
      @tab-click="handleTabClick"
      @pinned-click="handlePinnedTabClick"
      @unpin="handleUnpinSearch"
      @reorder="reorderPinnedSearches"
    />

    <!-- 列表内容 -->
    <div ref="listContainerRef" class="list-container" tabindex="-1">
      <!-- 搜索加载指示器 -->
      <div v-if="isSearching" class="search-loading">
        <svg
          class="loading-spinner"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <circle
            cx="12"
            cy="12"
            r="10"
            stroke-dasharray="60"
            stroke-dashoffset="20"
          />
        </svg>
        <span>搜索中...</span>
      </div>

      <EmptyState
        v-else-if="filteredHistory.length === 0"
        :has-search-query="!!searchQuery"
      />

      <DynamicScroller
        v-else
        ref="scrollerRef"
        class="scroller"
        :items="filteredHistory"
        :min-item-size="60"
        key-field="id"
        @scroll.native="handleScroll"
        v-slot="{ item, index, active }"
      >
        <DynamicScrollerItem
          :item="item"
          :active="active"
          :size-dependencies="[item.content, item.tags, item.content_type]"
        >
          <ClipboardItem
            :item="item"
            :index="index"
            :is-selected="selectedIndex === index"
            :show-tags="true"
            :highlight-keywords="parsedQuery.keywords"
            @click="handleItemClick"
            @dblclick="handleItemDoubleClick"
            @contextmenu="
              (event: MouseEvent) => handleItemContextMenu(event, item, index)
            "
            @quick-action="handleQuickAction"
            @remove-tag="handleRemoveTag"
            @tag-click="handleTagClick"
          />
        </DynamicScrollerItem>
      </DynamicScroller>
    </div>

    <!-- 右键上下文菜单 -->
    <ContextMenu
      v-model:visible="contextMenuVisible"
      :position="contextMenuPosition"
      :item="contextMenuItem"
      @action="handleContextMenuAction"
    />

    <!-- 抽屉编辑器 -->
    <DrawerEditor
      v-model:visible="drawerVisible"
      :item="drawerItem"
      @copy="handleDrawerCopy"
      @paste="handleDrawerPaste"
      @saveAsNew="handleSaveAsNew"
    />

    <!-- 标签管理器 -->
    <TagManager
      v-model:visible="tagManagerVisible"
      :item="tagManagerItem"
      @save="handleTagManagerSave"
    />

    <!-- 删除确认对话框 -->
    <DeleteConfirmDialog
      :visible="deleteConfirmVisible"
      @confirm="confirmDelete"
      @cancel="cancelDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { BookmarkPlus, Pin as PinIcon, Settings as SettingsIcon } from "lucide-vue-next";
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from "vue";
import { DynamicScroller, DynamicScrollerItem } from "vue-virtual-scroller";
import { useClipboard } from "@/composables/useClipboard";
import { useClipboardList } from "@/composables/useClipboardList";
import { useKeyboardNavigation } from "@/composables/useKeyboardNavigation";
import { usePinMode } from "@/composables/usePinMode";
import { usePinnedSearches } from "@/composables/usePinnedSearches";
import { useScrollHandler } from "@/composables/useScrollHandler";
import { useSearch } from "@/composables/useSearch";
import { useSettings } from "@/composables/useSettings";
import {
  addSearchHistory,
  clearSearchHistory,
  getSearchHistory,
  parseSearchQuery,
  removeTypeTokensFromQuery,
  removeSearchHistory as removeSearchHistoryItem,
} from "@/composables/useSmartSearch";
import ClipboardItem from "../ClipboardItem.vue";
import ContextMenu from "../ContextMenu.vue";
import DrawerEditor from "../DrawerEditor.vue";
import SmartSearch from "../SmartSearch.vue";
import TagManager from "../TagManager.vue";
import DeleteConfirmDialog from "./DeleteConfirmDialog.vue";
import EmptyState from "./EmptyState.vue";
import TabBar from "./TabBar.vue";

const FIXED_TABS = [
  { key: "all", label: "全部" },
  { key: "text", label: "文本" },
  { key: "image", label: "图片" },
  { key: "file", label: "文件" },
];

const TAB_TYPE_TOKEN: Record<string, string> = {
  text: "文本",
  image: "图片",
  file: "文件",
};

const TEXT_TYPES = ["text", "html", "rtf"];
const IMAGE_TYPES = ["image"];
const FILE_TYPES = ["file", "files", "folder"];

type DynamicScrollerAdapter = {
  scrollToItem: (index: number, position: string) => void;
};

const createScrollerAdapter = (
  scroller: InstanceType<typeof DynamicScroller>,
): DynamicScrollerAdapter => ({
  scrollToItem: (index: number, position: string) => {
    scroller.scrollToItem(index);
    if (position === "center") {
      nextTick(() => {
        const el = scroller.$el as HTMLElement | undefined;
        if (!el) return;
        const containerHeight = el.clientHeight;
        const selectedEl = el.querySelector(
          ".clipboard-item.is-selected",
        ) as HTMLElement | null;
        if (!selectedEl) return;
        const itemRect = selectedEl.getBoundingClientRect();
        const containerRect = el.getBoundingClientRect();
        const relativeTop = itemRect.top - containerRect.top;
        const offset =
          relativeTop - (containerHeight - itemRect.height) / 2;
        el.scrollTop += offset;
      });
    }
  },
});

const {
  history,
  init: initClipboard,
  loadHistory,
  deleteItem,
  restoreToClipboard,
} = useClipboard();

const { settings } = useSettings();
const { isPinned, togglePinMode, loadPinMode } = usePinMode();

const smartSearchRef = ref<InstanceType<typeof SmartSearch> | null>(null);
const scrollerRef = ref<InstanceType<typeof DynamicScroller> | null>(null);
const searchQuery = ref("");
const searchHistory = ref<string[]>([]);
const hasActivated = ref(false);
const savedSearchQuery = ref("");
const savedScrollPosition = ref(0);

const {
  pinnedSearches,
  canPinCurrentSearch,
  loadPinnedSearches,
  pinCurrentSearch,
  unpinSearch: unpinPinnedSearch,
  reorderPinnedSearches,
} = usePinnedSearches(searchQuery);

const scrollerForSearch = ref<DynamicScrollerAdapter | null>(null);
watch(scrollerRef, (value) => {
  scrollerForSearch.value = value ? createScrollerAdapter(value) : null;
});

const selectedIndexFallback = ref(-1);
const onSearchComplete = (resultCount: number) => {
  if (resultCount > 0 && selectedIndexFallback.value < 0) {
    selectedIndexFallback.value = 0;
  }
};

const {
  filteredHistory,
  isSearching,
  searchHasMore,
  parsedQuery,
  loadMoreResults,
  handleSmartSearch: handleSmartSearchInner,
} = useSearch({
  searchQuery,
  history,
  scrollerRef: scrollerForSearch,
  onSearchComplete,
});

const activePinnedIds = computed<string[]>(() => {
  const currentParsed = parsedQuery.value;
  if (!currentParsed.isValid) {
    return [];
  }

  return pinnedSearches.value
    .filter((ps) => {
      const pinnedParsed = parseSearchQuery(ps.query);
      if (!pinnedParsed.isValid) return false;

      const tagsMatch = pinnedParsed.tags.every((t) =>
        currentParsed.tags.some((ct) => ct.toLowerCase() === t.toLowerCase()),
      );
      const typesMatch = pinnedParsed.types.every((t) =>
        currentParsed.types.includes(t),
      );
      const keywordsMatch = pinnedParsed.keywords.every((k) =>
        currentParsed.keywords.some(
          (ck) => ck.toLowerCase() === k.toLowerCase(),
        ),
      );

      return tagsMatch && typesMatch && keywordsMatch;
    })
    .map((ps) => ps.id);
});

const derivedActiveTab = computed(() => {
  const types = parsedQuery.value.types;

  if (types.length === 0) {
    return "all";
  }

  if (types.every((type) => TEXT_TYPES.includes(type))) {
    return "text";
  }

  if (types.every((type) => IMAGE_TYPES.includes(type))) {
    return "image";
  }

  if (types.every((type) => FILE_TYPES.includes(type))) {
    return "file";
  }

  return "all";
});

const activeFixedTab = computed(() => derivedActiveTab.value);

const handleSmartSearch = async (
  query: string,
  shouldScrollToTop = true,
) => {
  searchQuery.value = query;
  await handleSmartSearchInner(shouldScrollToTop);
};

const resetDefaultList = () => {
  filteredHistory.value = history.value.slice(0, 50);
  searchHasMore.value = history.value.length > 50;
};

const resetPanelState = () => {
  searchQuery.value = "";
  savedSearchQuery.value = "";
  savedScrollPosition.value = 0;
  resetDefaultList();
  selectedIndexFallback.value = -1;
  selectedIndex.value = -1;
  if (scrollerRef.value) {
    scrollerRef.value.scrollToItem(0, "start");
  }
};

const {
  uiState,
  executeClipboardAction,
  handleItemClick,
  handleItemDoubleClick,
  handleItemContextMenu,
  handleQuickAction,
  handleDrawerCopy,
  handleDrawerPaste,
  handleSaveAsNew,
  confirmDelete,
  cancelDelete,
  handleTagManagerSave,
  handleRemoveTag,
  handleContextMenuAction,
} = useClipboardList({
  settings,
  isPinned,
  history,
  deleteItem,
  loadHistory,
  restoreToClipboard,
  resetPanelState,
});

const {
  contextMenuVisible,
  contextMenuPosition,
  contextMenuItem,
  drawerVisible,
  drawerItem,
  tagManagerVisible,
  tagManagerItem,
  deleteConfirmVisible,
  selectedIndex,
} = uiState;

watch(selectedIndex, (value) => {
  if (value >= 0 && hasActivated.value) {
    selectedIndexFallback.value = value;
  }
});

const smartSearchForKeyboard = ref<{ focus: () => void } | null>(null);
watch(smartSearchRef, (value) => {
  smartSearchForKeyboard.value = value
    ? {
        focus: () => value.focus(),
      }
    : null;
});

const scrollerForKeyboard = ref<DynamicScrollerAdapter | null>(null);
watch(scrollerRef, (value) => {
  scrollerForKeyboard.value = value ? createScrollerAdapter(value) : null;
});

const handleEscape = async () => {
  savedSearchQuery.value = searchQuery.value;
  savedScrollPosition.value = scrollerRef.value?.$el?.scrollTop || 0;
  hasActivated.value = false;
  selectedIndex.value = -1;
  await invoke("hide_clipboard_window");
};

const handleClearSearch = async () => {
  searchQuery.value = "";
  await handleSmartSearchInner(true);
};

const { handleKeyDown } = useKeyboardNavigation({
  filteredHistory,
  selectedIndex,
  searchQuery,
  settings,
  smartSearchRef: smartSearchForKeyboard,
  scrollerRef: scrollerForKeyboard,
  executeClipboardAction,
  onEscape: handleEscape,
  onTogglePinMode: togglePinMode,
  handleSmartSearch: handleClearSearch,
});

const { handleScroll, cleanup: cleanupScroll } = useScrollHandler({
  isSearching,
  searchHasMore,
  loadMoreResults,
});

const loadSearchHistory = () => {
  searchHistory.value = getSearchHistory();
};

const removeSearchHistory = (query: string) => {
  removeSearchHistoryItem(query);
  loadSearchHistory();
};

const clearAllHistory = () => {
  clearSearchHistory();
  loadSearchHistory();
};

const handleSearchCommit = (query: string) => {
  if (query.trim()) {
    addSearchHistory(query.trim());
    loadSearchHistory();
  }
  handleSmartSearch(query);
};

const handleTagClick = (tag: string) => {
  const tagQuery = `@${tag}`;
  const current = searchQuery.value.trim();

  if (current.includes(tagQuery)) {
    return;
  }

  if (current) {
    searchQuery.value = `${current} ${tagQuery}`;
  } else {
    searchQuery.value = tagQuery;
  }

  handleSmartSearch(searchQuery.value);
};

const handleTabClick = async (tabKey: string) => {
  const queryWithoutTypes = removeTypeTokensFromQuery(searchQuery.value);
  const typeToken = TAB_TYPE_TOKEN[tabKey];
  const nextQuery = typeToken
    ? `${queryWithoutTypes} @${typeToken}`.trim()
    : queryWithoutTypes;

  await handleSmartSearch(nextQuery);
};

const tokenizeQuery = (query: string) =>
  query
    .trim()
    .split(/\s+/)
    .filter((token) => token.length > 0);

const mergeQueryTokens = (currentQuery: string, additionalQuery: string) => {
  const currentTokens = tokenizeQuery(currentQuery);
  const currentTokenSet = new Set(
    currentTokens.map((token) => token.toLowerCase()),
  );

  const mergedTokens = [...currentTokens];
  tokenizeQuery(additionalQuery).forEach((token) => {
    const normalizedToken = token.toLowerCase();
    if (currentTokenSet.has(normalizedToken)) {
      return;
    }

    currentTokenSet.add(normalizedToken);
    mergedTokens.push(token);
  });

  return mergedTokens.join(" ");
};

const removeQueryTokens = (currentQuery: string, removableQuery: string) => {
  const removableTokenCounts = new Map<string, number>();
  tokenizeQuery(removableQuery).forEach((token) => {
    const normalizedToken = token.toLowerCase();
    removableTokenCounts.set(
      normalizedToken,
      (removableTokenCounts.get(normalizedToken) ?? 0) + 1,
    );
  });

  return tokenizeQuery(currentQuery)
    .filter((token) => {
      const normalizedToken = token.toLowerCase();
      const remainingCount = removableTokenCounts.get(normalizedToken) ?? 0;

      if (remainingCount <= 0) {
        return true;
      }

      removableTokenCounts.set(normalizedToken, remainingCount - 1);
      return false;
    })
    .join(" ");
};

const handlePinnedTabClick = async (pinnedId: string) => {
  const pinnedSearch = pinnedSearches.value.find(
    (item) => item.id === pinnedId,
  );

  if (!pinnedSearch) {
    return;
  }

  const nextQuery = activePinnedIds.value.includes(pinnedId)
    ? removeQueryTokens(searchQuery.value, pinnedSearch.query)
    : mergeQueryTokens(searchQuery.value, pinnedSearch.query);

  await handleSmartSearch(nextQuery);
};

const handleUnpinSearch = async (pinnedId: string) => {
  const previousQuery = searchQuery.value;
  unpinPinnedSearch(pinnedId);

  if (searchQuery.value !== previousQuery) {
    await handleSmartSearch(searchQuery.value);
  }
};

const handleWindowFocus = async () => {
  smartSearchRef.value?.focus();
};

const openSettings = async () => {
  try {
    await invoke("show_settings_window");
  } catch (err) {
    console.error("Failed to open settings:", err);
  }
};

let cleanupClipboard: (() => void) | null = null;
let unlistenPinMode: (() => void) | null = null;
let unlistenOpacity: (() => void) | null = null;
let unlistenFocus: (() => void) | null = null;
let unlistenBlur: (() => void) | null = null;

onMounted(async () => {
  cleanupClipboard = initClipboard();

  await loadHistory(settings.value.max_history_count, 0);
  loadSearchHistory();
  loadPinnedSearches();
  loadPinMode();
  window.addEventListener("keydown", handleKeyDown);

  if (filteredHistory.value.length > 0) {
    selectedIndex.value = 0;
  }

  const appWindow = getCurrentWindow();

  let blurDeactivateTimer: number | null = null;
  unlistenBlur = await appWindow.listen("tauri://blur", () => {
    savedSearchQuery.value = searchQuery.value;
    savedScrollPosition.value = scrollerRef.value?.$el?.scrollTop || 0;
    selectedIndex.value = -1;
    if (blurDeactivateTimer) clearTimeout(blurDeactivateTimer);
    blurDeactivateTimer = window.setTimeout(() => {
      hasActivated.value = false;
    }, 200);
  });

  unlistenFocus = await appWindow.listen("tauri://focus", () => {
    if (blurDeactivateTimer) {
      clearTimeout(blurDeactivateTimer);
      blurDeactivateTimer = null;
    }
    if (!hasActivated.value) {
      hasActivated.value = true;
      handleWindowFocus();
      if (savedSearchQuery.value) {
        searchQuery.value = savedSearchQuery.value;
      }
      nextTick(() => {
        if (filteredHistory.value.length === 0) {
          selectedIndex.value = -1;
          return;
        }

        const restoredIndex =
          selectedIndexFallback.value >= 0
            ? Math.min(
                selectedIndexFallback.value,
                filteredHistory.value.length - 1,
              )
            : 0;

        selectedIndex.value = restoredIndex;

        if (scrollerRef.value && savedScrollPosition.value > 0) {
          scrollerRef.value.$el.scrollTop = savedScrollPosition.value;
        }
      });
    }
  });

  unlistenPinMode = await listen<{ pinned: boolean }>(
    "pin-mode-changed",
    (event) => {
      isPinned.value = event.payload.pinned;
    },
  );

  unlistenOpacity = await listen<{ opacity: number }>(
    "window-opacity-change",
    (event) => {
      const opacity = event.payload.opacity;
      document.body.style.opacity = String(opacity);
    },
  );
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleKeyDown);
  cleanupScroll();
  unlistenFocus?.();
  unlistenBlur?.();
  unlistenPinMode?.();
  unlistenOpacity?.();
  cleanupClipboard?.();
  hasActivated.value = false;
});

watch(filteredHistory, () => {
  if (selectedIndex.value >= filteredHistory.value.length) {
    selectedIndex.value =
      filteredHistory.value.length > 0 ? filteredHistory.value.length - 1 : -1;
  } else if (selectedIndex.value < 0 && filteredHistory.value.length > 0) {
    const fallbackIndex =
      selectedIndexFallback.value >= 0
        ? Math.min(selectedIndexFallback.value, filteredHistory.value.length - 1)
        : 0;
    selectedIndex.value = fallbackIndex;
  }
});

watch(
  history,
  () => {
    if (!searchQuery.value) {
      resetDefaultList();
    }
  },
  { immediate: true },
);
</script>

<style scoped>
/* 搜索加载指示器 */
.search-loading {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 20px;
  color: #8c8c8c;
  font-size: 13px;
}

.search-loading .loading-spinner {
  width: 16px;
  height: 16px;
  animation: spin 1s linear infinite;
}

/* 加载更多动画 */
.loading-spinner-small {
  width: 14px;
  height: 14px;
  animation: spin 1s linear infinite;
}

/* 没有更多数据提示 */
.no-more-data {
  text-align: center;
  padding: 16px;
  color: #bfbfbf;
  font-size: 12px;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.clipboard-list {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: #fff;
}

.search-bar {
  padding: 10px 12px;
  background: #fff;
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.search-bar-top {
  border-bottom: 1px solid #f0f0f0;
}

.search-wrapper {
  display: flex;
  align-items: center;
  gap: 8px;
}

.icon-btn {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  color: #8c8c8c;
  transition: all 0.2s;
  flex-shrink: 0;
  padding: 0;
}

.icon-btn:hover {
  color: #1890ff;
  background: rgba(24, 144, 255, 0.08);
}

.icon-btn.is-active {
  color: #1890ff;
}

.icon-btn svg {
  width: 18px;
  height: 18px;
}

.list-container {
  flex: 1;
  overflow: hidden;
  background: #fff;
  position: relative;
}

.scroller {
  height: 100%;
  overflow-y: auto;
}

.scroller :deep(.vue-recycle-scroller__item-view) {
  /* 确保每个项目视图正确显示边框 */
  box-sizing: border-box;
}

.loading-more {
  text-align: center;
  padding: 12px;
  color: #8c8c8c;
  font-size: 12px;
}

/* 滚动条样式 */
::-webkit-scrollbar {
  width: 6px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: #c0c0c0;
  border-radius: 3px;
}

::-webkit-scrollbar-thumb:hover {
  background: #a0a0a0;
}
</style>
