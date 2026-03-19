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
          :title="isPinned ? '取消钉住' : `钉住面板 (${settings.pin_shortcut || 'Ctrl+Shift+P'})`"
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
        <svg class="loading-spinner" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10" stroke-dasharray="60" stroke-dashoffset="20"/>
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
            @contextmenu="(event: MouseEvent) => handleItemContextMenu(event, item, index)"
            @quick-action="handleQuickAction"
            @remove-tag="handleRemoveTag"
            @tag-click="handleTagClick"
          />
        </DynamicScrollerItem>
      </DynamicScroller>

      <!-- 加载更多指示器 -->
      <div v-if="isSearching && searchHasMore && filteredHistory.length > 0" class="loading-more">
        <svg class="loading-spinner-small" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10" stroke-dasharray="60" stroke-dashoffset="20"/>
        </svg>
        <span>加载更多...</span>
      </div>

      <!-- 没有更多数据提示 -->
      <div v-else-if="!searchHasMore && filteredHistory.length >= 200" class="no-more-data">
        没有更多数据了
      </div>
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
import { useClipboard } from "@/composables/useClipboard";
import { useSettings } from "@/composables/useSettings";
import { usePinMode } from "@/composables/usePinMode";
import {
  addSearchHistory,
  clearSearchHistory,
  getSearchHistory,
  matchItemWithQuery,
  parseSearchQuery,
  removeSearchHistory as removeSearchHistoryItem,
  type ParsedQuery,
} from "@/composables/useSmartSearch";
import type { ClipboardItem as ClipboardItemType, PinnedSearch } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { writeText } from "tauri-plugin-clipboard-x-api";
import { decodeHtmlEntities } from "@/utils/htmlUtils";
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from "vue";
import { DynamicScroller, DynamicScrollerItem } from "vue-virtual-scroller";
import { Pin as PinIcon, PinOff as PinOffIcon, BookmarkPlus } from "lucide-vue-next";
import ClipboardItem from "../ClipboardItem.vue";
import ContextMenu from "../ContextMenu.vue";
import DrawerEditor from "../DrawerEditor.vue";
import SmartSearch from "../SmartSearch.vue";
import TagManager from "../TagManager.vue";
import DeleteConfirmDialog from "./DeleteConfirmDialog.vue";
import EmptyState from "./EmptyState.vue";
import TabBar from "./TabBar.vue";

const {
  history,
  lastCopyTime,
  init: initClipboard,
  loadHistory,
  deleteItem,
  restoreToClipboard,
} = useClipboard();

const { settings } = useSettings();
const { isPinned, togglePinMode, loadPinMode } = usePinMode();

// 搜索相关
const smartSearchRef = ref<InstanceType<typeof SmartSearch> | null>(null);
const searchQuery = ref("");
const searchHistory = ref<string[]>([]);
const parsedQuery = computed<ParsedQuery>(() =>
  parseSearchQuery(searchQuery.value)
);

// 标签相关
const FIXED_TABS = [
  { key: "all", label: "全部" },
  { key: "text", label: "文本" },
  { key: "image", label: "图片" },
  { key: "file", label: "文件" },
];
const activeTab = ref("all");
const pinnedSearches = ref<PinnedSearch[]>([]);
const PINNED_SEARCH_STORAGE_KEY = "paste_library_pinned_searches";

// 列表状态
const loading = ref(false);
const hasMore = ref(true);
const offset = ref(0);
const limit = 50;
const selectedIndex = ref(-1);
const listContainerRef = ref<HTMLElement | null>(null);
const scrollerRef = ref<InstanceType<typeof DynamicScroller> | null>(null);

// 右键菜单状态
const contextMenuVisible = ref(false);
const contextMenuPosition = ref({ x: 0, y: 0 });
const contextMenuItem = ref<ClipboardItemType | null>(null);

// 删除确认状态
const deleteConfirmVisible = ref(false);
const itemToDelete = ref<ClipboardItemType | null>(null);

// Drawer 状态
const drawerVisible = ref(false);
const drawerItem = ref<ClipboardItemType | null>(null);

// 标签管理器状态
const tagManagerVisible = ref(false);
const tagManagerItem = ref<ClipboardItemType | null>(null);

// 激活标记
const hasActivated = ref(false);

// 粘贴高亮状态
const highlightedItemId = ref<number | null>(null);

// 状态保存（用于 Esc/失焦后恢复）
const savedSearchQuery = ref("");
const savedScrollPosition = ref(0);

// 加载搜索历史
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

// 固定搜索相关
const loadPinnedSearches = () => {
  try {
    const stored = localStorage.getItem(PINNED_SEARCH_STORAGE_KEY);
    if (stored) {
      pinnedSearches.value = JSON.parse(stored);
    }
  } catch {
    pinnedSearches.value = [];
  }
};

const savePinnedSearches = () => {
  try {
    localStorage.setItem(
      PINNED_SEARCH_STORAGE_KEY,
      JSON.stringify(pinnedSearches.value)
    );
  } catch {
    // 忽略存储错误
  }
};

const canPinCurrentSearch = computed(() => {
  const query = searchQuery.value.trim();
  if (!query) return false;
  return !pinnedSearches.value.some((ps) => ps.query === query);
});

const pinCurrentSearch = () => {
  const query = searchQuery.value.trim();
  if (!query) return;

  let label = query;
  if (query.length > 10) {
    label = query.slice(0, 10) + "...";
  }

  const newPinned: PinnedSearch = {
    id: `pinned_${Date.now()}`,
    label,
    query,
    created_at: Date.now(),
  };

  pinnedSearches.value.push(newPinned);
  savePinnedSearches();
  activeTab.value = newPinned.id;
};

const unpinSearch = (id: string) => {
  const index = pinnedSearches.value.findIndex((ps) => ps.id === id);
  if (index > -1) {
    pinnedSearches.value.splice(index, 1);
    savePinnedSearches();

    if (activeTab.value === id) {
      activeTab.value = "all";
      searchQuery.value = "";
    }
  }
};

const reorderPinnedSearches = (fromIndex: number, toIndex: number) => {
  const item = pinnedSearches.value.splice(fromIndex, 1)[0];
  pinnedSearches.value.splice(toIndex, 0, item);
  savePinnedSearches();
};

// 处理标签点击
watch(activeTab, (key) => {
  const pinned = pinnedSearches.value.find((ps) => ps.id === key);
  if (pinned) {
    // 切换到固定搜索标签时，使用该固定搜索的查询
    searchQuery.value = pinned.query;
  }
  // 切换到固定标签（all, text, image, file, folder）时，保留当前搜索查询
  // 不再自动清除搜索条件
});

// 智能激活
const handleSmartActivate = async () => {
  smartSearchRef.value?.focus();

  if (settings.value.smart_activate) {
    const timeDiff = Date.now() - lastCopyTime.value;

    if (timeDiff < 3000) {
      searchQuery.value = "";
      await handleSmartSearch("");

      if (listContainerRef.value) {
        listContainerRef.value.scrollTop = 0;
      }

      activeTab.value = "all";
    }
  }
};

// 异步搜索结果
const filteredHistory = ref<ClipboardItemType[]>([]);
const isSearching = ref(false);
const searchOffset = ref(0); // 搜索分页偏移量
const searchHasMore = ref(true); // 搜索是否还有更多数据
const SEARCH_PAGE_SIZE = 200; // 每页加载数量

// 防抖工具函数
function debounce<T extends (...args: Parameters<T>) => void>(
  fn: T,
  delay: number
): (...args: Parameters<T>) => void {
  let timer: number | null = null;
  return (...args: Parameters<T>) => {
    if (timer) clearTimeout(timer);
    timer = window.setTimeout(() => fn(...args), delay);
  };
}

/**
 * 执行异步搜索（调用 Rust 后端）
 * 支持标签过滤、类型过滤和关键词搜索
 * @param query 搜索查询
 * @param isLoadMore 是否为加载更多（累加数据）
 */
const performSearch = async (query: string, isLoadMore = false) => {
  const parsed = parseSearchQuery(query);

  // 如果不是加载更多，重置分页状态
  if (!isLoadMore) {
    searchOffset.value = 0;
    searchHasMore.value = true;
  }

  // 如果有钉住搜索，使用钉住的查询
  const pinnedSearch = pinnedSearches.value.find(
    (ps) => ps.id === activeTab.value
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
        types: pinnedParsed.types.map(t => {
          const typeMap: Record<string, string> = {
            'text': 'Text', 'html': 'Html', 'rtf': 'Rtf',
            'image': 'Image', 'file': 'File', 'folder': 'Folder', 'files': 'Files'
          };
          return typeMap[t] || 'Text';
        }),
        limit: SEARCH_PAGE_SIZE,
        offset: searchOffset.value
      };
    } else {
      searchRequest = { keywords: [], tags: [], types: [], limit: SEARCH_PAGE_SIZE, offset: 0 };
    }
  } else if (activeTab.value !== "all" && !parsed.isValid) {
    // Tab 过滤（非搜索状态）
    const typeMap: Record<string, string[]> = {
      'file': ['File', 'Folder', 'Files'],
      'text': ['Text', 'Html', 'Rtf'],
      'image': ['Image'],
    };
    const types = typeMap[activeTab.value] || [activeTab.value.charAt(0).toUpperCase() + activeTab.value.slice(1)];

    searchRequest = {
      keywords: [],
      tags: [],
      types: types,
      limit: SEARCH_PAGE_SIZE,
      offset: searchOffset.value
    };
  } else if (parsed.isValid) {
    // 普通搜索
    searchRequest = {
      keywords: parsed.keywords,
      tags: parsed.tags,
      types: parsed.types.map(t => {
        const typeMap: Record<string, string> = {
          'text': 'Text', 'html': 'Html', 'rtf': 'Rtf',
          'image': 'Image', 'file': 'File', 'folder': 'Folder', 'files': 'Files'
        };
        return typeMap[t] || 'Text';
      }),
      limit: SEARCH_PAGE_SIZE,
      offset: searchOffset.value
    };
  } else {
    // 无搜索条件，显示全部
    if (!isLoadMore) {
      filteredHistory.value = history.value.slice(0, SEARCH_PAGE_SIZE);
      searchHasMore.value = history.value.length > SEARCH_PAGE_SIZE;
    }
    return;
  }

  // 执行搜索
  isSearching.value = true;
  try {
    const results = await invoke<ClipboardItemType[]>("search_clipboard_advanced", {
      request: searchRequest
    });

    if (isLoadMore) {
      // 加载更多：追加数据
      filteredHistory.value = [...filteredHistory.value, ...results];
    } else {
      // 新搜索：替换数据
      filteredHistory.value = results;
    }

    // 更新分页状态
    searchOffset.value += SEARCH_PAGE_SIZE;
    searchHasMore.value = results.length === SEARCH_PAGE_SIZE;
  } finally {
    isSearching.value = false;
  }
};

/**
 * 加载更多搜索结果（滚动到底部触发）
 */
const loadMoreResults = async () => {
  if (isSearching.value) return;

  // 如果有搜索条件或标签过滤，调用后端加载更多
  if (searchQuery.value || activeTab.value !== 'all') {
    if (!searchHasMore.value) return;
    await performSearch(searchQuery.value, true);
  } else {
    // 全部模式：从历史记录中加载更多
    const currentLength = filteredHistory.value.length;
    const moreItems = history.value.slice(currentLength, currentLength + SEARCH_PAGE_SIZE);
    if (moreItems.length > 0) {
      filteredHistory.value = [...filteredHistory.value, ...moreItems];
    }
    // 更新是否有更多标志
    searchHasMore.value = currentLength + moreItems.length < history.value.length;
  }
};

// 防抖搜索（250ms 延迟）
const debouncedSearch = debounce(performSearch, 250);

// 搜索处理
const handleSmartSearch = async (query: string) => {
  searchQuery.value = query;

  // 立即执行搜索（不防抖）
  await performSearch(query);

  nextTick(() => {
    // 使用虚拟滚动器滚动到顶部
    if (scrollerRef.value) {
      scrollerRef.value.scrollToItem(0, "start");
    }
    if (filteredHistory.value.length > 0) {
      selectedIndex.value = 0;
    }
  });
};

const handleSearchCommit = (query: string) => {
  if (query.trim()) {
    addSearchHistory(query.trim());
    loadSearchHistory();
  }
};

// 标签点击
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

// 重置面板状态
const resetPanelState = () => {
  searchQuery.value = "";
  activeTab.value = "all";
  selectedIndex.value = -1;
  if (scrollerRef.value) {
    scrollerRef.value.scrollToItem(0, "start");
  }
};

// 显示粘贴成功反馈
const showPasteFeedback = (itemId: number) => {
  highlightedItemId.value = itemId;
  setTimeout(() => {
    if (highlightedItemId.value === itemId) {
      highlightedItemId.value = null;
    }
  }, 500);
};

// 项目点击处理
const handleItemClick = async (item: ClipboardItemType, index: number) => {
  selectedIndex.value = index;

  const clickAction = settings.value.click_action;
  if (clickAction === "none") {
    return;
  }

  const copyAsPlainText = settings.value.copy_as_plain_text;
  await restoreToClipboard(item, { copyAsPlainText });

  // 显示粘贴反馈（绿色高亮）
  showPasteFeedback(item.id);

  if (clickAction === "paste") {
    await simulatePaste(item.id);
  } else if (settings.value.hide_window_after_copy) {
    await invoke("hide_clipboard_window");
  }
};

const handleItemDoubleClick = async (
  item: ClipboardItemType,
  index: number
) => {
  selectedIndex.value = index;

  const doubleClickAction = settings.value.double_click_action;
  if (doubleClickAction === "none") {
    return;
  }

  const copyAsPlainText = settings.value.copy_as_plain_text;
  await restoreToClipboard(item, { copyAsPlainText });

  // 显示粘贴反馈（绿色高亮）
  showPasteFeedback(item.id);

  if (doubleClickAction === "paste") {
    await simulatePaste(item.id);
  } else if (settings.value.hide_window_after_copy) {
    await invoke("hide_clipboard_window");
  }
};

const handleItemContextMenu = (
  event: MouseEvent,
  item: ClipboardItemType,
  index: number
) => {
  contextMenuPosition.value = { x: event.clientX, y: event.clientY };
  contextMenuItem.value = item;
  contextMenuVisible.value = true;
  selectedIndex.value = index;
};

// 快捷操作
const handleQuickAction = async (action: string, item: ClipboardItemType) => {
  switch (action) {
    case "detail":
      drawerItem.value = item;
      drawerVisible.value = true;
      break;
    case "copy":
      await restoreToClipboard(item, {
        copyAsPlainText: settings.value.copy_as_plain_text,
      });
      break;
    case "delete":
      await handleDelete(item);
      break;
    case "tag":
      tagManagerItem.value = item;
      tagManagerVisible.value = true;
      break;
  }
};

// 删除处理
const handleDelete = async (item: ClipboardItemType) => {
  if (settings.value.confirm_delete) {
    itemToDelete.value = item;
    deleteConfirmVisible.value = true;
  } else {
    await deleteItem(item.id);
  }
};

const confirmDelete = async () => {
  if (itemToDelete.value) {
    await deleteItem(itemToDelete.value.id);
    itemToDelete.value = null;
  }
  deleteConfirmVisible.value = false;
};

const cancelDelete = () => {
  itemToDelete.value = null;
  deleteConfirmVisible.value = false;
};

// Drawer 处理
const handleDrawerCopy = async (item: ClipboardItemType) => {
  await restoreToClipboard(item, {
    copyAsPlainText: settings.value.copy_as_plain_text,
  });
  if (settings.value.hide_window_after_copy) {
    await invoke("hide_clipboard_window");
  }
};

const handleDrawerPaste = async (item: ClipboardItemType) => {
  await restoreToClipboard(item, {
    copyAsPlainText: settings.value.copy_as_plain_text,
  });
  await simulatePaste();
};

const handleSaveAsNew = async (content: string, type: string) => {
  try {
    if (type === "html") {
      await invoke("add_clipboard_item", {
        text: content.replace(/<[^>]*>/g, ""),
        html: content,
      });
    } else {
      await invoke("add_clipboard_item", { text: content, html: null });
    }
    await loadHistory();
  } catch (error) {
    console.error("Failed to save as new:", error);
  }
};

// 标签管理器处理
const handleTagManagerSave = async (itemId: number, tags: string[]) => {
  const item = history.value.find((h) => h.id === itemId);
  if (item) {
    item.tags = tags.length > 0 ? tags : undefined;
  }
};

const handleRemoveTag = async (item: ClipboardItemType, tag: string) => {
  try {
    const newTags = (item.tags || []).filter((t) => t !== tag);
    await invoke("update_tags", {
      id: item.id,
      tags: newTags.length > 0 ? newTags : null,
    });
    item.tags = newTags.length > 0 ? newTags : undefined;
  } catch (error) {
    console.error("Failed to remove tag:", error);
  }
};

// 复制文件路径
const copyFilePath = async (item: ClipboardItemType): Promise<void> => {
  try {
    let pathToCopy = "";

    if (item.file_paths && item.file_paths.length > 0) {
      pathToCopy = item.file_paths.join("\r\n") + "\r\n";
    } else if (item.content) {
      pathToCopy = item.content + "\r\n";
    }

    if (pathToCopy) {
      await writeText(pathToCopy);
    }
  } catch (error) {
    console.error("Failed to copy file path:", error);
  }
};

// 上下文菜单动作
const handleContextMenuAction = async (
  action: string,
  item: ClipboardItemType
) => {
  switch (action) {
    case "copy":
      await restoreToClipboard(item, {
        copyAsPlainText: settings.value.copy_as_plain_text,
      });
      // 显示粘贴反馈
      showPasteFeedback(item.id);
      if (settings.value.hide_window_after_copy) {
        await invoke("hide_clipboard_window");
      }
      break;
    case "paste":
      await restoreToClipboard(item, {
        copyAsPlainText: settings.value.copy_as_plain_text,
      });
      // 显示粘贴反馈
      showPasteFeedback(item.id);
      await simulatePaste(item.id);
      break;
    case "tag":
      tagManagerItem.value = item;
      tagManagerVisible.value = true;
      break;
    case "copyPlain":
      // 使用已存储的 text_content，或从 content 提取并解码 HTML 实体
      const plainContent = item.text_content || decodeHtmlEntities(item.content.replace(/<[^>]*>/g, ""));
      await restoreToClipboard({
        ...item,
        content_type: "text",
        content: plainContent,
      });
      if (settings.value.hide_window_after_copy) {
        await invoke("hide_clipboard_window");
      }
      break;
    case "pastePlain":
      // 使用已存储的 text_content，或从 content 提取并解码 HTML 实体
      const pastePlainContent = item.text_content || decodeHtmlEntities(item.content.replace(/<[^>]*>/g, ""));
      await restoreToClipboard({
        ...item,
        content_type: "text",
        content: pastePlainContent,
      });
      await simulatePaste();
      break;
    case "delete":
      await handleDelete(item);
      break;
    case "openFile":
      if (item.file_paths && item.file_paths.length > 0) {
        await invoke("open_file", { path: item.file_paths[0] });
      } else if (item.thumbnail_path) {
        await invoke("open_file", { path: item.thumbnail_path });
      }
      break;
    case "showInFolder":
      if (item.file_paths && item.file_paths.length > 0) {
        await invoke("show_in_folder", { path: item.file_paths[0] });
      } else if (item.thumbnail_path) {
        await invoke("show_in_folder", { path: item.thumbnail_path });
      }
      break;
    case "copyFilePath":
      await copyFilePath(item);
      break;
  }
};

// 模拟粘贴
const simulatePaste = async (itemId?: number): Promise<void> => {
  try {
    // 默认模式下：关闭面板并重置状态
    // 钉住模式下：保持面板开启，不重置状态
    if (!isPinned.value) {
      await invoke("hide_clipboard_window");
      // 重置面板状态
      resetPanelState();
    }

    await new Promise((resolve) => setTimeout(resolve, 100));
    await invoke("simulate_paste", {
      pasteShortcut: settings.value.paste_shortcut,
    });
  } catch (error) {
    console.log("Paste simulation not available yet");
  }
};

// 滚动处理 - 检测滚动到底部加载更多
let scrollThrottleTimer: number | null = null;

const handleScroll = () => {
  if (scrollThrottleTimer) return;

  scrollThrottleTimer = window.setTimeout(() => {
    scrollThrottleTimer = null;

    const scroller = scrollerRef.value?.$el;
    if (!scroller) return;

    const scrollTop = scroller.scrollTop;
    const scrollHeight = scroller.scrollHeight;
    const clientHeight = scroller.clientHeight;

    // 距离底部 200px 时触发加载更多
    if (scrollTop + clientHeight >= scrollHeight - 200) {
      loadMoreResults();
    }
  }, 100);
};

// 键盘导航
const handleKeyDown = async (e: KeyboardEvent) => {
  // Ctrl+F 聚焦搜索框
  if ((e.ctrlKey || e.metaKey) && e.key === "f") {
    e.preventDefault();
    smartSearchRef.value?.focus();
    return;
  }

  // 钉住模式快捷键
  const pinShortcut = settings.value.pin_shortcut || "Ctrl+Shift+P";
  const shortcutParts = pinShortcut.toLowerCase().split("+");
  const hasCtrl = shortcutParts.includes("ctrl");
  const hasShift = shortcutParts.includes("shift");
  const hasAlt = shortcutParts.includes("alt");
  const hasMeta = shortcutParts.includes("meta") || shortcutParts.includes("cmd");
  const keyPart = shortcutParts.find(p => !["ctrl", "shift", "alt", "meta", "cmd"].includes(p));

  if (
    (hasCtrl === e.ctrlKey) &&
    (hasShift === e.shiftKey) &&
    (hasAlt === e.altKey) &&
    (hasMeta === e.metaKey) &&
    e.key.toLowerCase() === (keyPart || "p")
  ) {
    e.preventDefault();
    await togglePinMode();
    return;
  }

  // Esc 关闭窗口（保存当前状态）
  if (e.key === "Escape") {
    e.preventDefault();
    // 保存当前状态
    savedSearchQuery.value = searchQuery.value;
    savedScrollPosition.value = scrollerRef.value?.$el?.scrollTop || 0;
    selectedIndex.value = -1; // 清除高亮
    invoke("hide_clipboard_window");
    return;
  }

  // 上下导航
  if (e.key === "ArrowUp" || e.key === "ArrowDown") {
    e.preventDefault();
    const direction = e.key === "ArrowUp" ? -1 : 1;
    const newIndex = selectedIndex.value + direction;

    if (newIndex >= 0 && newIndex < filteredHistory.value.length) {
      selectedIndex.value = newIndex;
      scrollSelectedItemIntoView();
    }
    return;
  }

  // Enter 粘贴选中项
  if (e.key === "Enter" && selectedIndex.value >= 0) {
    e.preventDefault();
    const item = filteredHistory.value[selectedIndex.value];
    if (item) {
      await restoreToClipboard(item, {
        copyAsPlainText: settings.value.copy_as_plain_text,
      });
      // 显示粘贴反馈
      showPasteFeedback(item.id);
      await simulatePaste(item.id);
    }
    return;
  }

  // 数字键 1-9 快速粘贴
  if (e.key >= "1" && e.key <= "9") {
    const shortcut = settings.value.number_key_shortcut || "ctrl";
    let shouldTrigger = false;

    if (shortcut === "none") {
      shouldTrigger = !e.ctrlKey && !e.altKey && !e.shiftKey && !e.metaKey;
    } else {
      const requiredModifiers = shortcut
        .toLowerCase()
        .split("+")
        .map((s) => s.trim());
      const actualModifiers: string[] = [];
      if (e.ctrlKey) actualModifiers.push("ctrl");
      if (e.altKey) actualModifiers.push("alt");
      if (e.shiftKey) actualModifiers.push("shift");
      if (e.metaKey) actualModifiers.push("meta");

      const allRequiredPressed = requiredModifiers.every((mod) =>
        actualModifiers.includes(mod)
      );
      const noExtraModifiers = actualModifiers.every((mod) =>
        requiredModifiers.includes(mod)
      );
      shouldTrigger = allRequiredPressed && noExtraModifiers;
    }

    if (shouldTrigger) {
      const index = parseInt(e.key) - 1;
      if (index < filteredHistory.value.length) {
        e.preventDefault();
        const item = filteredHistory.value[index];
        if (item) {
          await restoreToClipboard(item, {
            copyAsPlainText: settings.value.copy_as_plain_text,
          });
          // 显示粘贴反馈
          showPasteFeedback(item.id);
          await simulatePaste(item.id);
        }
      }
    }
    return;
  }
};

// 滚动选中项到可视区域 - 使用虚拟滚动器的 scrollToItem
const scrollSelectedItemIntoView = () => {
  if (selectedIndex.value < 0 || !scrollerRef.value) return;

  scrollerRef.value.scrollToItem(selectedIndex.value, "center");
};

// 生命周期
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
      handleSmartActivate();
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
    selectedIndex.value = -1; // 清除高亮
  });

  // 监听钉住模式变化
  unlistenPinMode = await listen<{ pinned: boolean }>("pin-mode-changed", (event) => {
    isPinned.value = event.payload.pinned;
  });

  // 监听窗口透明度变化
  unlistenOpacity = await listen<{ opacity: number }>("window-opacity-change", (event) => {
    const opacity = event.payload.opacity;
    document.body.style.opacity = String(opacity);
  });

  (window as any).__unlistenFocus = unlistenFocus;
  (window as any).__unlistenBlur = unlistenBlur;
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleKeyDown);
  // 清理滚动节流 timer
  if (scrollThrottleTimer) {
    clearTimeout(scrollThrottleTimer);
  }
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
  // 清理剪贴板监听
  if (cleanupClipboard) {
    cleanupClipboard();
  }
  hasActivated.value = false;
});

// 监听过滤变化，重置选中状态
watch(filteredHistory, () => {
  if (selectedIndex.value >= filteredHistory.value.length) {
    selectedIndex.value = filteredHistory.value.length > 0 ? 0 : -1;
  } else if (selectedIndex.value < 0 && filteredHistory.value.length > 0) {
    selectedIndex.value = 0;
  }
});

// 监听历史记录变化，初始化时加载
watch(history, () => {
  if (!searchQuery.value && activeTab.value === 'all') {
    // 初始状态，显示全部
    filteredHistory.value = history.value.slice(0, SEARCH_PAGE_SIZE);
    searchHasMore.value = history.value.length > SEARCH_PAGE_SIZE;
  }
}, { immediate: true });

// 监听标签切换，重新搜索
watch(activeTab, () => {
  performSearch(searchQuery.value);
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
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
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
