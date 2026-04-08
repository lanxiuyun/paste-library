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
          class="pin-btn"
          title="固定当前搜索"
          @click="pinCurrentSearch"
        >
          <BookmarkPlus :size="18" />
        </button>
        <!-- 钉住模式按钮 -->
        <button
          class="pin-mode-btn"
          :class="{ 'is-pinned': isPinned }"
          :title="
            isPinned
              ? '取消钉住'
              : `钉住面板 (${settings.pin_shortcut || 'Ctrl+Shift+P'})`
          "
          @click="togglePinMode"
        >
          <!-- 使用 Lucide 图标 -->
          <PinIcon :size="18" />
        </button>
      </div>
    </div>

    <!-- 标签栏 -->
    <TabBar
      v-model="activeTab"
      :fixed-tabs="FIXED_TABS"
      :pinned-searches="pinnedSearches"
      @unpin="unpinSearch"
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
            :is-highlighted="highlightedItemId === item.id"
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
// ============================================
// Imports
// ============================================
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
  removeSearchHistory as removeSearchHistoryItem,
} from "@/composables/useSmartSearch";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { BookmarkPlus, Pin as PinIcon } from "lucide-vue-next";
import { nextTick, onMounted, onUnmounted, ref, watch } from "vue";
import { DynamicScroller, DynamicScrollerItem } from "vue-virtual-scroller";
import ClipboardItem from "../ClipboardItem.vue";
import ContextMenu from "../ContextMenu.vue";
import DrawerEditor from "../DrawerEditor.vue";
import SmartSearch from "../SmartSearch.vue";
import TagManager from "../TagManager.vue";
import DeleteConfirmDialog from "./DeleteConfirmDialog.vue";
import EmptyState from "./EmptyState.vue";
import TabBar from "./TabBar.vue";

// ============================================
// Constants
// ============================================
const FIXED_TABS = [
  { key: "all", label: "全部" },
  { key: "text", label: "文本" },
  { key: "image", label: "图片" },
  { key: "file", label: "文件" },
];

// ============================================
// Core Composables
// ============================================
const {
  history,
  init: initClipboard,
  loadHistory,
  deleteItem,
  restoreToClipboard,
} = useClipboard();

const { settings } = useSettings();
const { isPinned, togglePinMode, loadPinMode } = usePinMode();

// ============================================
// Refs & State
// ============================================
const smartSearchRef = ref<InstanceType<typeof SmartSearch> | null>(null);
const scrollerRef = ref<InstanceType<typeof DynamicScroller> | null>(null);
const searchQuery = ref("");
const searchHistory = ref<string[]>([]);
const activeTab = ref("all");
const hasActivated = ref(false);
const savedSearchQuery = ref("");
const savedScrollPosition = ref(0);

// ============================================
// Pinned Searches
// ============================================
const {
  pinnedSearches,
  canPinCurrentSearch,
  loadPinnedSearches,
  pinCurrentSearch,
  unpinSearch,
  reorderPinnedSearches,
} = usePinnedSearches(activeTab, searchQuery);

// ============================================
// Search
// ============================================
const scrollerForSearch = ref<{
  scrollToItem: (index: number, position: string) => void;
} | null>(null);
watch(scrollerRef, (val) => {
  if (val) {
    scrollerForSearch.value = {
      scrollToItem: (index: number, position: string) => {
        val.scrollToItem(index, position as any);
      },
    };
  }
});

const onSearchComplete = (resultCount: number) => {
  if (resultCount > 0 && selectedIndex.value < 0) {
    selectedIndex.value = 0;
  }
};

const {
  filteredHistory,
  isSearching,
  searchHasMore,
  parsedQuery,
  performSearch,
  loadMoreResults,
  handleSmartSearch: _handleSmartSearch,
} = useSearch({
  activeTab,
  searchQuery,
  pinnedSearches,
  history,
  scrollerRef: scrollerForSearch,
  onSearchComplete,
});

// 搜索处理包装器
const handleSmartSearch = async (query: string, shouldScrollToTop = true) => {
  searchQuery.value = query;
  await _handleSmartSearch(shouldScrollToTop);
};

// ============================================
// Clipboard List Core
// ============================================
const resetPanelState = () => {
  searchQuery.value = "";
  activeTab.value = "all";
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
  handleDelete,
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

// 解构 UI 状态
const {
  contextMenuVisible,
  contextMenuPosition,
  contextMenuItem,
  drawerVisible,
  drawerItem,
  tagManagerVisible,
  tagManagerItem,
  deleteConfirmVisible,
  itemToDelete,
  highlightedItemId,
  selectedIndex,
} = uiState;

// ============================================
// Keyboard Navigation
// ============================================
const smartSearchForKeyboard = ref<{ focus: () => void } | null>(null);
watch(smartSearchRef, (val) => {
  if (val) {
    smartSearchForKeyboard.value = {
      focus: () => val.focus(),
    };
  }
});

const scrollerForKeyboard = ref<{
  scrollToItem: (index: number, position: string) => void;
} | null>(null);
watch(scrollerRef, (val) => {
  if (val) {
    scrollerForKeyboard.value = {
      scrollToItem: (index: number, position: string) => {
        val.scrollToItem(index, position as any);
      },
    };
  }
});

const handleEscape = () => {
  // 保存当前状态
  savedSearchQuery.value = searchQuery.value;
  savedScrollPosition.value = scrollerRef.value?.$el?.scrollTop || 0;
  selectedIndex.value = -1;
  invoke("hide_clipboard_window");
};

const handleTogglePinMode = async () => {
  await togglePinMode();
};

const handleClearSearch = async () => {
  searchQuery.value = "";
  await _handleSmartSearch(true);
};

const { handleKeyDown } = useKeyboardNavigation({
  filteredHistory,
  selectedIndex,
  searchQuery,
  settings,
  isPinned,
  smartSearchRef: smartSearchForKeyboard,
  scrollerRef: scrollerForKeyboard,
  executeClipboardAction,
  onEscape: handleEscape,
  onTogglePinMode: handleTogglePinMode,
  resetPanelState,
  handleSmartSearch: handleClearSearch,
});

// ============================================
// Scroll Handler
// ============================================
const { handleScroll, cleanup: cleanupScroll } = useScrollHandler({
  isSearching,
  searchHasMore,
  loadMoreResults,
});

// ============================================
// Search History
// ============================================
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

// ============================================
// Tag Click
// ============================================
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

// ============================================
// Window Focus
// ============================================
const handleWindowFocus = async () => {
  smartSearchRef.value?.focus();
};

// ============================================
// Lifecycle
// ============================================
let cleanupClipboard: (() => void) | null = null;
let unlistenPinMode: (() => void) | null = null;
let unlistenOpacity: (() => void) | null = null;

onMounted(async () => {
  // 初始化剪贴板监听
  cleanupClipboard = initClipboard();

  loadHistory(settings.value.max_history_count, 0);
  loadSearchHistory();
  loadPinnedSearches();
  loadPinMode();
  window.addEventListener("keydown", handleKeyDown);

  if (filteredHistory.value.length > 0) {
    selectedIndex.value = 0;
  }

  const appWindow = getCurrentWindow();
  const unlistenFocus = await appWindow.listen("tauri://focus", () => {
    if (!hasActivated.value) {
      hasActivated.value = true;
      handleWindowFocus();
      // 恢复保存的状态
      if (savedSearchQuery.value) {
        searchQuery.value = savedSearchQuery.value;
        nextTick(() => {
          if (scrollerRef.value && savedScrollPosition.value > 0) {
            scrollerRef.value.$el.scrollTop = savedScrollPosition.value;
          }
        });
      }
    }
  });

  const unlistenBlur = await appWindow.listen("tauri://blur", () => {
    hasActivated.value = false;
    // 保存当前状态
    savedSearchQuery.value = searchQuery.value;
    savedScrollPosition.value = scrollerRef.value?.$el?.scrollTop || 0;
    selectedIndex.value = -1;
  });

  // 监听钉住模式变化
  unlistenPinMode = await listen<{ pinned: boolean }>(
    "pin-mode-changed",
    (event) => {
      isPinned.value = event.payload.pinned;
    }
  );

  // 监听窗口透明度变化
  unlistenOpacity = await listen<{ opacity: number }>(
    "window-opacity-change",
    (event) => {
      const opacity = event.payload.opacity;
      document.body.style.opacity = String(opacity);
    }
  );

  (window as any).__unlistenFocus = unlistenFocus;
  (window as any).__unlistenBlur = unlistenBlur;
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleKeyDown);
  cleanupScroll();
  if ((window as any).__unlistenFocus) {
    (window as any).__unlistenFocus();
  }
  if ((window as any).__unlistenBlur) {
    (window as any).__unlistenBlur();
  }
  if (unlistenPinMode) {
    unlistenPinMode();
  }
  if (unlistenOpacity) {
    unlistenOpacity();
  }
  if (cleanupClipboard) {
    cleanupClipboard();
  }
  hasActivated.value = false;
});

// ============================================
// Watchers
// ============================================
// 监听过滤变化，重置选中状态
watch(filteredHistory, () => {
  if (selectedIndex.value >= filteredHistory.value.length) {
    selectedIndex.value = filteredHistory.value.length > 0 ? 0 : -1;
  } else if (selectedIndex.value < 0 && filteredHistory.value.length > 0) {
    selectedIndex.value = 0;
  }
});

// 监听历史记录变化，初始化时加载
watch(
  history,
  () => {
    if (!searchQuery.value && activeTab.value === "all") {
      // 初始状态，显示全部
      filteredHistory.value = history.value.slice(0, 50);
    }
  },
  { immediate: true }
);

// 监听标签切换，重新搜索
watch(activeTab, () => {
  performSearch();
});
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

.pin-mode-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f5f5f5;
  border: 1px solid transparent;
  border-radius: 8px;
  cursor: pointer;
  color: #8c8c8c;
  transition: all 0.2s;
  flex-shrink: 0;
  position: relative;
}

.pin-mode-btn:hover {
  background: #e6f7ff;
  border-color: #91d5ff;
  color: #1890ff;
}

.pin-mode-btn.is-pinned {
  background: #1890ff;
  border-color: #1890ff;
  color: #fff;
}

.pin-mode-btn svg {
  width: 18px;
  height: 18px;
}

.pin-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f5f5f5;
  border: 1px solid transparent;
  border-radius: 8px;
  cursor: pointer;
  color: #8c8c8c;
  transition: all 0.2s;
  flex-shrink: 0;
}

.pin-btn:hover {
  background: #e6f7ff;
  border-color: #91d5ff;
  color: #1890ff;
}

.pin-btn svg {
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

/* 虚拟滚动器项目容器样式 - 确保分割线正确显示 */
.scroller :deep(.vue-recycle-scroller__item-wrapper) {
  /* 虚拟滚动器内部包装器 */
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
