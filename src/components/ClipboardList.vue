<template>
  <div class="clipboard-list">
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
    <div class="list-container" @scroll="handleScroll">
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
          @click="handleItemClick"
          @dblclick="handleItemDoubleClick"
          @contextmenu="handleItemContextMenu"
          @delete="handleItemDelete"
          @toggle-favorite="handleToggleFavorite"
          @copy="handleItemCopy"
        />
      </template>

      <div v-if="loading" class="loading-more">加载中...</div>
    </div>

    <!-- 底部搜索栏 -->
    <div class="search-bar">
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
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import ClipboardItem from './ClipboardItem.vue';
import ContextMenu from './ContextMenu.vue';
import { useClipboard } from '@/composables/useClipboard';
import { invoke } from '@tauri-apps/api/core';
import type { ClipboardItem as ClipboardItemType } from '@/types';

const {
  history,
  loadHistory,
  searchHistory,
  deleteItem,
  restoreToClipboard,
} = useClipboard();

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

const filteredHistory = computed(() => {
  let result = history.value;

  // 按标签过滤
  if (activeTab.value !== 'all') {
    if (activeTab.value === 'favorite') {
      // 过滤收藏的项目
      result = result.filter(item => item.is_favorite);
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
  // 单击：复制到剪贴板
  await restoreToClipboard(item);
};

const handleItemDoubleClick = async (item: ClipboardItemType) => {
  // 双击：复制并粘贴
  await restoreToClipboard(item);
  // TODO: 实现模拟粘贴功能
};

const handleItemContextMenu = (event: MouseEvent, item: ClipboardItemType) => {
  contextMenuPosition.value = { x: event.clientX, y: event.clientY };
  contextMenuItem.value = item;
  contextMenuVisible.value = true;
};

const handleContextMenuAction = async (action: string, item: ClipboardItemType) => {
  switch (action) {
    case 'copy':
      await restoreToClipboard(item);
      break;
    case 'paste':
      await restoreToClipboard(item);
      // TODO: 实现模拟粘贴功能
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
      handleToggleFavorite(item.id, !item.is_favorite);
      break;
    case 'delete':
      await deleteItem(item.id);
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
  }
};

const handleItemCopy = async (item: ClipboardItemType) => {
  await restoreToClipboard(item);
};

const handleItemDelete = async (id: number) => {
  await deleteItem(id);
};

const handleToggleFavorite = async (id: number, isFavorite: boolean) => {
  try {
    await invoke('toggle_favorite', { id, isFavorite });
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

// Ctrl+F 快捷键聚焦搜索框
const handleKeyDown = (e: KeyboardEvent) => {
  if ((e.ctrlKey || e.metaKey) && e.key === 'f') {
    e.preventDefault();
    searchInputRef.value?.focus();
  }
};

onMounted(() => {
  loadHistory(limit, 0);
  window.addEventListener('keydown', handleKeyDown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown);
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

/* 底部搜索栏 */
.search-bar {
  padding: 10px 12px;
  border-top: 1px solid #f0f0f0;
  background: #fff;
  -webkit-app-region: no-drag;
  app-region: no-drag;
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
</style>
