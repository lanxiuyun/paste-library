<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useClipboard } from '@/composables/useClipboard';
import ClipboardItem from '@/components/ClipboardItem.vue';
import type { ClipboardItem as ClipboardItemType } from '@/types';

type TabType = 'all' | 'text' | 'image' | 'file';

const {
  history,
  loadHistory,
  searchHistory,
  deleteItem,
  restoreToClipboard,
} = useClipboard();

const searchQuery = ref('');
const activeTab = ref<TabType>('all');
const selectedIndex = ref(0);
const listRef = ref<HTMLElement | null>(null);

// Filter items by tab
const filteredItems = computed(() => {
  if (activeTab.value === 'all') {
    return history.value;
  }
  if (activeTab.value === 'text') {
    return history.value.filter(item => 
      item.content_type === 'text' || 
      item.content_type === 'html' || 
      item.content_type === 'rtf'
    );
  }
  if (activeTab.value === 'image') {
    return history.value.filter(item => item.content_type === 'image');
  }
  if (activeTab.value === 'file') {
    return history.value.filter(item => 
      item.content_type === 'file' || 
      item.content_type === 'folder' || 
      item.content_type === 'files'
    );
  }
  return history.value;
});

// Tab counts
const tabCounts = computed(() => {
  return {
    all: history.value.length,
    text: history.value.filter(item => 
      item.content_type === 'text' || 
      item.content_type === 'html' || 
      item.content_type === 'rtf'
    ).length,
    image: history.value.filter(item => item.content_type === 'image').length,
    file: history.value.filter(item => 
      item.content_type === 'file' || 
      item.content_type === 'folder' || 
      item.content_type === 'files'
    ).length,
  };
});

// Search handler
const handleSearch = async () => {
  if (searchQuery.value.trim()) {
    await searchHistory(searchQuery.value);
  } else {
    await loadHistory();
  }
};

// Tab change
const setTab = (tab: TabType) => {
  activeTab.value = tab;
  selectedIndex.value = 0;
};

// Keyboard navigation
const handleKeyDown = (event: KeyboardEvent) => {
  const items = filteredItems.value;
  if (items.length === 0) return;

  switch (event.key) {
    case 'ArrowDown':
      event.preventDefault();
      selectedIndex.value = Math.min(selectedIndex.value + 1, items.length - 1);
      scrollToSelected();
      break;
    case 'ArrowUp':
      event.preventDefault();
      selectedIndex.value = Math.max(selectedIndex.value - 1, 0);
      scrollToSelected();
      break;
    case 'Enter':
      event.preventDefault();
      if (items[selectedIndex.value]) {
        handleItemClick(items[selectedIndex.value]);
      }
      break;
    case 'Escape':
      searchQuery.value = '';
      loadHistory();
      break;
    case '1': case '2': case '3': case '4': case '5':
    case '6': case '7': case '8': case '9':
      const num = parseInt(event.key);
      if (num <= items.length) {
        handleItemClick(items[num - 1]);
      }
      break;
  }
};

// Scroll to selected item
const scrollToSelected = () => {
  if (!listRef.value) return;
  const items = listRef.value.querySelectorAll('.clipboard-item');
  if (items[selectedIndex.value]) {
    items[selectedIndex.value].scrollIntoView({ block: 'nearest' });
  }
};

// Item click handler
const handleItemClick = async (item: ClipboardItemType) => {
  await restoreToClipboard(item);
};

// Item double click handler
const handleItemDoubleClick = async (item: ClipboardItemType) => {
  await restoreToClipboard(item);
  // Hide window after paste (optional, depends on settings)
};

// Item right click handler
const handleItemRightClick = (event: MouseEvent, item: ClipboardItemType) => {
  // TODO: Show context menu
  console.log('Right click:', event, item);
};

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown);
});
</script>

<template>
  <div class="clipboard-list">
    <!-- Search Bar -->
    <div class="search-bar">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="搜索剪贴板历史..."
        @keyup.enter="handleSearch"
      />
      <button @click="handleSearch">搜索</button>
    </div>

    <!-- Tabs -->
    <div class="tabs">
      <button 
        :class="{ active: activeTab === 'all' }" 
        @click="setTab('all')"
      >
        全部 ({{ tabCounts.all }})
      </button>
      <button 
        :class="{ active: activeTab === 'text' }" 
        @click="setTab('text')"
      >
        文本 ({{ tabCounts.text }})
      </button>
      <button 
        :class="{ active: activeTab === 'image' }" 
        @click="setTab('image')"
      >
        图片 ({{ tabCounts.image }})
      </button>
      <button 
        :class="{ active: activeTab === 'file' }" 
        @click="setTab('file')"
      >
        文件 ({{ tabCounts.file }})
      </button>
    </div>

    <!-- List -->
    <div class="list-container" ref="listRef">
      <ClipboardItem
        v-for="(item, index) in filteredItems"
        :key="item.id"
        :item="item"
        :index="index"
        :is-selected="index === selectedIndex"
        @click="handleItemClick(item)"
        @double-click="handleItemDoubleClick(item)"
        @right-click="handleItemRightClick"
      />
      <div v-if="filteredItems.length === 0" class="empty-state">
        <p>暂无剪贴板历史</p>
        <p class="hint">复制一些内容后会显示在这里</p>
      </div>
    </div>

    <!-- Status Bar -->
    <div class="status-bar">
      共 {{ filteredItems.length }} 条记录
    </div>
  </div>
</template>

<style scoped>
.clipboard-list {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.search-bar {
  display: flex;
  gap: 8px;
  padding: 12px 16px;
  border-bottom: 1px solid #f0f0f0;
}

.search-bar input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-size: 14px;
}

.search-bar input:focus {
  border-color: #1890ff;
  outline: none;
}

.search-bar button {
  padding: 8px 16px;
  background-color: #1890ff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.search-bar button:hover {
  background-color: #40a9ff;
}

.tabs {
  display: flex;
  gap: 4px;
  padding: 8px 16px;
  border-bottom: 1px solid #f0f0f0;
}

.tabs button {
  padding: 6px 12px;
  background-color: transparent;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  color: #666;
}

.tabs button:hover {
  background-color: #f5f5f5;
}

.tabs button.active {
  background-color: #e6f7ff;
  color: #1890ff;
}

.list-container {
  flex: 1;
  overflow-y: auto;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 48px;
  color: #999;
}

.empty-state p {
  margin: 0;
}

.empty-state .hint {
  font-size: 12px;
  margin-top: 8px;
}

.status-bar {
  padding: 8px 16px;
  border-top: 1px solid #f0f0f0;
  font-size: 12px;
  color: #999;
}
</style>
