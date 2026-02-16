<template>
  <div class="clipboard-list">
    <!-- 搜索栏 - 顶部位置 -->
    <div v-if="settings.search_position === 'top'" class="search-bar search-bar-top">
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
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2a2 2 0 0 0-2 2v3.586l-3.707 3.707A1 1 0 0 0 6 12v2a1 1 0 0 0 1 1h3l2 7 2-7h3a1 1 0 0 0 1-1v-2a1 1 0 0 0-.293-.707L14 7.586V4a2 2 0 0 0-2-2z"/>
          </svg>
        </button>
      </div>
    </div>

    <!-- 标签栏 -->
    <div class="list-header">
      <div class="tabs">
        <button
          v-for="(tab, index) in allTabs"
          :key="tab.key"
          class="tab-btn"
          :class="{ active: activeTab === tab.key, 'is-pinned': tab.isPinned, 'is-fixed': !tab.isPinned }"
          :draggable="tab.isPinned"
          @click="handleTabClick(tab.key)"
          @dragstart="handleDragStart($event, index)"
          @dragover="handleDragOver($event, index)"
          @drop="handleDrop($event, index)"
          @dragend="handleDragEnd"
        >
          <span class="tab-label">{{ tab.label }}</span>
          <span
            v-if="tab.isPinned"
            class="tab-close"
            @click.stop="unpinSearch(tab.key)"
            title="取消固定"
          >×</span>
        </button>
      </div>
      <!-- 右侧操作按钮（暂时隐藏） -->
      <!-- <div class="header-actions">
        <button class="icon-btn" title="通知">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"/>
            <path d="M13.73 21a2 2 0 0 1-3.46 0"/>
          </svg>
        </button>
        <button class="icon-btn" title="AI/设置">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2L2 7l10 5 10-5-10-5z"/>
            <path d="M2 17l10 5 10-5"/>
            <path d="M2 12l10 5 10-5"/>
          </svg>
        </button>
      </div> -->
    </div>

    <!-- 列表内容 -->
    <div ref="listContainerRef" class="list-container" @scroll="handleScroll" tabindex="-1">
      <div v-if="filteredHistory.length === 0" class="empty-state">
        <div class="empty-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
            <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
          </svg>
        </div>
        <div class="empty-text">
          {{ searchQuery ? '没有找到匹配的内容' : '剪贴板为空' }}
        </div>
      </div>

      <template v-else>
        <ClipboardItem
          v-for="(item, index) in filteredHistory"
          :key="item.id"
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
      </template>

      <div v-if="loading" class="loading-more">加载中...</div>
    </div>

    <!-- 搜索栏 - 底部位置（默认） -->
    <div v-if="settings.search_position !== 'top'" class="search-bar search-bar-bottom">
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
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2a2 2 0 0 0-2 2v3.586l-3.707 3.707A1 1 0 0 0 6 12v2a1 1 0 0 0 1 1h3l2 7 2-7h3a1 1 0 0 0 1-1v-2a1 1 0 0 0-.293-.707L14 7.586V4a2 2 0 0 0-2-2z"/>
          </svg>
        </button>
      </div>
    </div>

    <!-- 右键上下文菜单 -->
    <ContextMenu
      v-model:visible="contextMenuVisible"
      :position="contextMenuPosition"
      :item="contextMenuItem"
      @action="handleContextMenuAction"
    />

    <!-- 粘贴队列面板（暂时隐藏） -->
    <!-- <PasteQueuePanel
      ref="pasteQueueRef"
      @paste="handleQueuePaste"
    /> -->

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
    <div v-if="deleteConfirmVisible" class="confirm-dialog-overlay" @click="cancelDelete">
      <div class="confirm-dialog" @click.stop>
        <div class="confirm-dialog-header">
          <svg class="warning-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
            <line x1="12" y1="9" x2="12" y2="13"/>
            <line x1="12" y1="17" x2="12.01" y2="17"/>
          </svg>
          <h3>确认删除</h3>
        </div>
        <div class="confirm-dialog-content">
          <p>确定要删除这条剪贴板记录吗？</p>
          <p class="confirm-dialog-subtitle">此操作不可撤销</p>
        </div>
        <div class="confirm-dialog-actions">
          <button class="confirm-btn cancel" @click="cancelDelete">取消</button>
          <button class="confirm-btn confirm" @click="confirmDelete">删除</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick, watch } from 'vue';
import ClipboardItem from './ClipboardItem.vue';
import ContextMenu from './ContextMenu.vue';
// import PasteQueuePanel from './PasteQueuePanel.vue';
import DrawerEditor from './DrawerEditor.vue';
import TagManager from './TagManager.vue';
import SmartSearch from './SmartSearch.vue';
import { useClipboard } from '@/composables/useClipboard';
import { writeText } from 'tauri-plugin-clipboard-x-api';
// import { usePasteQueue } from '@/composables/usePasteQueue';
import { useSettings } from '@/composables/useSettings';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import type { ClipboardItem as ClipboardItemType, PinnedSearch } from '@/types';
import { 
  parseSearchQuery, 
  matchItemWithQuery, 
  getSearchHistory, 
  addSearchHistory, 
  removeSearchHistory as removeSearchHistoryItem,
  clearSearchHistory,
  type ParsedQuery 
} from '@/composables/useSmartSearch';

const {
  history,
  lastCopyTime,
  loadHistory,
  deleteItem,
  restoreToClipboard,
} = useClipboard();

// 智能搜索
const smartSearchRef = ref<InstanceType<typeof SmartSearch> | null>(null);
const searchHistory = ref<string[]>([]);
const parsedQuery = computed<ParsedQuery>(() => parseSearchQuery(searchQuery.value));

// 加载搜索历史
const loadSearchHistory = () => {
  searchHistory.value = getSearchHistory();
};

// 移除搜索历史
const removeSearchHistory = (query: string) => {
  removeSearchHistoryItem(query);
  loadSearchHistory();
};

// 清空所有搜索历史
const clearAllHistory = () => {
  clearSearchHistory();
  loadSearchHistory();
};


// const { addToQueue } = usePasteQueue();
const { settings } = useSettings();
// const pasteQueueRef = ref<InstanceType<typeof PasteQueuePanel> | null>(null);

// Drawer editor state
const drawerVisible = ref(false);
const drawerItem = ref<ClipboardItemType | null>(null);

// Tag manager state
const tagManagerVisible = ref(false);
const tagManagerItem = ref<ClipboardItemType | null>(null);

// 固定标签（不可拖拽）
const FIXED_TABS = [
  { key: 'all', label: '全部' },
  { key: 'text', label: '文本' },
  { key: 'image', label: '图片' },
  { key: 'file', label: '文件' },
];

// 固定搜索列表
const pinnedSearches = ref<PinnedSearch[]>([]);
const PINNED_SEARCH_STORAGE_KEY = 'paste_library_pinned_searches';

// 加载固定搜索
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

// 保存固定搜索
const savePinnedSearches = () => {
  try {
    localStorage.setItem(PINNED_SEARCH_STORAGE_KEY, JSON.stringify(pinnedSearches.value));
  } catch {
    // 忽略存储错误
  }
};

// 计算所有标签（固定 + 钉住的搜索）
const allTabs = computed(() => {
  const fixedTabs = FIXED_TABS.map(tab => ({ ...tab, isPinned: false }));
  const pinnedTabs = pinnedSearches.value.map(ps => ({
    key: ps.id,
    label: ps.label,
    isPinned: true,
    query: ps.query,
  }));
  return [...fixedTabs, ...pinnedTabs];
});

// 判断当前搜索是否可以固定
const canPinCurrentSearch = computed(() => {
  const query = searchQuery.value.trim();
  if (!query) return false;
  // 检查是否已存在相同的固定搜索
  return !pinnedSearches.value.some(ps => ps.query === query);
});

// 固定当前搜索
const pinCurrentSearch = () => {
  const query = searchQuery.value.trim();
  if (!query) return;

  // 生成标签（简化显示）
  let label = query;
  if (query.length > 10) {
    label = query.slice(0, 10) + '...';
  }

  const newPinned: PinnedSearch = {
    id: `pinned_${Date.now()}`,
    label,
    query,
    created_at: Date.now(),
  };

  pinnedSearches.value.push(newPinned);
  savePinnedSearches();

  // 切换到新固定的标签
  activeTab.value = newPinned.id;
};

// 取消固定搜索
const unpinSearch = (id: string) => {
  const index = pinnedSearches.value.findIndex(ps => ps.id === id);
  if (index > -1) {
    pinnedSearches.value.splice(index, 1);
    savePinnedSearches();

    // 如果当前激活的是被取消固定的标签，切换到全部
    if (activeTab.value === id) {
      activeTab.value = 'all';
      searchQuery.value = '';
    }
  }
};

// 拖拽相关状态
const dragIndex = ref<number | null>(null);

// 处理拖拽开始
const handleDragStart = (event: DragEvent, index: number) => {
  // 不允许拖拽固定标签（前4个）
  if (index < FIXED_TABS.length) {
    event.preventDefault();
    return;
  }
  dragIndex.value = index;
  if (event.dataTransfer) {
    event.dataTransfer.effectAllowed = 'move';
  }
};

// 处理拖拽经过
const handleDragOver = (event: DragEvent, _index: number) => {
  event.preventDefault();
  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = 'move';
  }
};

// 处理放置
const handleDrop = (event: DragEvent, dropIndex: number) => {
  event.preventDefault();
  if (dragIndex.value === null) return;

  const fromIndex = dragIndex.value;
  const toIndex = dropIndex;

  // 不允许放置到固定标签区域（前4个）
  if (toIndex < FIXED_TABS.length) return;
  // 不允许从固定标签区域拖拽
  if (fromIndex < FIXED_TABS.length) return;

  // 计算在 pinnedSearches 数组中的索引
  const pinnedFromIndex = fromIndex - FIXED_TABS.length;
  const pinnedToIndex = toIndex - FIXED_TABS.length;

  // 交换位置
  const item = pinnedSearches.value.splice(pinnedFromIndex, 1)[0];
  pinnedSearches.value.splice(pinnedToIndex, 0, item);

  savePinnedSearches();
  dragIndex.value = null;
};

// 处理拖拽结束
const handleDragEnd = () => {
  dragIndex.value = null;
};

// 处理标签点击
const handleTabClick = (key: string) => {
  activeTab.value = key;

  // 如果是固定搜索标签，设置搜索查询
  const pinned = pinnedSearches.value.find(ps => ps.id === key);
  if (pinned) {
    searchQuery.value = pinned.query;
  } else if (key !== 'all') {
    // 固定标签（text/image/file）
    searchQuery.value = '';
  } else {
    // 全部
    searchQuery.value = '';
  }
};

const activeTab = ref('all');
const searchQuery = ref('');
const loading = ref(false);
const hasMore = ref(true);
const offset = ref(0);
const limit = 50;
const searchInputRef = ref<HTMLInputElement | null>(null);

// 右键菜单状态
const contextMenuVisible = ref(false);
const contextMenuPosition = ref({ x: 0, y: 0 });
const contextMenuItem = ref<ClipboardItemType | null>(null);

const selectedIndex = ref(-1);
const listContainerRef = ref<HTMLElement | null>(null);

// 删除确认对话框状态
const deleteConfirmVisible = ref(false);
const itemToDelete = ref<ClipboardItemType | null>(null);

// 智能激活逻辑
const handleSmartActivate = async () => {
  // 检查智能激活是否开启
  if (settings.value.smart_activate) {
    const timeDiff = Date.now() - lastCopyTime.value;

    // 如果距离上次复制 < 5秒，执行智能激活
    if (timeDiff < 5000) {
      // 1. 清空搜索框
      searchQuery.value = '';
      await handleSearch();

      // 2. 滚动到顶部
      if (listContainerRef.value) {
        listContainerRef.value.scrollTop = 0;
      }

      // 3. 切换到"全部"标签
      activeTab.value = 'all';

      // 4. 聚焦搜索框（不自动滚动，避免影响列表）
      searchInputRef.value?.focus({ preventScroll: true } as FocusOptions);

      // 智能激活完成后不再执行 focus_search_on_activate
      return;
    }
  }

  // 独立的「激活时聚焦搜索框」设置（与 smart_activate 独立工作）
  if (settings.value.focus_search_on_activate) {
    searchInputRef.value?.focus({ preventScroll: true } as FocusOptions);
  }
};

// 精确匹配函数
const fuzzyMatch = (query: string, text: string): boolean => {
  const queryLower = query.toLowerCase();
  const textLower = text.toLowerCase();
  
  // 精确匹配
  return textLower.includes(queryLower);
};

const filteredHistory = computed(() => {
  let result = history.value;

  // 检查是否是固定搜索标签
  const pinnedSearch = pinnedSearches.value.find(ps => ps.id === activeTab.value);
  if (pinnedSearch) {
    // 固定搜索标签：使用其 query 进行过滤
    const pinnedQuery = parseSearchQuery(pinnedSearch.query);
    if (pinnedQuery.isValid) {
      result = result.filter(item => matchItemWithQuery(item, pinnedQuery, fuzzyMatch));
    }
  } else if (activeTab.value !== 'all') {
    // 按类型过滤（Tab 切换）
    if (activeTab.value === 'file') {
      // 文件标签页显示所有文件相关类型：单个文件、多文件、文件夹
      result = result.filter(item =>
        item.content_type === 'file' ||
        item.content_type === 'files' ||
        item.content_type === 'folder'
      );
    } else if (activeTab.value === 'text') {
      // 文本标签页显示所有文本相关类型：纯文本、HTML、RTF
      result = result.filter(item =>
        item.content_type === 'text' ||
        item.content_type === 'html' ||
        item.content_type === 'rtf'
      );
    } else {
      result = result.filter(item => item.content_type === activeTab.value);
    }
  }

  // 按智能搜索查询过滤（当不是固定搜索标签时）
  if (!pinnedSearch && parsedQuery.value.isValid) {
    result = result.filter(item => matchItemWithQuery(item, parsedQuery.value, fuzzyMatch));
  }

  return result;
});

// 智能搜索处理（实时过滤，不保存历史）
const handleSmartSearch = async (query: string) => {
  searchQuery.value = query;

  // 滚动到顶部并重置选中项
  nextTick(() => {
    if (listContainerRef.value) {
      listContainerRef.value.scrollTop = 0;
    }
    if (filteredHistory.value.length > 0) {
      selectedIndex.value = 0;
    }
  });
};

// 搜索提交处理（保存到历史记录）
const handleSearchCommit = (query: string) => {
  if (query.trim()) {
    addSearchHistory(query.trim());
    loadSearchHistory();
  }
};

// 处理标签点击（从子组件传递上来的事件）
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

// 保留旧的处理函数用于兼容
const handleSearch = async () => {
  await handleSmartSearch(searchQuery.value);
};

const handleItemClick = async (item: ClipboardItemType, index: number) => {
  // 单击时选中该项
  selectedIndex.value = index;

  // 单击：根据 click_action 设置执行复制、粘贴或不操作
  const clickAction = settings.value.click_action;

  // 如果设置为不操作，仅选中该项
  if (clickAction === 'none') {
    return;
  }

  const copyAsPlainText = settings.value.copy_as_plain_text;

  // 先复制到剪贴板
  await restoreToClipboard(item, { copyAsPlainText });

  // 如果设置为粘贴，则执行粘贴动作
  if (clickAction === 'paste') {
    await simulatePaste();
  } else if (settings.value.hide_window_after_copy) {
    // 如果设置了复制后隐藏窗口（且不是粘贴操作，因为粘贴已经会隐藏窗口）
    await invoke('hide_clipboard_window');
  }
};

const handleItemDoubleClick = async (item: ClipboardItemType, index: number) => {
  // 双击时选中该项
  selectedIndex.value = index;

  // 双击：根据 double_click_action 设置执行复制、粘贴或不操作
  const doubleClickAction = settings.value.double_click_action;

  // 如果设置为不操作，仅选中该项
  if (doubleClickAction === 'none') {
    return;
  }

  const copyAsPlainText = settings.value.copy_as_plain_text;

  // 先复制到剪贴板
  await restoreToClipboard(item, { copyAsPlainText });

  // 如果设置为粘贴，则执行粘贴动作
  if (doubleClickAction === 'paste') {
    await simulatePaste();
  } else if (settings.value.hide_window_after_copy) {
    // 如果设置了复制后隐藏窗口（且不是粘贴操作，因为粘贴已经会隐藏窗口）
    await invoke('hide_clipboard_window');
  }
};

const handleItemContextMenu = (event: MouseEvent, item: ClipboardItemType, index: number) => {
  contextMenuPosition.value = { x: event.clientX, y: event.clientY };
  contextMenuItem.value = item;
  contextMenuVisible.value = true;
  
  // 右键点击时，高亮该Item（设置选中索引）
  selectedIndex.value = index;
};

const handleQuickAction = async (action: string, item: ClipboardItemType) => {
  switch (action) {
    case 'detail':
      // 打开抽屉编辑器查看详情
      drawerItem.value = item;
      drawerVisible.value = true;
      break;
    case 'copy':
      await restoreToClipboard(item, { copyAsPlainText: settings.value.copy_as_plain_text });
      break;
    case 'delete':
      await handleDelete(item);
      break;
    case 'tag':
      // 打开标签管理器
      tagManagerItem.value = item;
      tagManagerVisible.value = true;
      break;
    // case 'queue':
    //   // 添加到粘贴队列
    //   addToQueue(item);
    //   break;
  }
};

// 处理删除操作（带确认对话框）
const handleDelete = async (item: ClipboardItemType) => {
  if (settings.value.confirm_delete) {
    // 显示确认对话框
    itemToDelete.value = item;
    deleteConfirmVisible.value = true;
  } else {
    // 直接删除
    await deleteItem(item.id);
  }
};

// 确认删除
const confirmDelete = async () => {
  if (itemToDelete.value) {
    await deleteItem(itemToDelete.value.id);
    itemToDelete.value = null;
  }
  deleteConfirmVisible.value = false;
};

// 取消删除
const cancelDelete = () => {
  itemToDelete.value = null;
  deleteConfirmVisible.value = false;
};

// const showTagManager = (item: ClipboardItemType) => {
//   // TODO: 实现标签管理弹窗
//   console.log('Tag manager for item:', item.id);
// };

// const handleQueuePaste = async (content: string) => {
//   // 将合并后的内容写入剪贴板并粘贴
//   try {
//     const { writeText } = await import('tauri-plugin-clipboard-x-api');
//     await writeText(content);
//     // TODO: 模拟粘贴操作
//     console.log('Queue pasted:', content.substring(0, 100) + '...');
//   } catch (error) {
//     console.error('Failed to paste queue:', error);
//   }
// };

// Drawer handlers
const handleDrawerCopy = async (item: ClipboardItemType) => {
  await restoreToClipboard(item, { copyAsPlainText: settings.value.copy_as_plain_text });
  // 如果设置了复制后隐藏窗口
  if (settings.value.hide_window_after_copy) {
    await invoke('hide_clipboard_window');
  }
};

const handleDrawerPaste = async (item: ClipboardItemType) => {
  await restoreToClipboard(item, { copyAsPlainText: settings.value.copy_as_plain_text });
  // 执行粘贴操作（与单击/双击粘贴行为一致）
  await simulatePaste();
};

// 模拟粘贴操作
const simulatePaste = async (): Promise<void> => {
  try {
    // 隐藏窗口，让用户看到粘贴效果
    await invoke('hide_clipboard_window');

    // 等待窗口完全隐藏
    await new Promise(resolve => setTimeout(resolve, 100));

    // 调用后端模拟粘贴命令，传入用户设置的快捷键模式
    await invoke('simulate_paste', { pasteShortcut: settings.value.paste_shortcut });
  } catch (error) {
    // 如果后端命令不存在，静默失败
    console.log('Paste simulation not available yet');
  }
};

// 复制文件路径到剪贴板
const copyFilePath = async (item: ClipboardItemType): Promise<void> => {
  try {
    let pathToCopy = '';

    if (item.file_paths && item.file_paths.length > 0) {
      // 多文件时复制所有路径，用换行符分隔（Windows风格 \r\n）
      pathToCopy = item.file_paths.join('\r\n') + '\r\n';
    } else if (item.content) {
      // 使用 content 字段（文件夹类型），末尾添加换行符
      pathToCopy = item.content + '\r\n';
    }

    if (pathToCopy) {
      await writeText(pathToCopy);
    }
  } catch (error) {
    console.error('Failed to copy file path:', error);
  }
};

const handleSaveAsNew = async (content: string, type: string) => {
  try {
    if (type === 'html') {
      await invoke('add_clipboard_item', { text: content.replace(/<[^>]*>/g, ''), html: content });
    } else {
      await invoke('add_clipboard_item', { text: content, html: null });
    }
    await loadHistory();
  } catch (error) {
    console.error('Failed to save as new:', error);
  }
};

// 处理标签管理器保存
const handleTagManagerSave = async (itemId: number, tags: string[]) => {
  // 更新本地数据
  const item = history.value.find(h => h.id === itemId);
  if (item) {
    item.tags = tags.length > 0 ? tags : undefined;
  }
};

// 处理删除单个标签
const handleRemoveTag = async (item: ClipboardItemType, tag: string) => {
  try {
    // 从标签列表中移除该标签
    const newTags = (item.tags || []).filter(t => t !== tag);

    // 调用后端更新标签
    await invoke('update_tags', {
      id: item.id,
      tags: newTags.length > 0 ? newTags : null,
    });

    // 更新本地数据
    item.tags = newTags.length > 0 ? newTags : undefined;
  } catch (error) {
    console.error('Failed to remove tag:', error);
  }
};

const handleContextMenuAction = async (action: string, item: ClipboardItemType) => {
  switch (action) {
    case 'copy':
      // 右键菜单复制：仅复制到剪贴板（与单击/双击的复制行为一致）
      await restoreToClipboard(item, { copyAsPlainText: settings.value.copy_as_plain_text });
      // 如果设置了复制后隐藏窗口
      if (settings.value.hide_window_after_copy) {
        await invoke('hide_clipboard_window');
      }
      break;
    case 'paste':
      // 右键菜单粘贴：复制到剪贴板并执行粘贴（与单击/双击的粘贴行为一致）
      await restoreToClipboard(item, { copyAsPlainText: settings.value.copy_as_plain_text });
      await simulatePaste();
      break;
    case 'tag':
      // 打开标签管理器
      tagManagerItem.value = item;
      tagManagerVisible.value = true;
      break;
    case 'copyPlain':
      // 复制为纯文本
      await restoreToClipboard({
        ...item,
        content_type: 'text',
        content: item.content.replace(/<[^>]*>/g, ''),
      });
      // 如果设置了复制后隐藏窗口
      if (settings.value.hide_window_after_copy) {
        await invoke('hide_clipboard_window');
      }
      break;
    case 'pastePlain':
      // 粘贴为纯文本：复制纯文本到剪贴板并执行粘贴
      await restoreToClipboard({
        ...item,
        content_type: 'text',
        content: item.content.replace(/<[^>]*>/g, ''),
      });
      await simulatePaste();
      break;
    case 'delete':
      await handleDelete(item);
      break;
    case 'openFile':
      // 打开文件
      if (item.file_paths && item.file_paths.length > 0) {
        await invoke('open_file', { path: item.file_paths[0] });
      } else if (item.thumbnail_path) {
        await invoke('open_file', { path: item.thumbnail_path });
      }
      break;
    case 'showInFolder':
      // 在文件夹中显示
      if (item.file_paths && item.file_paths.length > 0) {
        await invoke('show_in_folder', { path: item.file_paths[0] });
      } else if (item.thumbnail_path) {
        await invoke('show_in_folder', { path: item.thumbnail_path });
      }
      break;
    case 'copyFilePath':
      // 复制文件路径到剪贴板
      await copyFilePath(item);
      break;
  }
};

const handleScroll = async (event: Event) => {
  const target = event.target as HTMLElement;
  const bottom = target.scrollHeight - target.scrollTop <= target.clientHeight + 100;

  if (bottom && hasMore.value && !loading.value && !searchQuery.value) {
    loading.value = true;
    offset.value += limit;
    loading.value = false;
  }
};

// 键盘导航处理
const handleKeyDown = async (e: KeyboardEvent) => {
  // Ctrl+F 聚焦搜索框
  if ((e.ctrlKey || e.metaKey) && e.key === 'f') {
    e.preventDefault();
    smartSearchRef.value?.focus();
    return;
  }

  // Esc 关闭窗口
  if (e.key === 'Escape') {
    e.preventDefault();
    invoke('hide_clipboard_window');
    return;
  }

  // 上下导航
  if (e.key === 'ArrowUp' || e.key === 'ArrowDown') {
    e.preventDefault();
    const direction = e.key === 'ArrowUp' ? -1 : 1;
    const newIndex = selectedIndex.value + direction;

    if (newIndex >= 0 && newIndex < filteredHistory.value.length) {
      selectedIndex.value = newIndex;
      
      // 自动滚动到可视区域
      scrollSelectedItemIntoView();
    }
    return;
  }

  // Enter 粘贴选中项
  if (e.key === 'Enter' && selectedIndex.value >= 0) {
    e.preventDefault();
    const item = filteredHistory.value[selectedIndex.value];
    if (item) {
      await restoreToClipboard(item, { copyAsPlainText: settings.value.copy_as_plain_text });
      // 模拟粘贴
      await simulatePaste();
    }
    return;
  }

  // 数字键 1-9 快速粘贴
  if (e.key >= '1' && e.key <= '9') {
    // 根据设置判断是否需要修饰键
    const shortcut = settings.value.number_key_shortcut || 'ctrl';
    let shouldTrigger = false;

    if (shortcut === 'none') {
      // 直接按数字键即可触发
      shouldTrigger = !e.ctrlKey && !e.altKey && !e.shiftKey && !e.metaKey;
    } else {
      // 解析用户设置的修饰键组合（如 "ctrl", "ctrl+shift", "alt+shift" 等）
      const requiredModifiers = shortcut.toLowerCase().split('+').map(s => s.trim());
      const actualModifiers: string[] = [];
      if (e.ctrlKey) actualModifiers.push('ctrl');
      if (e.altKey) actualModifiers.push('alt');
      if (e.shiftKey) actualModifiers.push('shift');
      if (e.metaKey) actualModifiers.push('meta');

      // 检查是否完全匹配（修饰键种类和数量都相同）
      const allRequiredPressed = requiredModifiers.every(mod => actualModifiers.includes(mod));
      const noExtraModifiers = actualModifiers.every(mod => requiredModifiers.includes(mod));
      shouldTrigger = allRequiredPressed && noExtraModifiers;
    }

    if (shouldTrigger) {
      const index = parseInt(e.key) - 1;
      if (index < filteredHistory.value.length) {
        e.preventDefault();
        const item = filteredHistory.value[index];
        if (item) {
          await restoreToClipboard(item, { copyAsPlainText: settings.value.copy_as_plain_text });
          // 模拟粘贴
          await simulatePaste();
        }
      }
    }
    return;
  }
};

// 标记是否已经执行过智能激活（每次窗口打开时重置）
const hasActivated = ref(false);

onMounted(async () => {
  loadHistory(settings.value.max_history_count, 0);
  loadSearchHistory();
  loadPinnedSearches();
  window.addEventListener('keydown', handleKeyDown);
  // 初始化选中第一项
  if (filteredHistory.value.length > 0) {
    selectedIndex.value = 0;
  }
  
  // 使用 Tauri 窗口监听 focus 事件（智能激活）
  const appWindow = getCurrentWindow();
  const unlistenFocus = await appWindow.listen('tauri://focus', () => {
    // 只在首次激活时执行智能激活，避免频繁焦点切换导致 hover 失效
    if (!hasActivated.value) {
      hasActivated.value = true;
      handleSmartActivate();
    }
  });
  
  // 监听窗口隐藏事件，重置激活标记
  const unlistenBlur = await appWindow.listen('tauri://blur', () => {
    // 窗口失去焦点时重置标记，下次打开时重新执行
    hasActivated.value = false;
  });
  
  // 保存清理函数
  (window as any).__unlistenFocus = unlistenFocus;
  (window as any).__unlistenBlur = unlistenBlur;
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown);
  // 清理 Tauri 事件监听
  if ((window as any).__unlistenFocus) {
    (window as any).__unlistenFocus();
  }
  if ((window as any).__unlistenBlur) {
    (window as any).__unlistenBlur();
  }
  // 重置激活标记
  hasActivated.value = false;
});

// 滚动选中项到可视区域
const scrollSelectedItemIntoView = () => {
  if (selectedIndex.value < 0 || !listContainerRef.value) return;
  
  const container = listContainerRef.value;
  const itemElement = container.children[selectedIndex.value] as HTMLElement;
  
  if (itemElement) {
    itemElement.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
  }
};

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

/* 头部导航 */
.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background: #fff;
  border-bottom: 1px solid #f0f0f0;
  -webkit-app-region: no-drag;
  app-region: no-drag;
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

.tabs {
  display: flex;
  gap: 4px;
}

.tab-btn {
  padding: 4px 10px;
  border: none;
  background: transparent;
  color: #595959;
  font-size: 13px;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: 4px;
}

.tab-btn:hover {
  color: #262626;
  background: #f5f5f5;
}

.tab-btn.active {
  color: #fff;
  background: #262626;
}

.tab-btn.is-pinned {
  background: #e6f7ff;
  color: #1890ff;
  border: 1px solid #91d5ff;
}

.tab-btn.is-pinned:hover {
  background: #bae7ff;
}

.tab-btn.is-pinned.active {
  background: #1890ff;
  color: #fff;
}

.tab-btn.is-fixed {
  cursor: default;
}

.tab-btn.is-fixed:hover {
  background: transparent;
}

.tab-btn.is-fixed.active:hover {
  background: #262626;
}

.tab-label {
  pointer-events: none;
}

.tab-close {
  width: 14px;
  height: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  font-size: 12px;
  line-height: 1;
  margin-left: 2px;
  opacity: 0.6;
  transition: all 0.2s;
}

.tab-close:hover {
  opacity: 1;
  background: rgba(0, 0, 0, 0.1);
}

.tab-btn.is-pinned.active .tab-close:hover {
  background: rgba(255, 255, 255, 0.2);
}

.header-actions {
  display: flex;
  gap: 4px;
}

.icon-btn {
  width: 28px;
  height: 28px;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #8c8c8c;
  transition: all 0.2s;
}

.icon-btn:hover {
  background: #f5f5f5;
  color: #262626;
}

.icon-btn svg {
  width: 16px;
  height: 16px;
}

/* 列表内容 */
.list-container {
  flex: 1;
  overflow-y: auto;
  background: #fff;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  color: #bfbfbf;
}

.empty-icon {
  width: 48px;
  height: 48px;
  margin-bottom: 12px;
  color: #d9d9d9;
}

.empty-icon svg {
  width: 100%;
  height: 100%;
}

.empty-text {
  font-size: 13px;
}

.loading-more {
  text-align: center;
  padding: 12px;
  color: #8c8c8c;
  font-size: 12px;
}

/* 搜索栏 */
.search-bar {
  padding: 10px 12px;
  background: #fff;
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.search-bar-top {
  border-bottom: 1px solid #f0f0f0;
}

.search-bar-bottom {
  border-top: 1px solid #f0f0f0;
}

.search-input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

.search-icon {
  position: absolute;
  left: 10px;
  width: 16px;
  height: 16px;
  color: #bfbfbf;
}

.search-input-wrapper input {
  width: 100%;
  padding: 8px 60px 8px 32px;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  outline: none;
  transition: all 0.2s;
  background: #f5f5f5;
}

.search-input-wrapper input:focus {
  background: #fff;
  box-shadow: 0 0 0 1px #262626;
}

.search-input-wrapper input::placeholder {
  color: #8c8c8c;
}

.shortcut-key {
  position: absolute;
  right: 8px;
  top: 50%;
  transform: translateY(-50%);
  padding: 3px 8px;
  font-size: 11px;
  color: #737373;
  background: #e8e8e8;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
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

/* 确认对话框 */
.confirm-dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.confirm-dialog {
  background: #fff;
  border-radius: 12px;
  width: 360px;
  max-width: 90%;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  overflow: hidden;
}

.confirm-dialog-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 20px 20px 12px;
}

.warning-icon {
  width: 32px;
  height: 32px;
  color: #faad14;
  flex-shrink: 0;
}

.confirm-dialog-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #262626;
}

.confirm-dialog-content {
  padding: 0 20px 20px;
  padding-left: 64px;
}

.confirm-dialog-content p {
  margin: 0 0 4px;
  font-size: 14px;
  color: #595959;
  line-height: 1.5;
}

.confirm-dialog-subtitle {
  font-size: 12px !important;
  color: #8c8c8c !important;
}

.confirm-dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 12px 20px 20px;
  padding-left: 64px;
}

.confirm-btn {
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
}

.confirm-btn.cancel {
  background: #f5f5f5;
  color: #595959;
}

.confirm-btn.cancel:hover {
  background: #e8e8e8;
  color: #262626;
}

.confirm-btn.confirm {
  background: #ff4d4f;
  color: #fff;
}

.confirm-btn.confirm:hover {
  background: #ff7875;
}
</style>
