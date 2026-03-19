<template>
  <div class="smart-search" ref="searchContainerRef">
    <div class="search-input-wrapper" :class="{ focused: isFocused }" @click="focusInput">
      <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="8"/>
        <path d="M21 21l-4.35-4.35"/>
      </svg>

      <!-- 搜索内容区域 - 使用 contenteditable -->
      <div
        ref="editorRef"
        class="search-editor"
        contenteditable="true"
        :placeholder="placeholderText"
        @focus="handleFocus"
        @blur="handleBlur"
        @keydown="handleKeyDown"
        @input="handleInput"
        @compositionstart="isComposing = true"
        @compositionend="handleCompositionEnd"
        @click="handleEditorClick"
      ></div>

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

    <!-- @补全面板 - 跟随光标位置 -->
    <div
      v-if="showMentionPanel"
      class="search-dropdown"
      :style="mentionPanelStyle"
      @mousedown.prevent
    >
      <div class="dropdown-header">
        <span>{{ mentionFilter ? `匹配 "${mentionFilter}"` : '选择标签或类型' }}</span>
        <span class="hint">↑↓ 选择 · Enter 确认 · Esc 关闭</span>
      </div>
      <div ref="mentionListRef" class="mention-list" v-if="filteredMentionOptions.length > 0">
        <div
          v-for="(item, index) in filteredMentionOptions"
          :key="item.value"
          class="mention-item"
          :class="{
            active: selectedMentionIndex === index,
            'is-type': item.category === 'type'
          }"
          @click="selectMention(item)"
          @mouseenter="selectedMentionIndex = index"
        >
          <span class="mention-item-icon">{{ item.category === 'tag' ? '#' : getMentionIcon(item) }}</span>
          <span class="mention-item-name">{{ item.value }}</span>
          <span class="mention-item-meta">
            {{ item.category === 'type' ? '类型' : `${item.count || 0}` }}
          </span>
        </div>
      </div>
      <div v-else class="dropdown-empty">未找到匹配项</div>
    </div>
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
const editorRef = ref<HTMLDivElement>();
const searchContainerRef = ref<HTMLElement>();

// State
const isFocused = ref(false);
const isComposing = ref(false);

// Mention panel states
const showMentionPanel = ref(false);
const mentionFilter = ref('');
const selectedMentionIndex = ref(0);
const mentionPanelPosition = ref({ left: 0, top: 0 });

// Usage tracking
const mentionUsageCount = ref<Record<string, number>>({});

// Constants
const TYPE_OPTIONS = ['文本', 'html', '图片', '文件', '文件夹'];

// Track current mention range
const currentMentionRange = ref<Range | null>(null);
const currentMentionNode = ref<Text | null>(null);

// Track last committed query
const lastCommittedQuery = ref('');

// Computed
const hasContent = computed(() => {
  const text = getTextContent();
  return text.trim().length > 0;
});

const placeholderText = computed(() => {
  const text = getTextContent();
  return text.trim() ? '' : '搜索剪贴板... 使用 @标签名 或 @类型';
});

const mentionPanelStyle = computed(() => ({
  position: 'fixed' as const,
  left: `${mentionPanelPosition.value.left}px`,
  top: `${mentionPanelPosition.value.top}px`,
  zIndex: 1000,
}));

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

  // Filter out already selected tags (check in current text)
  const existingTags = getExistingTags();
  options = options.filter(o => !existingTags.includes(o.value));

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

// Get existing tags from editor content
function getExistingTags(): string[] {
  const tags: string[] = [];
  const editor = editorRef.value;
  if (!editor) return tags;

  const spans = editor.querySelectorAll('.inline-tag');
  spans.forEach(span => {
    const tagName = span.getAttribute('data-tag');
    if (tagName) tags.push(tagName);
  });
  return tags;
}

// Get plain text content from editor
function getTextContent(): string {
  const editor = editorRef.value;
  if (!editor) return '';

  let text = '';
  const walk = (node: Node) => {
    if (node.nodeType === Node.TEXT_NODE) {
      text += node.textContent || '';
    } else if (node.nodeType === Node.ELEMENT_NODE) {
      const el = node as HTMLElement;
      if (el.classList.contains('inline-tag')) {
        const tagName = el.getAttribute('data-tag');
        if (tagName) text += `@${tagName}`;
      } else {
        node.childNodes.forEach(walk);
      }
      if (el.tagName === 'DIV' && el !== editor) {
        text += '\n';
      }
    }
  };

  editor.childNodes.forEach(walk);
  return text;
}

// Set editor content from parsed tags and text
function setEditorContent(tags: { value: string; type: 'type' | 'tag' }[], text: string) {
  const editor = editorRef.value;
  if (!editor) return;

  editor.innerHTML = '';

  // Add tags as inline elements
  tags.forEach(tag => {
    const span = createInlineTagElement(tag.value, tag.type);
    editor.appendChild(span);
    editor.appendChild(document.createTextNode('\u00A0')); // Non-breaking space after tag
  });

  // Add remaining text
  if (text) {
    const textNode = document.createTextNode(text);
    editor.appendChild(textNode);
  }

  // Place cursor at the end
  nextTick(() => {
    const range = document.createRange();
    const sel = window.getSelection();
    if (editor.lastChild) {
      range.setStartAfter(editor.lastChild);
      range.collapse(true);
      sel?.removeAllRanges();
      sel?.addRange(range);
    }
  });
}

// Create inline tag element
function createInlineTagElement(value: string, type: 'type' | 'tag'): HTMLSpanElement {
  const span = document.createElement('span');
  span.className = `inline-tag ${type === 'type' ? 'is-type' : ''}`;
  span.contentEditable = 'false';
  span.setAttribute('data-tag', value);
  span.setAttribute('data-type', type);
  span.textContent = `@${value}`;
  return span;
}

// Get caret position in text (character index)
function getCaretPosition(): number {
  const editor = editorRef.value;
  if (!editor) return 0;

  const sel = window.getSelection();
  if (!sel || sel.rangeCount === 0) return 0;

  const range = sel.getRangeAt(0);
  let pos = 0;

  const walk = (node: Node, targetNode: Node, targetOffset: number): boolean => {
    if (node === targetNode) {
      pos += targetOffset;
      return true;
    }

    if (node.nodeType === Node.TEXT_NODE) {
      pos += node.textContent?.length || 0;
    } else if (node.nodeType === Node.ELEMENT_NODE) {
      const el = node as HTMLElement;
      if (el.classList.contains('inline-tag')) {
        const tagName = el.getAttribute('data-tag');
        if (tagName) pos += `@${tagName}`.length;
      } else {
        for (let i = 0; i < node.childNodes.length; i++) {
          if (walk(node.childNodes[i], targetNode, targetOffset)) return true;
        }
      }
    }
    return false;
  };

  walk(editor, range.startContainer, range.startOffset);
  return pos;
}

// Get caret coordinates for positioning dropdown
function getCaretCoordinates(): { left: number; top: number } | null {
  const sel = window.getSelection();
  if (!sel || sel.rangeCount === 0) return null;

  const range = sel.getRangeAt(0);
  const rect = range.getBoundingClientRect();

  if (rect.width === 0 && rect.height === 0) {
    // If collapsed range has no dimensions, try to get from range's client rects
    const rects = range.getClientRects();
    if (rects.length > 0) {
      const lastRect = rects[rects.length - 1];
      return {
        left: lastRect.left,
        top: lastRect.bottom + 4
      };
    }
  }

  return {
    left: rect.left,
    top: rect.bottom + 4
  };
}

// Check if there's an active @ mention
function checkMention() {
  const sel = window.getSelection();
  if (!sel || sel.rangeCount === 0) {
    showMentionPanel.value = false;
    return;
  }

  const range = sel.getRangeAt(0);
  const node = range.startContainer;

  // Only check text nodes
  if (node.nodeType !== Node.TEXT_NODE) {
    showMentionPanel.value = false;
    return;
  }

  const text = node.textContent || '';
  const offset = range.startOffset;

  // Find the last @ before cursor
  const textBeforeCursor = text.slice(0, offset);
  const lastAtIndex = textBeforeCursor.lastIndexOf('@');

  if (lastAtIndex === -1) {
    showMentionPanel.value = false;
    return;
  }

  // Check if @ is at start or after space
  if (lastAtIndex > 0 && textBeforeCursor[lastAtIndex - 1] !== ' ') {
    showMentionPanel.value = false;
    return;
  }

  // Check if there's a space after @ (meaning user finished typing)
  const afterAt = textBeforeCursor.slice(lastAtIndex + 1);
  if (afterAt.includes(' ')) {
    showMentionPanel.value = false;
    return;
  }

  // Show mention panel
  mentionFilter.value = afterAt;
  currentMentionNode.value = node as Text;

  // Save range for later replacement
  const mentionRange = document.createRange();
  mentionRange.setStart(node, lastAtIndex);
  mentionRange.setEnd(node, offset);
  currentMentionRange.value = mentionRange;

  showMentionPanel.value = true;
  selectedMentionIndex.value = 0;

  // Position panel
  nextTick(() => {
    const coords = getCaretCoordinates();
    if (coords) {
      mentionPanelPosition.value = coords;
    }
  });
}

// Select a mention and replace @xxx with inline tag
function selectMention(item: { value: string; category: 'type' | 'tag' }) {
  clearBlurTimer();

  const range = currentMentionRange.value;
  const node = currentMentionNode.value;
  const editor = editorRef.value;

  if (range && node && editor) {
    // Delete the @xxx text
    range.deleteContents();

    // Insert inline tag
    const tagSpan = createInlineTagElement(item.value, item.category);
    range.insertNode(tagSpan);

    // Add space after tag
    const spaceNode = document.createTextNode('\u00A0');
    tagSpan.after(spaceNode);

    // Move cursor after space
    const sel = window.getSelection();
    const newRange = document.createRange();
    newRange.setStartAfter(spaceNode);
    newRange.collapse(true);
    sel?.removeAllRanges();
    sel?.addRange(newRange);
  }

  // Record usage
  mentionUsageCount.value[item.value] = (mentionUsageCount.value[item.value] || 0) + 1;

  showMentionPanel.value = false;
  currentMentionRange.value = null;
  currentMentionNode.value = null;

  // Update search
  updateSearchQuery(true);

  nextTick(() => {
    editorRef.value?.focus();
  });
}

// Refs
const mentionListRef = ref<HTMLDivElement>();

// Navigation in mention list with scroll into view
const navigateMentionList = (direction: 'up' | 'down') => {
  if (direction === 'up') {
    selectedMentionIndex.value = Math.max(0, selectedMentionIndex.value - 1);
  } else {
    selectedMentionIndex.value = Math.min(
      filteredMentionOptions.value.length - 1,
      selectedMentionIndex.value + 1
    );
  }

  // Scroll active item into view
  nextTick(() => {
    const listEl = mentionListRef.value;
    if (!listEl) return;

    const activeItem = listEl.querySelector('.mention-item.active') as HTMLElement;
    if (!activeItem) return;

    const listRect = listEl.getBoundingClientRect();
    const itemRect = activeItem.getBoundingClientRect();

    // Check if item is above visible area
    if (itemRect.top < listRect.top) {
      activeItem.scrollIntoView({ block: 'start', behavior: 'smooth' });
    }
    // Check if item is below visible area
    else if (itemRect.bottom > listRect.bottom) {
      activeItem.scrollIntoView({ block: 'end', behavior: 'smooth' });
    }
  });
};

// Icon mapping
const getMentionIcon = (item: { value: string; category: 'type' | 'tag' }): string => {
  if (item.category === 'tag') {
    return '🏷️';
  }
  switch (item.value) {
    case '文本': return '📝';
    case 'html': return '🌐';
    case '图片': return '🖼️';
    case '文件': return '📄';
    case '文件夹': return '📁';
    default: return '📋';
  }
};

// Focus input
const focusInput = () => {
  editorRef.value?.focus();
};

// Handle focus
const handleFocus = () => {
  isFocused.value = true;
  checkMention();
};

// Handle blur
let blurTimer: number | null = null;

const clearBlurTimer = () => {
  if (blurTimer) {
    clearTimeout(blurTimer);
    blurTimer = null;
  }
};

const handleBlur = () => {
  // Commit search on blur
  const fullQuery = getTextContent();
  if (fullQuery.trim() && fullQuery !== lastCommittedQuery.value) {
    lastCommittedQuery.value = fullQuery;
    emit('search-commit', fullQuery);
  }

  blurTimer = window.setTimeout(() => {
    isFocused.value = false;
    showMentionPanel.value = false;
  }, 150);
};

// Handle input
const handleInput = () => {
  if (isComposing.value) return;
  checkMention();
  updateSearchQuery();
};

const handleCompositionEnd = () => {
  isComposing.value = false;
  checkMention();
  updateSearchQuery();
};

// Update search query
const updateSearchQuery = (shouldCommit = false) => {
  const fullQuery = getTextContent();

  emit('update:modelValue', fullQuery);
  if (fullQuery.trim()) {
    emit('search', fullQuery);
  }
  if (shouldCommit && fullQuery.trim()) {
    emit('search-commit', fullQuery);
  }
};

// Clear all content
const clearAll = () => {
  const editor = editorRef.value;
  if (editor) {
    editor.innerHTML = '';
  }
  updateSearchQuery();
  editorRef.value?.focus();
};

// Handle editor click - check if clicked on tag
const handleEditorClick = (e: MouseEvent) => {
  const target = e.target as HTMLElement;
  if (target.classList.contains('inline-tag')) {
    // Select the tag for potential deletion
    const range = document.createRange();
    const sel = window.getSelection();
    range.selectNode(target);
    sel?.removeAllRanges();
    sel?.addRange(range);
  }
};

// Handle keydown
const handleKeyDown = (e: KeyboardEvent) => {
  // Mention panel navigation
  if (showMentionPanel.value) {
    e.stopPropagation();

    switch (e.key) {
      case 'ArrowDown':
        e.preventDefault();
        navigateMentionList('down');
        break;
      case 'ArrowUp':
        e.preventDefault();
        navigateMentionList('up');
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
      case 'Tab':
        e.preventDefault();
        if (filteredMentionOptions.value.length > 0) {
          selectMention(filteredMentionOptions.value[selectedMentionIndex.value]);
        }
        break;
    }
    return;
  }

  // Handle Backspace to delete selected tag
  if (e.key === 'Backspace') {
    const sel = window.getSelection();
    if (sel && sel.rangeCount > 0) {
      const range = sel.getRangeAt(0);
      // Check if a tag is selected
      if (!range.collapsed) {
        const container = range.commonAncestorContainer;
        const tagSpan = container.nodeType === Node.ELEMENT_NODE
          ? container as HTMLElement
          : container.parentElement;

        if (tagSpan?.classList.contains('inline-tag')) {
          e.preventDefault();
          tagSpan.remove();
          updateSearchQuery();
          return;
        }
      }

      // Check if cursor is right after a tag
      if (range.collapsed && range.startOffset === 0) {
        const node = range.startContainer;
        const prevNode = node.previousSibling;
        if (prevNode && prevNode.nodeType === Node.ELEMENT_NODE) {
          const el = prevNode as HTMLElement;
          if (el.classList.contains('inline-tag')) {
            e.preventDefault();
            el.remove();
            updateSearchQuery();
            return;
          }
        }
      }
    }
  }

  // Enter to commit search
  if (e.key === 'Enter' && !e.shiftKey && !showMentionPanel.value) {
    e.preventDefault();
    const fullQuery = getTextContent();
    if (fullQuery.trim()) {
      emit('search-commit', fullQuery);
    }
  }
};

// Watch modelValue changes
watch(() => props.modelValue, (newValue) => {
  // Don't update if mention panel is open
  if (showMentionPanel.value) return;

  // Parse query to tags and text
  const parts = newValue.split(/\s+/);
  const newTags: { value: string; type: 'type' | 'tag' }[] = [];
  let remainingText = '';

  parts.forEach(part => {
    if (part.startsWith('@')) {
      const value = part.slice(1);
      // @ 后面紧跟空格表示搜索 "@" 符号，不是标签
      if (!value || newValue.includes('@ ')) {
        remainingText += (remainingText ? ' ' : '') + part;
        return;
      }
      const isType = TYPE_OPTIONS.includes(value);
      newTags.push({
        value,
        type: isType ? 'type' : 'tag'
      });
    } else {
      remainingText += (remainingText ? ' ' : '') + part;
    }
  });

  // Update editor content
  const currentText = getTextContent();
  if (currentText !== newValue) {
    setEditorContent(newTags, remainingText);
  }
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

/* Contenteditable Editor */
.search-editor {
  flex: 1;
  min-width: 60px;
  padding: 4px;
  border: none;
  background: transparent;
  font-size: 13px;
  outline: none;
  color: #262626;
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-word;
}

.search-editor:empty::before {
  content: attr(placeholder);
  color: #8c8c8c;
  pointer-events: none;
}

.search-editor:focus {
  outline: none;
}

/* Inline Tags */
:deep(.inline-tag) {
  display: inline-flex;
  align-items: center;
  gap: 2px;
  padding: 1px 6px;
  background: #e6f7ff;
  border: 1px solid #91d5ff;
  border-radius: 4px;
  font-size: 12px;
  color: #1890ff;
  font-weight: 500;
  margin: 0 2px;
  cursor: default;
  user-select: none;
  vertical-align: middle;
  line-height: 1.4;
}

:deep(.inline-tag.is-type) {
  background: #f6ffed;
  border-color: #b7eb8f;
  color: #52c41a;
}

:deep(.inline-tag:hover) {
  background: #bae7ff;
  border-color: #69c0ff;
}

:deep(.inline-tag.is-type:hover) {
  background: #d9f7be;
  border-color: #95de64;
}

:deep(.inline-tag.selected) {
  background: #ff4d4f;
  border-color: #ff7875;
  color: #fff;
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

/* Dropdown - Fixed position */
.search-dropdown {
  background: #ffffff;
  border-radius: 6px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
  border: 1px solid #f0f0f0;
  overflow: hidden;
  min-width: 200px;
  max-width: 280px;
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

.dropdown-empty {
  padding: 20px;
  text-align: center;
  color: #bfbfbf;
  font-size: 13px;
}

/* Mention list - 简洁风格 */
.mention-list {
  display: flex;
  flex-direction: column;
  padding: 4px 0;
  max-height: 240px;
  overflow-y: auto;
}

.mention-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 12px;
  margin: 0 4px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  color: #262626;
  background: transparent;
  transition: background 0.1s ease;
  min-height: 32px;
}

.mention-item:hover {
  background: #f5f5f5;
}

.mention-item.active {
  background: #262626;
  color: #fff;
}

.mention-item-icon {
  font-size: 14px;
  flex-shrink: 0;
  width: 18px;
  text-align: center;
  color: #8c8c8c;
}

.mention-item.active .mention-item-icon {
  color: #fff;
}

.mention-item-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-weight: 400;
}

.mention-item-meta {
  font-size: 11px;
  color: #bfbfbf;
  flex-shrink: 0;
  margin-left: auto;
  padding-left: 8px;
}

.mention-item.active .mention-item-meta {
  color: #d9d9d9;
}
</style>
