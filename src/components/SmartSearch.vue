<template>
  <div class="smart-search" ref="searchContainerRef">
    <div class="search-input-wrapper" :class="{ focused: isFocused }" @click="focusInput">
      <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="8"/>
        <path d="M21 21l-4.35-4.35"/>
      </svg>
      
      <!-- 搜索内容区域 -->
      <div class="search-content" ref="contentRef">
        <!-- 已添加的 tags -->
        <span 
          v-for="(tag, index) in selectedTags" 
          :key="index"
          class="search-tag"
          :class="{ 'is-type': tag.type === 'type' }"
        >
          @{{ tag.value }}
          <button class="tag-remove" @click.stop="removeTag(index)">×</button>
        </span>
        
        <!-- 输入框 -->
        <input
          ref="inputRef"
          v-model="inputText"
          type="text"
          :placeholder="selectedTags.length ? '' : '搜索剪贴板... 使用 @标签名 或 @类型'"
          @focus="handleFocus"
          @blur="handleBlur"
          @keydown="handleKeyDown"
          @input="handleInput"
          @compositionstart="isComposing = true"
          @compositionend="handleCompositionEnd"
        />
      </div>
      
      <!-- 清除按钮 -->
      <button v-if="hasContent" class="clear-btn" @click.stop="clearAll">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="18" y1="6" x2="6" y2="18"/>
          <line x1="6" y1="6" x2="18" y2="18"/>
        </svg>
      </button>

      <!-- 快捷键提示 -->
      <span v-else class="shortcut-hint">Ctrl + F</span>
    </div>

    <!-- @补全面板 -->
    <div v-if="showMentionPanel" class="search-dropdown" @mousedown.prevent>
      <div class="dropdown-header">
        <span>{{ mentionFilter ? `匹配 "${mentionFilter}"` : '选择标签或类型' }}</span>
        <span class="hint">↑↓←→ 选择 · Enter 确认 · Esc 关闭</span>
      </div>
      <div class="mention-grid" v-if="filteredMentionOptions.length > 0">
        <div
          v-for="(item, index) in filteredMentionOptions"
          :key="item.value"
          class="mention-cell"
          :class="{ 
            active: selectedMentionIndex === index,
            'is-type': item.category === 'type'
          }"
          :style="item.category === 'tag' ? getTagStyle(item.value) : {}"
          @click="selectMention(item)"
          @mouseenter="selectedMentionIndex = index"
        >
          <span class="mention-icon">@</span>
          <span class="mention-name">{{ item.value }}</span>
          <span v-if="item.count > 0" class="mention-count">{{ item.count }}</span>
        </div>
      </div>
      <div v-else class="dropdown-empty">未找到匹配项</div>
    </div>

    <!-- 历史记录面板 (已屏蔽) -->
    <!-- <div v-else-if="showHistory" class="search-dropdown" @mousedown.prevent>
      <div class="dropdown-header">
        <span>最近搜索</span>
        <button v-if="props.history.length" class="clear-btn-text" @click="clearAllHistory">清空</button>
      </div>
      <div class="history-list">
        <div
          v-for="(h, i) in props.history"
          :key="h"
          class="history-item"
          :class="{ active: selectedHistoryIndex === i }"
          @click="selectHistory(h)"
          @mouseenter="selectedHistoryIndex = i"
        >
          <span class="history-text">{{ h }}</span>
          <button class="history-delete" @click.stop="$emit('clear-history', h)">×</button>
        </div>
        <div v-if="!props.history.length" class="dropdown-empty">暂无搜索历史</div>
      </div>
    </div> -->
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick, onUnmounted, watch } from 'vue';
import { getTagStyle } from '@/utils/tagColors';
import type { ClipboardItem } from '@/types';

interface Props {
  modelValue: string;
  history: string[];
  items: ClipboardItem[];
}

const props = defineProps<Props>();

const emit = defineEmits<{
  'update:modelValue': [string];
  search: [query: string];
  'search-commit': [query: string];
  'clear-history': [string];
  'clear-all-history': [];
}>();

// Refs
const inputRef = ref<HTMLInputElement>();
const contentRef = ref<HTMLElement>();
const searchContainerRef = ref<HTMLElement>();

// State
const isFocused = ref(false);
const inputText = ref('');
const selectedTags = ref<{ value: string; type: 'type' | 'tag' }[]>([]);

// Dropdown states
const showMentionPanel = ref(false);
const showHistory = ref(false);
const mentionFilter = ref('');
const selectedMentionIndex = ref(0);
const selectedHistoryIndex = ref(-1);
const isComposing = ref(false);

// Usage tracking
const mentionUsageCount = ref<Record<string, number>>({});

// Constants
const GRID_COLS = 3;
const TYPE_OPTIONS = ['文本', 'html', '图片', '文件', '文件夹'];

// Computed
const hasContent = computed(() => selectedTags.value.length > 0 || inputText.value.length > 0);

const allTags = computed(() => {
  const tags = new Set<string>();
  props.items.forEach(item => item.tags?.forEach(t => tags.add(t)));
  return Array.from(tags);
});

const mentionOptions = computed(() => {
  const options: { value: string; category: 'type' | 'tag'; count: number }[] = [];
  
  TYPE_OPTIONS.forEach(type => {
    options.push({
      value: type,
      category: 'type',
      count: mentionUsageCount.value[type] || 0
    });
  });
  
  allTags.value.forEach(tag => {
    options.push({
      value: tag,
      category: 'tag',
      count: mentionUsageCount.value[tag] || 0
    });
  });
  
  return options;
});

const filteredMentionOptions = computed(() => {
  let options = mentionOptions.value;
  
  // Filter out already selected tags
  const selectedValues = selectedTags.value.map(t => t.value);
  options = options.filter(o => !selectedValues.includes(o.value));
  
  // Filter by input
  if (mentionFilter.value) {
    const f = mentionFilter.value.toLowerCase();
    options = options.filter(o => o.value.toLowerCase().includes(f));
  }
  
  // Sort by usage count
  return options.sort((a, b) => {
    if (b.count !== a.count) return b.count - a.count;
    return a.value.localeCompare(b.value);
  });
});

// Methods
const focusInput = () => {
  inputRef.value?.focus();
};

const handleFocus = () => {
  isFocused.value = true;
  // 检查是否需要显示@补全面板
  checkMention();
  // 历史记录功能已屏蔽
  // if (!showMentionPanel.value && props.history.length > 0) {
  //   showHistory.value = true;
  //   selectedHistoryIndex.value = -1;
  // }
};

// Track last committed query to avoid duplicate history entries
const lastCommittedQuery = ref('');

let blurTimer: number | null = null;

const handleBlur = () => {
  // Commit search on blur if content changed
  const fullQuery = selectedTags.value.map(t => `@${t.value}`).join(' ') +
    (selectedTags.value.length && inputText.value ? ' ' : '') + inputText.value;
  if (fullQuery.trim() && fullQuery !== lastCommittedQuery.value) {
    lastCommittedQuery.value = fullQuery;
    emit('search-commit', fullQuery);
  }

  blurTimer = window.setTimeout(() => {
    isFocused.value = false;
    showMentionPanel.value = false;
    showHistory.value = false;
  }, 150);
};

const clearBlurTimer = () => {
  if (blurTimer) {
    clearTimeout(blurTimer);
    blurTimer = null;
  }
};

const checkMention = () => {
  const text = inputText.value;
  const lastAt = text.lastIndexOf('@');

  if (lastAt === -1) {
    // 没有@符号时，关闭补全面板
    showMentionPanel.value = false;
    // 历史记录功能已屏蔽
    // if (!text.trim() && props.history.length > 0 && isFocused.value) {
    //   showHistory.value = true;
    // }
    return;
  }

  // Check if there's a space after @ (meaning user finished typing)
  const afterAt = text.slice(lastAt + 1);
  if (afterAt.includes(' ')) {
    showMentionPanel.value = false;
    // 历史记录功能已屏蔽
    // const beforeAt = text.slice(0, lastAt).trim();
    // if (!beforeAt && props.history.length > 0 && isFocused.value) {
    //   showHistory.value = true;
    // }
    return;
  }

  // Check @ is at start or after space
  if (lastAt > 0 && text[lastAt - 1] !== ' ') {
    showMentionPanel.value = false;
    return;
  }

  // 显示@补全面板，隐藏历史记录
  mentionFilter.value = afterAt;
  showMentionPanel.value = true;
  showHistory.value = false;
  selectedMentionIndex.value = 0;
};

const handleInput = () => {
  if (isComposing.value) return;
  checkMention();
  updateSearchQuery();
  // 如果有输入内容（非@标签），关闭历史记录
  const text = inputText.value.trim();
  if (text && !showMentionPanel.value) {
    showHistory.value = false;
  }
};

const handleCompositionEnd = () => {
  isComposing.value = false;
  checkMention();
  updateSearchQuery();
  // 如果有输入内容（非@标签），关闭历史记录
  const text = inputText.value.trim();
  if (text && !showMentionPanel.value) {
    showHistory.value = false;
  }
};

const selectMention = (item: { value: string; category: 'type' | 'tag' }) => {
  clearBlurTimer();

  // Remove the @ and filter text from input
  const text = inputText.value;
  const lastAt = text.lastIndexOf('@');
  inputText.value = text.slice(0, lastAt);

  // Add to selected tags
  selectedTags.value.push({
    value: item.value,
    type: item.category
  });

  // Record usage
  mentionUsageCount.value[item.value] = (mentionUsageCount.value[item.value] || 0) + 1;

  showMentionPanel.value = false;
  // Commit search when selecting a mention
  updateSearchQuery(true);

  nextTick(() => {
    inputRef.value?.focus();
  });
};

const removeTag = (index: number) => {
  selectedTags.value.splice(index, 1);
  updateSearchQuery();
  inputRef.value?.focus();
};

const selectHistory = (h: string) => {
  clearBlurTimer();

  // Parse history to extract tags and text
  const parts = h.split(/\s+/);
  const newTags: { value: string; type: 'type' | 'tag' }[] = [];
  let remainingText = '';

  parts.forEach(part => {
    if (part.startsWith('@')) {
      const value = part.slice(1);
      const isType = TYPE_OPTIONS.includes(value);
      newTags.push({
        value,
        type: isType ? 'type' : 'tag'
      });
    } else {
      remainingText += (remainingText ? ' ' : '') + part;
    }
  });

  selectedTags.value = newTags;
  inputText.value = remainingText;
  showHistory.value = false;

  // 如果有剩余文本，需要检查是否有新的@需要补全
  nextTick(() => {
    checkMention();
    updateSearchQuery();
    inputRef.value?.focus();
  });
};

// 历史记录功能已屏蔽
// const clearAllHistory = () => {
//   emit('clear-all-history');
// };

const clearAll = () => {
  selectedTags.value = [];
  inputText.value = '';
  updateSearchQuery();
  inputRef.value?.focus();
};

const updateSearchQuery = (shouldCommit = false) => {
  const tags = selectedTags.value.map(t => `@${t.value}`).join(' ');
  const fullQuery = tags + (tags && inputText.value ? ' ' : '') + inputText.value;
  emit('update:modelValue', fullQuery);
  emit('search', fullQuery);
  if (shouldCommit && fullQuery.trim()) {
    emit('search-commit', fullQuery);
  }
};

// Grid navigation
const navigateGrid = (direction: 'up' | 'down' | 'left' | 'right') => {
  const currentRow = Math.floor(selectedMentionIndex.value / GRID_COLS);
  const currentCol = selectedMentionIndex.value % GRID_COLS;
  const totalRows = Math.ceil(filteredMentionOptions.value.length / GRID_COLS);
  
  let newRow = currentRow;
  let newCol = currentCol;
  
  switch (direction) {
    case 'up':
      newRow = Math.max(0, currentRow - 1);
      break;
    case 'down':
      newRow = Math.min(totalRows - 1, currentRow + 1);
      break;
    case 'left':
      newCol = Math.max(0, currentCol - 1);
      break;
    case 'right':
      newCol = Math.min(GRID_COLS - 1, currentCol + 1);
      break;
  }
  
  const newIndex = newRow * GRID_COLS + newCol;
  if (newIndex < filteredMentionOptions.value.length) {
    selectedMentionIndex.value = newIndex;
  }
};

const handleKeyDown = (e: KeyboardEvent) => {
  // 快捷键：Ctrl+Shift+H 切换历史记录显示（已屏蔽）
  // if (e.ctrlKey && e.shiftKey && e.key.toLowerCase() === 'h') {
  //   e.preventDefault();
  //   e.stopPropagation();
  //   if (props.history.length > 0) {
  //     if (showMentionPanel.value || showHistory.value) {
  //       showMentionPanel.value = false;
  //       showHistory.value = false;
  //     } else {
  //       showMentionPanel.value = false;
  //       showHistory.value = true;
  //       selectedHistoryIndex.value = 0;
  //     }
  //   }
  //   return;
  // }
  
  // Mention panel navigation
  if (showMentionPanel.value) {
    e.stopPropagation();
    
    switch (e.key) {
      case 'ArrowDown':
        e.preventDefault();
        navigateGrid('down');
        break;
      case 'ArrowUp':
        e.preventDefault();
        navigateGrid('up');
        break;
      case 'ArrowLeft':
        e.preventDefault();
        navigateGrid('left');
        break;
      case 'ArrowRight':
        e.preventDefault();
        navigateGrid('right');
        break;
      case 'Enter':
        e.preventDefault();
        if (selectedMentionIndex.value < filteredMentionOptions.value.length) {
          selectMention(filteredMentionOptions.value[selectedMentionIndex.value]);
        }
        break;
      case 'Escape':
        e.preventDefault();
        showMentionPanel.value = false;
        break;
    }
    return;
  }
  
  // History navigation
  if (showHistory.value) {
    if (e.key === 'ArrowDown' || e.key === 'ArrowUp' || e.key === 'Enter') {
      e.stopPropagation();
    }
    
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      if (selectedHistoryIndex.value < props.history.length - 1) {
        selectedHistoryIndex.value++;
      }
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      if (selectedHistoryIndex.value > 0) {
        selectedHistoryIndex.value--;
      }
    } else if (e.key === 'Enter' && selectedHistoryIndex.value >= 0) {
      e.preventDefault();
      selectHistory(props.history[selectedHistoryIndex.value]);
    } else if (e.key === 'Escape') {
      showHistory.value = false;
    }
    return;
  }
  
  // Backspace to delete last tag when input is empty
  if (e.key === 'Backspace' && !inputText.value && selectedTags.value.length > 0) {
    removeTag(selectedTags.value.length - 1);
  }

  // 普通输入模式下按 Enter，提交搜索
  if (e.key === 'Enter' && !showMentionPanel.value && !showHistory.value) {
    const fullQuery = selectedTags.value.map(t => `@${t.value}`).join(' ') +
      (selectedTags.value.length && inputText.value ? ' ' : '') + inputText.value;
    if (fullQuery.trim()) {
      emit('search-commit', fullQuery);
    }
    showHistory.value = false;
  }
};

// 监听 modelValue 变化，同步更新内部状态（用于外部如标签点击时更新）
watch(() => props.modelValue, (newValue) => {
  // 解析 query 为 tags 和 inputText
  const parts = newValue.split(/\s+/);
  const newTags: { value: string; type: 'type' | 'tag' }[] = [];
  let remainingText = '';

  parts.forEach(part => {
    if (part.startsWith('@')) {
      const value = part.slice(1);
      // 忽略只有 @ 没有内容的情况（用户正在输入）
      if (!value) return;
      const isType = TYPE_OPTIONS.includes(value);
      newTags.push({
        value,
        type: isType ? 'type' : 'tag'
      });
    } else {
      remainingText += (remainingText ? ' ' : '') + part;
    }
  });

  selectedTags.value = newTags;
  inputText.value = remainingText;
}, { immediate: true });

onUnmounted(() => {
  if (blurTimer) clearTimeout(blurTimer);
});

defineExpose({ focus: focusInput });
</script>

<style scoped>
.smart-search {
  position: relative;
  width: 100%;
}

.search-input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  background: #f5f5f5;
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 4px 8px;
  min-height: 40px;
  cursor: text;
}

.search-input-wrapper.focused {
  background: #fff;
  border-color: #262626;
}

.search-icon {
  width: 16px;
  height: 16px;
  color: #bfbfbf;
  flex-shrink: 0;
  margin-right: 8px;
}

.search-content {
  flex: 1;
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 4px;
  min-width: 0;
}

/* Tags */
.search-tag {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 2px 8px;
  background: #e6f7ff;
  border: 1px solid #91d5ff;
  border-radius: 4px;
  font-size: 13px;
  color: #1890ff;
  font-weight: 500;
}

.search-tag.is-type {
  background: #f6ffed;
  border-color: #b7eb8f;
  color: #52c41a;
}

.tag-remove {
  width: 14px;
  height: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.1);
  border: none;
  border-radius: 50%;
  cursor: pointer;
  font-size: 10px;
  color: inherit;
  padding: 0;
}

.tag-remove:hover {
  background: rgba(0, 0, 0, 0.2);
}

/* Input */
input {
  flex: 1;
  min-width: 60px;
  padding: 4px;
  border: none;
  background: transparent;
  font-size: 13px;
  outline: none;
  color: #262626;
}

input::placeholder {
  color: #8c8c8c;
}

/* Clear button */
.clear-btn {
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #d9d9d9;
  border: none;
  border-radius: 50%;
  cursor: pointer;
  color: #fff;
  flex-shrink: 0;
  margin-left: 8px;
  padding: 0;
}

.clear-btn:hover {
  background: #bfbfbf;
}

.clear-btn svg {
  width: 12px;
  height: 12px;
}

/* Shortcut hint */
.shortcut-hint {
  font-size: 11px;
  color: #595959;
  background: #e6f7ff;
  padding: 3px 8px;
  border-radius: 4px;
  border: 1px solid #91d5ff;
  flex-shrink: 0;
  margin-left: 8px;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
  white-space: nowrap;
  font-weight: 500;
}

/* Dropdown */
.search-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  margin-top: 4px;
  background: #fff;
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  border: 1px solid #f0f0f0;
  overflow: hidden;
}

.dropdown-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  font-size: 12px;
  color: #8c8c8c;
  border-bottom: 1px solid #f5f5f5;
}

.hint {
  font-size: 11px;
  color: #bfbfbf;
}

.clear-btn-text {
  font-size: 11px;
  color: #8c8c8c;
  background: transparent;
  border: none;
  cursor: pointer;
  padding: 2px 6px;
}

.clear-btn-text:hover {
  color: #ff4d4f;
}

.dropdown-empty {
  padding: 20px;
  text-align: center;
  color: #bfbfbf;
  font-size: 13px;
}

/* Mention grid */
.mention-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 6px;
  padding: 8px;
  max-height: 200px;
  overflow-y: auto;
}

.mention-cell {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 10px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  background: #fafafa;
  border: 1px solid transparent;
  transition: all 0.15s;
  min-height: 36px;
}

/* 类型默认灰色背景 */
.mention-cell.is-type {
  background: #f5f5f5;
}

/* hover 效果 - 仅加深边框，不改变背景 */
.mention-cell:hover {
  border-color: #d9d9d9;
  box-shadow: 0 2px 4px rgba(0,0,0,0.05);
}

/* active/选中状态 - 深色边框 */
.mention-cell.active {
  border-color: #262626;
  box-shadow: 0 0 0 1px #262626;
}

.mention-icon {
  font-weight: 600;
  opacity: 0.6;
  font-size: 12px;
}

.mention-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.mention-count {
  font-size: 10px;
  padding: 1px 5px;
  background: rgba(0, 0, 0, 0.1);
  border-radius: 8px;
  opacity: 0.7;
}

.mention-cell.active .mention-count {
  background: rgba(255, 255, 255, 0.2);
}

/* History list */
.history-list {
  max-height: 180px;
  overflow-y: auto;
}

.history-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  cursor: pointer;
  font-size: 13px;
  transition: background 0.15s;
}

.history-item:hover,
.history-item.active {
  background: #f5f5f5;
}

.history-text {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.history-delete {
  background: transparent;
  border: none;
  color: #bfbfbf;
  cursor: pointer;
  font-size: 14px;
  padding: 2px 6px;
}

.history-delete:hover {
  color: #ff4d4f;
}
</style>
