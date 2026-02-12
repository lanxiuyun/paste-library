<template>
  <div class="clipboard-list">
    <!-- 搜索栏 - 顶部位置 -->
    <div v-if="settings.search_position === 'top'" class="search-bar search-bar-top">
      <div class="search-input-wrapper">
        <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8"/>
          <path d="M21 21l-4.35-4.35"/>
        </svg>
        <input
          ref="searchInputRef"
          v-model="searchQuery"
          type="text"
          placeholder="搜索剪贴板..."
          @input="handleSearch"
        />
        <kbd class="shortcut-key">Ctrl+F</kbd>
      </div>
    </div>

    <!-- 标签栏 -->
    <div class="list-header">
      <div class="tabs">
        <button 
          v-for="tab in tabs" 
          :key="tab.key"
          class="tab-btn"
          :class="{ active: activeTab === tab.key }"
          @click="activeTab = tab.key"
        >
          {{ tab.label }}
        </button>
      </div>
      <div class="header-actions">
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
      </div>
    </div>

    <!-- 列表内容 -->
    <div ref="listContainerRef" class="list-container" @scroll="handleScroll">
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
          @click="handleItemClick"
          @dblclick="handleItemDoubleClick"
          @contextmenu="handleItemContextMenu"
          @quick-action="handleQuickAction"
        />
      </template>

      <div v-if="loading" class="loading-more">加载中...</div>
    </div>

    <!-- 搜索栏 - 底部位置（默认） -->
    <div v-if="settings.search_position !== 'top'" class="search-bar search-bar-bottom">
      <div class="search-input-wrapper">
        <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8"/>
          <path d="M21 21l-4.35-4.35"/>
        </svg>
        <input
          ref="searchInputRef"
          v-model="searchQuery"
          type="text"
          placeholder="搜索剪贴板..."
          @input="handleSearch"
        />
        <kbd class="shortcut-key">Ctrl+F</kbd>
      </div>
    </div>

    <!-- 右键上下文菜单 -->
    <ContextMenu
      v-model:visible="contextMenuVisible"
      :position="contextMenuPosition"
      :item="contextMenuItem"
      @action="handleContextMenuAction"
    />

    <!-- 粘贴队列面板 -->
    <PasteQueuePanel
      ref="pasteQueueRef"
      @paste="handleQueuePaste"
    />

    <!-- 抽屉编辑器 -->
    <DrawerEditor
      v-model:visible="drawerVisible"
      :item="drawerItem"
      @copy="handleDrawerCopy"
      @paste="handleDrawerPaste"
      @saveAsNew="handleSaveAsNew"
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
import { ref, computed, onMounted, onUnmounted } from 'vue';
import ClipboardItem from './ClipboardItem.vue';
import ContextMenu from './ContextMenu.vue';
import PasteQueuePanel from './PasteQueuePanel.vue';
import DrawerEditor from './DrawerEditor.vue';
import { useClipboard } from '@/composables/useClipboard';
import { writeText } from 'tauri-plugin-clipboard-x-api';
import { usePasteQueue } from '@/composables/usePasteQueue';
import { useSettings } from '@/composables/useSettings';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import type { ClipboardItem as ClipboardItemType } from '@/types';

const {
  history,
  lastCopyTime,
  loadHistory,
  searchHistory,
  deleteItem,
  restoreToClipboard,
} = useClipboard();

const { addToQueue } = usePasteQueue();
const { settings } = useSettings();
const pasteQueueRef = ref<InstanceType<typeof PasteQueuePanel> | null>(null);

// Drawer editor state
const drawerVisible = ref(false);
const drawerItem = ref<ClipboardItemType | null>(null);

const tabs = [
  { key: 'all', label: '全部' },
  { key: 'text', label: '文本' },
  { key: 'image', label: '图片' },
  { key: 'file', label: '文件' },
  { key: 'favorite', label: '收藏' },
];

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
const handleSmartActivate = () => {
  // 检查智能激活是否开启
  if (!settings.value.smart_activate) return;
  
  const timeDiff = Date.now() - lastCopyTime.value;
  
  // 如果距离上次复制 < 5秒，执行智能激活
  if (timeDiff < 5000) {
    // 1. 滚动到顶部
    if (listContainerRef.value) {
      listContainerRef.value.scrollTop = 0;
    }
    
    // 2. 切换到"全部"标签
    activeTab.value = 'all';
    
    // 3. 聚焦搜索框
    searchInputRef.value?.focus();
  }
};

const filteredHistory = computed(() => {
  let result = history.value;

  // 按标签过滤
  if (activeTab.value !== 'all') {
    if (activeTab.value === 'favorite') {
      // 过滤带"收藏"标签的项目
      result = result.filter(item => item.tags?.includes('收藏'));
    } else {
      result = result.filter(item => item.content_type === activeTab.value);
    }
  }

  return result;
});

const handleSearch = async () => {
  if (searchQuery.value) {
    await searchHistory(searchQuery.value);
  } else {
    await loadHistory();
  }
};

const handleItemClick = async (item: ClipboardItemType) => {
  // 单击：根据 click_action 设置执行复制或粘贴
  const clickAction = settings.value.click_action;
  const copyAsPlainText = settings.value.copy_as_plain_text;

  // 先复制到剪贴板
  await restoreToClipboard(item, { copyAsPlainText });

  // 如果设置为粘贴，则执行粘贴动作
  if (clickAction === 'paste') {
    await simulatePaste();
  }
};

const handleItemDoubleClick = async (item: ClipboardItemType) => {
  // 双击：根据 double_click_action 设置执行复制或粘贴
  const doubleClickAction = settings.value.double_click_action;
  const copyAsPlainText = settings.value.copy_as_plain_text;

  // 先复制到剪贴板
  await restoreToClipboard(item, { copyAsPlainText });

  // 如果设置为粘贴，则执行粘贴动作
  if (doubleClickAction === 'paste') {
    await simulatePaste();
  }
};

const handleItemContextMenu = (event: MouseEvent, item: ClipboardItemType) => {
  contextMenuPosition.value = { x: event.clientX, y: event.clientY };
  contextMenuItem.value = item;
  contextMenuVisible.value = true;
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
      // 打开标签管理弹窗
      showTagManager(item);
      break;
    case 'queue':
      // 添加到粘贴队列
      addToQueue(item);
      break;
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

const showTagManager = (item: ClipboardItemType) => {
  // TODO: 实现标签管理弹窗
  console.log('Tag manager for item:', item.id);
};

const handleQueuePaste = async (content: string) => {
  // 将合并后的内容写入剪贴板并粘贴
  try {
    const { writeText } = await import('tauri-plugin-clipboard-x-api');
    await writeText(content);
    // TODO: 模拟粘贴操作
    console.log('Queue pasted:', content.substring(0, 100) + '...');
  } catch (error) {
    console.error('Failed to paste queue:', error);
  }
};

// Drawer handlers
const handleDrawerCopy = async (item: ClipboardItemType) => {
  await restoreToClipboard(item, { copyAsPlainText: settings.value.copy_as_plain_text });
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

const handleContextMenuAction = async (action: string, item: ClipboardItemType) => {
  switch (action) {
    case 'copy':
      // 右键菜单复制：仅复制到剪贴板（与单击/双击的复制行为一致）
      await restoreToClipboard(item, { copyAsPlainText: settings.value.copy_as_plain_text });
      break;
    case 'paste':
      // 右键菜单粘贴：复制到剪贴板并执行粘贴（与单击/双击的粘贴行为一致）
      await restoreToClipboard(item, { copyAsPlainText: settings.value.copy_as_plain_text });
      await simulatePaste();
      break;
    case 'queue':
      addToQueue(item);
      break;
    case 'copyPlain':
      // 复制为纯文本
      await restoreToClipboard({
        ...item,
        content_type: 'text',
        content: item.content.replace(/<[^>]*>/g, ''),
      });
      break;
    case 'favorite':
      handleToggleFavorite(item.id, !item.tags?.includes('收藏'));
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

const handleToggleFavorite = async (id: number, isFavorite: boolean) => {
  try {
    // 获取当前项目
    const item = history.value.find(i => i.id === id);
    if (!item) return;

    // 更新标签列表
    const currentTags = item.tags || [];
    let newTags: string[];

    if (isFavorite) {
      // 添加"收藏"标签
      if (!currentTags.includes('收藏')) {
        newTags = [...currentTags, '收藏'];
      } else {
        newTags = currentTags;
      }
    } else {
      // 移除"收藏"标签
      newTags = currentTags.filter(t => t !== '收藏');
    }

    await invoke('update_tags', { id, tags: newTags });
    await loadHistory();
  } catch (error) {
    console.error('Failed to toggle favorite:', error);
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
    searchInputRef.value?.focus();
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
    return;
  }
};

onMounted(async () => {
  loadHistory(limit, 0);
  window.addEventListener('keydown', handleKeyDown);
  // 初始化选中第一项
  if (filteredHistory.value.length > 0) {
    selectedIndex.value = 0;
  }
  
  // 使用 Tauri 窗口监听 focus 事件（智能激活）
  const appWindow = getCurrentWindow();
  const unlistenFocus = await appWindow.listen('tauri://focus', () => {
    // 窗口获得焦点时执行智能激活
    handleSmartActivate();
  });
  
  // 保存清理函数
  (window as any).__unlistenFocus = unlistenFocus;
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown);
  // 清理 Tauri 事件监听
  if ((window as any).__unlistenFocus) {
    (window as any).__unlistenFocus();
  }
});

// 监听过滤变化，重置选中状态
import { watch } from 'vue';
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
}

.tab-btn:hover {
  color: #262626;
  background: #f5f5f5;
}

.tab-btn.active {
  color: #fff;
  background: #262626;
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
