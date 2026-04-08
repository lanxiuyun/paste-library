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
import { useClipboard } from "@/composables/useClipboard";
import { usePinMode } from "@/composables/usePinMode";
import { useSettings } from "@/composables/useSettings";
import {
  addSearchHistory,
  clearSearchHistory,
  getSearchHistory,
  parseSearchQuery,
  removeSearchHistory as removeSearchHistoryItem,
  type ParsedQuery,
} from "@/composables/useSmartSearch";
import type { ClipboardItem as ClipboardItemType, PinnedSearch } from "@/types";
import { decodeHtmlEntities } from "@/utils/htmlUtils";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { BookmarkPlus, Pin as PinIcon } from "lucide-vue-next";
import { writeText } from "tauri-plugin-clipboard-x-api";
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from "vue";
import { DynamicScroller, DynamicScrollerItem } from "vue-virtual-scroller";
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
  parseSearchQuery(searchQuery.value),
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
const selectedIndex = ref(-1);
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
      JSON.stringify(pinnedSearches.value),
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

// 窗口聚焦时聚焦搜索框
const handleWindowFocus = async () => {
  smartSearchRef.value?.focus();
};

// 异步搜索结果
const filteredHistory = ref<ClipboardItemType[]>([]);
const isSearching = ref(false);
const searchOffset = ref(0); // 搜索分页偏移量（Rust内部每次扫描200条）
const searchHasMore = ref(true); // 搜索是否还有更多数据
const ITEMS_PER_PAGE = 50; // 每次返回给前端的数量

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
        // 后端使用 snake_case 序列化，所以发送小写的类型名
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
    // 后端使用 snake_case 序列化，所以发送小写的类型名
    const typeMap: Record<string, string[]> = {
      file: ["file", "folder", "files"],
      text: ["text", "html", "rtf"],
      image: ["image"],
    };
    const types = typeMap[activeTab.value] || [activeTab.value.toLowerCase()];

    searchRequest = {
      keywords: [],
      tags: [],
      types: types,
      limit: ITEMS_PER_PAGE,
      offset: searchOffset.value,
    };
  } else if (parsed.isValid) {
    // 普通搜索
    searchRequest = {
      keywords: parsed.keywords,
      tags: parsed.tags,
      // 后端使用 snake_case 序列化，所以发送小写的类型名
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
    }
    return;
  }

  // 执行搜索：只有新搜索才显示加载状态，加载更多时不显示
  // 避免 DynamicScroller 被销毁重建导致滚动位置丢失
  if (!isLoadMore) {
    isSearching.value = true;
  }
  try {
    const results = await invoke<ClipboardItemType[]>(
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
  if (searchQuery.value || activeTab.value !== "all") {
    if (!searchHasMore.value) return;
    await performSearch(searchQuery.value, true);
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

// 搜索处理
const handleSmartSearch = async (query: string, shouldScrollToTop = true) => {
  searchQuery.value = query;

  // 立即执行搜索（不防抖）
  await performSearch(query);

  nextTick(() => {
    // 只有需要时才滚动到顶部（新搜索时滚动，加载更多时不滚动）
    if (shouldScrollToTop && scrollerRef.value) {
      scrollerRef.value.scrollToItem(0, "start");
    }
    if (filteredHistory.value.length > 0 && selectedIndex.value < 0) {
      selectedIndex.value = 0;
    }
  });
};

const handleSearchCommit = (query: string) => {
  if (query.trim()) {
    addSearchHistory(query.trim());
    loadSearchHistory();
  }
  // 触发搜索（即使是空查询也要执行，用于显示所有数据）
  handleSmartSearch(query);
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
  savedSearchQuery.value = "";
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

// 剪贴板操作类型
type ClipboardAction = "copy" | "paste";

// 统一的剪贴板操作入口
const executeClipboardAction = async (
  item: ClipboardItemType,
  action: ClipboardAction,
  options: {
    copyAsPlainText?: boolean;
    showFeedback?: boolean;
    hideWindow?: boolean;
  } = {},
) => {
  const {
    copyAsPlainText = settings.value.copy_as_plain_text,
    showFeedback = true,
    hideWindow = settings.value.hide_window_after_copy,
  } = options;

  // 1. 恢复到剪贴板（复制）
  await restoreToClipboard(item, { copyAsPlainText });

  // 2. 显示视觉反馈（可选）
  if (showFeedback) {
    showPasteFeedback(item.id);
  }

  // 不是Pin 模式下，执行了复制/粘贴操作后，自动隐藏
  if (!isPinned.value && (action === "paste" || hideWindow)) {
    await invoke("hide_clipboard_window");
    resetPanelState();
  }

  // 3. 粘贴必须放在最后执行，确保焦点已回到目标输入框
  if (action === "paste") {
    await simulatePaste();
  }
};

// 项目点击/双击通用处理
const handleItemAction = async (
  item: ClipboardItemType,
  index: number,
  actionType: "click" | "doubleClick",
) => {
  selectedIndex.value = index;

  const action =
    actionType === "click"
      ? settings.value.click_action
      : settings.value.double_click_action;

  if (action === "none") {
    return;
  }

  await executeClipboardAction(item, action);
};

// 单击处理
const handleItemClick = (item: ClipboardItemType, index: number) => {
  handleItemAction(item, index, "click");
};

// 双击处理
const handleItemDoubleClick = (item: ClipboardItemType, index: number) => {
  handleItemAction(item, index, "doubleClick");
};

const handleItemContextMenu = (
  event: MouseEvent,
  item: ClipboardItemType,
  index: number,
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

// Drawer 处理（注：Drawer 场景也启用粘贴反馈，让用户明确知道操作成功）
const handleDrawerCopy = async (item: ClipboardItemType) => {
  await executeClipboardAction(item, "copy");
};

const handleDrawerPaste = async (item: ClipboardItemType) => {
  await executeClipboardAction(item, "paste");
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
  item: ClipboardItemType,
) => {
  switch (action) {
    case "copy":
      await executeClipboardAction(item, "copy");
      break;
    case "paste":
      await executeClipboardAction(item, "paste");
      break;
    case "tag":
      tagManagerItem.value = item;
      tagManagerVisible.value = true;
      break;
    case "copyPlain": {
      // 构造纯文本 item，然后走统一入口
      const plainContent =
        item.text_content ||
        decodeHtmlEntities(item.content.replace(/<[^>]*>/g, ""));
      const plainItem: ClipboardItemType = {
        ...item,
        content_type: "text",
        content: plainContent,
      };
      await executeClipboardAction(plainItem, "copy", { copyAsPlainText: true });
      break;
    }
    case "pastePlain": {
      // 构造纯文本 item，然后走统一入口
      const pastePlainContent =
        item.text_content ||
        decodeHtmlEntities(item.content.replace(/<[^>]*>/g, ""));
      const plainItem: ClipboardItemType = {
        ...item,
        content_type: "text",
        content: pastePlainContent,
      };
      await executeClipboardAction(plainItem, "paste", { copyAsPlainText: true });
      break;
    }
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
const simulatePaste = async (): Promise<void> => {
  try {
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

// 快捷键匹配辅助函数：检查键盘事件是否匹配指定的快捷键配置
const matchesShortcut = (
  e: KeyboardEvent,
  shortcutConfig: string,
): boolean => {
  if (shortcutConfig === "none") {
    return !e.ctrlKey && !e.altKey && !e.shiftKey && !e.metaKey;
  }

  const requiredModifiers = shortcutConfig
    .toLowerCase()
    .split("+")
    .map((s) => s.trim());

  const actualModifiers: string[] = [];
  if (e.ctrlKey) actualModifiers.push("ctrl");
  if (e.altKey) actualModifiers.push("alt");
  if (e.shiftKey) actualModifiers.push("shift");
  if (e.metaKey) actualModifiers.push("meta");

  const allRequiredPressed = requiredModifiers.every((mod) =>
    actualModifiers.includes(mod),
  );
  const noExtraModifiers = actualModifiers.every((mod) =>
    requiredModifiers.includes(mod),
  );

  return allRequiredPressed && noExtraModifiers;
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
  const hasMeta =
    shortcutParts.includes("meta") || shortcutParts.includes("cmd");
  const keyPart = shortcutParts.find(
    (p) => !["ctrl", "shift", "alt", "meta", "cmd"].includes(p),
  );

  if (
    hasCtrl === e.ctrlKey &&
    hasShift === e.shiftKey &&
    hasAlt === e.altKey &&
    hasMeta === e.metaKey &&
    e.key.toLowerCase() === (keyPart || "p")
  ) {
    e.preventDefault();
    await togglePinMode();
    return;
  }

  // Esc 处理：先检查搜索框状态
  if (e.key === "Escape") {
    // 如果搜索框有内容，先清空搜索框（不隐藏窗口）
    const query = searchQuery.value.trim();
    if (query) {
      e.preventDefault();
      searchQuery.value = "";
      handleSmartSearch(""); // 触发搜索以显示所有历史
      return;
    }

    // 搜索框已为空，隐藏窗口
    e.preventDefault();
    // 保存当前状态
    savedSearchQuery.value = searchQuery.value;
    savedScrollPosition.value = scrollerRef.value?.$el?.scrollTop || 0;
    selectedIndex.value = -1; // 清除高亮
    await invoke("hide_clipboard_window");
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
      await executeClipboardAction(item, "paste");
    }
    return;
  }

  // 数字键 1-9 快速粘贴
  if (e.key >= "1" && e.key <= "9") {
    const shortcut = settings.value.number_key_shortcut || "ctrl";

    if (matchesShortcut(e, shortcut)) {
      const index = parseInt(e.key) - 1;
      if (index < filteredHistory.value.length) {
        e.preventDefault();
        const item = filteredHistory.value[index];
        if (item) {
          await executeClipboardAction(item, "paste");
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
    selectedIndex.value = -1; // 清除高亮
  });

  // 监听钉住模式变化
  unlistenPinMode = await listen<{ pinned: boolean }>(
    "pin-mode-changed",
    (event) => {
      isPinned.value = event.payload.pinned;
    },
  );

  // 监听窗口透明度变化
  unlistenOpacity = await listen<{ opacity: number }>(
    "window-opacity-change",
    (event) => {
      const opacity = event.payload.opacity;
      document.body.style.opacity = String(opacity);
    },
  );

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
watch(
  history,
  () => {
    if (!searchQuery.value && activeTab.value === "all") {
      // 初始状态，显示全部
      filteredHistory.value = history.value.slice(0, ITEMS_PER_PAGE);
      searchHasMore.value = history.value.length > ITEMS_PER_PAGE;
    }
  },
  { immediate: true },
);

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
