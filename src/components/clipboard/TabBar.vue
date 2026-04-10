<template>
  <div class="list-header">
    <div class="tabs">
      <button
        v-for="(tab, index) in allTabs"
        :key="tab.key"
        class="tab-btn"
        :class="{
          active: isTabActive(tab),
          'is-pinned': tab.isPinned,
          'is-fixed': !tab.isPinned,
        }"
        :draggable="tab.isPinned"
        @click="tab.isPinned ? handlePinnedClick(tab.key) : handleTabClick(tab.key)"
        @dragstart="handleDragStart($event, index)"
        @dragover="handleDragOver($event, index)"
        @drop="handleDrop($event, index)"
        @dragend="handleDragEnd"
      >
        <span class="tab-label">{{ tab.label }}</span>
        <span
          v-if="tab.isPinned"
          class="tab-close"
          @click.stop="handleUnpin(tab.key)"
          title="取消固定"
        >×</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import type { PinnedSearch } from '@/types';

interface Tab {
  key: string;
  label: string;
  isPinned: boolean;
  query?: string;
}

interface Props {
  activeFixedTab: string;
  activePinnedIds: string[];
  fixedTabs: { key: string; label: string }[];
  pinnedSearches: PinnedSearch[];
}

const props = defineProps<Props>();

const emit = defineEmits<{
  'tab-click': [key: string];
  'pinned-click': [id: string];
  'unpin': [id: string];
  'reorder': [fromIndex: number, toIndex: number];
}>();

const dragIndex = ref<number | null>(null);

const allTabs = computed<Tab[]>(() => {
  const fixedTabs = props.fixedTabs.map(tab => ({ ...tab, isPinned: false as const }));
  const pinnedTabs = props.pinnedSearches.map(ps => ({
    key: ps.id,
    label: ps.label,
    isPinned: true as const,
    query: ps.query,
  }));
  return [...fixedTabs, ...pinnedTabs];
});

const isTabActive = (tab: Tab) => {
  if (tab.isPinned) {
    return props.activePinnedIds.includes(tab.key);
  }
  return props.activeFixedTab === tab.key;
};

const handleTabClick = (key: string) => {
  emit('tab-click', key);
};

const handlePinnedClick = (id: string) => {
  emit('pinned-click', id);
};

const handleUnpin = (key: string) => {
  emit('unpin', key);
};

const handleDragStart = (event: DragEvent, index: number) => {
  // 不允许拖拽固定标签
  if (index < props.fixedTabs.length) {
    event.preventDefault();
    return;
  }
  dragIndex.value = index;
  if (event.dataTransfer) {
    event.dataTransfer.effectAllowed = 'move';
  }
};

const handleDragOver = (event: DragEvent, _index: number) => {
  event.preventDefault();
  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = 'move';
  }
};

const handleDrop = (event: DragEvent, dropIndex: number) => {
  event.preventDefault();
  if (dragIndex.value === null) return;

  const fromIndex = dragIndex.value;
  const toIndex = dropIndex;

  // 不允许放置到固定标签区域
  if (toIndex < props.fixedTabs.length) return;
  // 不允许从固定标签区域拖拽
  if (fromIndex < props.fixedTabs.length) return;

  // 计算在 pinnedSearches 数组中的索引
  const pinnedFromIndex = fromIndex - props.fixedTabs.length;
  const pinnedToIndex = toIndex - props.fixedTabs.length;

  emit('reorder', pinnedFromIndex, pinnedToIndex);
  dragIndex.value = null;
};

const handleDragEnd = () => {
  dragIndex.value = null;
};
</script>

<style scoped>
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
  background: #f5f5f5;
  color: #595959;
  border: 1px solid #d9d9d9;
}

.tab-btn.is-pinned:hover {
  background: #e8e8e8;
}

.tab-btn.is-pinned.active {
  background: #262626;
  color: #fff;
  border-color: #262626;
}

.tab-btn.is-fixed {
  cursor: pointer;
}

.tab-btn.is-fixed:hover {
  background: #f5f5f5;
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
</style>
