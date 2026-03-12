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
          <svg
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path
              d="M12 2a2 2 0 0 0-2 2v3.586l-3.707 3.707A1 1 0 0 0 6 12v2a1 1 0 0 0 1 1h3l2 7 2-7h3a1 1 0 0 0 1-1v-2a1 1 0 0 0-.293-.707L14 7.586V4a2 2 0 0 0-2-2z"
            />
          </svg>
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
      <EmptyState
        v-if="filteredHistory.length === 0"
        :has-search-query="!!searchQuery"
      />

      <DynamicScroller
        v-else
        ref="scrollerRef"
        class="scroller"
        :items="filteredHistory"
        :min-item-size="60"
        key-field="id"
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
            @contextmenu="(event: MouseEvent) => handleItemContextMenu(event, item, index)"
            @quick-action="handleQuickAction"
            @remove-tag="handleRemoveTag"
            @tag-click="handleTagClick"
          />
        </DynamicScrollerItem>
      </DynamicScroller>

      <div v-if="loading" class="loading-more">加载中...</div>
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
import { getCurrentWindow } from "@tauri-apps/api/window";
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
  lastCopyTime,
  init: initClipboard,
  loadHistory,
  deleteItem,
  restoreToClipboard,
} = useClipboard();

const { settings } = useSettings();

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
    searchQuery.value = pinned.query;
  } else if (key !== "all") {
    searchQuery.value = "";
  } else {
    searchQuery.value = "";
  }
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

// 模糊匹配
const fuzzyMatch = (query: string, text: string): boolean => {
  const queryLower = query.toLowerCase();
  const textLower = text.toLowerCase();
  return textLower.includes(queryLower);
};

// 过滤后的历史记录
const filteredHistory = computed(() => {
  let result = history.value;

  const pinnedSearch = pinnedSearches.value.find(
    (ps) => ps.id === activeTab.value
  );
  if (pinnedSearch) {
    const pinnedQuery = parseSearchQuery(pinnedSearch.query);
    if (pinnedQuery.isValid) {
      result = result.filter((item) =>
        matchItemWithQuery(item, pinnedQuery, fuzzyMatch)
      );
    }
  } else if (activeTab.value !== "all") {
    if (activeTab.value === "file") {
      result = result.filter(
        (item) =>
          item.content_type === "file" ||
          item.content_type === "files" ||
          item.content_type === "folder"
      );
    } else if (activeTab.value === "text") {
      result = result.filter(
        (item) =>
          item.content_type === "text" ||
          item.content_type === "html" ||
          item.content_type === "rtf"
      );
    } else {
      result = result.filter((item) => item.content_type === activeTab.value);
    }
  }

  if (!pinnedSearch && parsedQuery.value.isValid) {
    result = result.filter((item) =>
      matchItemWithQuery(item, parsedQuery.value, fuzzyMatch)
    );
  }

  return result;
});

// 搜索处理
const handleSmartSearch = async (query: string) => {
  searchQuery.value = query;

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

// 项目点击处理
const handleItemClick = async (item: ClipboardItemType, index: number) => {
  selectedIndex.value = index;

  const clickAction = settings.value.click_action;
  if (clickAction === "none") {
    return;
  }

  const copyAsPlainText = settings.value.copy_as_plain_text;
  await restoreToClipboard(item, { copyAsPlainText });

  if (clickAction === "paste") {
    await simulatePaste();
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

  if (doubleClickAction === "paste") {
    await simulatePaste();
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
      if (settings.value.hide_window_after_copy) {
        await invoke("hide_clipboard_window");
      }
      break;
    case "paste":
      await restoreToClipboard(item, {
        copyAsPlainText: settings.value.copy_as_plain_text,
      });
      await simulatePaste();
      break;
    case "tag":
      tagManagerItem.value = item;
      tagManagerVisible.value = true;
      break;
    case "copyPlain":
      await restoreToClipboard({
        ...item,
        content_type: "text",
        content: item.content.replace(/<[^>]*>/g, ""),
      });
      if (settings.value.hide_window_after_copy) {
        await invoke("hide_clipboard_window");
      }
      break;
    case "pastePlain":
      await restoreToClipboard({
        ...item,
        content_type: "text",
        content: item.content.replace(/<[^>]*>/g, ""),
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
const simulatePaste = async (): Promise<void> => {
  try {
    await invoke("hide_clipboard_window");
    await new Promise((resolve) => setTimeout(resolve, 100));
    await invoke("simulate_paste", {
      pasteShortcut: settings.value.paste_shortcut,
    });
  } catch (error) {
    console.log("Paste simulation not available yet");
  }
};

// 滚动处理 - 虚拟滚动器自动处理，保留加载更多逻辑
const handleScroll = async () => {
  // 虚拟滚动器自动处理滚动，这里保留加载更多逻辑如果需要的话
};

// 键盘导航
const handleKeyDown = async (e: KeyboardEvent) => {
  // Ctrl+F 聚焦搜索框
  if ((e.ctrlKey || e.metaKey) && e.key === "f") {
    e.preventDefault();
    smartSearchRef.value?.focus();
    return;
  }

  // Esc 关闭窗口
  if (e.key === "Escape") {
    e.preventDefault();
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
      await simulatePaste();
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
          await simulatePaste();
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

onMounted(async () => {
  // 初始化剪贴板监听
  cleanupClipboard = initClipboard();

  loadHistory(settings.value.max_history_count, 0);
  loadSearchHistory();
  loadPinnedSearches();
  window.addEventListener("keydown", handleKeyDown);

  if (filteredHistory.value.length > 0) {
    selectedIndex.value = 0;
  }

  const appWindow = getCurrentWindow();
  const unlistenFocus = await appWindow.listen("tauri://focus", () => {
    if (!hasActivated.value) {
      hasActivated.value = true;
      handleSmartActivate();
    }
  });

  const unlistenBlur = await appWindow.listen("tauri://blur", () => {
    hasActivated.value = false;
  });

  (window as any).__unlistenFocus = unlistenFocus;
  (window as any).__unlistenBlur = unlistenBlur;
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleKeyDown);
  if ((window as any).__unlistenFocus) {
    (window as any).__unlistenFocus();
  }
  if ((window as any).__unlistenBlur) {
    (window as any).__unlistenBlur();
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
</script>

<style scoped>
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
  width: 16px;
  height: 16px;
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
