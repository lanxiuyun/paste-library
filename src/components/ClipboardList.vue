<template>
  <div class="clipboard-list">
    <!-- 顶部导航 -->
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
        <button class="icon-btn" title="设置">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="3"/>
            <path d="M12 1v6m0 6v6m4.22-10.22l4.24-4.24M6.34 17.66l-4.24 4.24M23 12h-6m-6 0H1m20.24 4.24l-4.24-4.24M6.34 6.34L2.1 2.1"/>
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
          v-for="item in filteredHistory"
          :key="item.id"
          :item="item"
          @click="handleItemClick"
          @delete="handleItemDelete"
          @toggle-favorite="handleToggleFavorite"
        />
      </template>

      <div v-if="loading" class="loading-more">加载中...</div>
    </div>

    <!-- 底部搜索框 -->
    <div class="search-bar">
      <div class="search-input-wrapper">
        <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8"/>
          <path d="M21 21l-4.35-4.35"/>
        </svg>
        <input
          v-model="searchQuery"
          type="text"
          placeholder="搜索"
          @input="handleSearch"
        />
        <button v-if="searchQuery" class="clear-btn" @click="clearSearch">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <path d="M15 9l-6 6M9 9l6 6"/>
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import ClipboardItem from './ClipboardItem.vue';
import { useClipboard } from '@/composables/useClipboard';
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

const filteredHistory = computed(() => {
  let result = history.value;

  // 按标签过滤
  if (activeTab.value !== 'all') {
    if (activeTab.value === 'favorite') {
      // 收藏功能暂时未实现，返回空数组
      result = [];
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

const clearSearch = async () => {
  searchQuery.value = '';
  await loadHistory();
};

const handleItemClick = async (item: ClipboardItemType) => {
  await restoreToClipboard(item);
};

const handleItemDelete = async (id: number) => {
  await deleteItem(id);
};

const handleToggleFavorite = (id: number, isFavorite: boolean) => {
  console.log('Toggle favorite:', id, isFavorite);
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

onMounted(() => {
  loadHistory(limit, 0);
});
</script>

<style scoped>
.clipboard-list {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: #f5f5f5;
}

/* 头部导航 */
.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px 12px;
  background: #fff;
  border-bottom: 1px solid #e8e8e8;
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.tabs {
  display: flex;
  gap: 4px;
}

.tab-btn {
  padding: 6px 16px;
  border: none;
  background: transparent;
  color: #666;
  font-size: 14px;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s;
}

.tab-btn:hover {
  color: #1890ff;
  background: #f0f0f0;
}

.tab-btn.active {
  color: #fff;
  background: #1890ff;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.icon-btn {
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #666;
  transition: all 0.2s;
}

.icon-btn:hover {
  background: #f0f0f0;
  color: #1890ff;
}

.icon-btn svg {
  width: 18px;
  height: 18px;
}

/* 列表内容 */
.list-container {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 20px;
  color: #999;
}

.empty-icon {
  width: 64px;
  height: 64px;
  margin-bottom: 16px;
  color: #d9d9d9;
}

.empty-icon svg {
  width: 100%;
  height: 100%;
}

.empty-text {
  font-size: 14px;
}

.loading-more {
  text-align: center;
  padding: 16px;
  color: #999;
  font-size: 14px;
}

/* 底部搜索栏 */
.search-bar {
  padding: 12px 16px;
  background: #fff;
  border-top: 1px solid #e8e8e8;
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
  left: 12px;
  width: 18px;
  height: 18px;
  color: #999;
}

.search-input-wrapper input {
  width: 100%;
  padding: 10px 36px 10px 38px;
  border: 1px solid #d9d9d9;
  border-radius: 6px;
  font-size: 14px;
  outline: none;
  transition: all 0.2s;
  background: #fafafa;
}

.search-input-wrapper input:focus {
  border-color: #1890ff;
  background: #fff;
}

.search-input-wrapper input::placeholder {
  color: #999;
}

.clear-btn {
  position: absolute;
  right: 8px;
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #999;
  transition: all 0.2s;
}

.clear-btn:hover {
  background: #f0f0f0;
  color: #666;
}

.clear-btn svg {
  width: 16px;
  height: 16px;
}

/* 滚动条样式 */
::-webkit-scrollbar {
  width: 6px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: #d9d9d9;
  border-radius: 3px;
}

::-webkit-scrollbar-thumb:hover {
  background: #bFBFBF;
}
</style>