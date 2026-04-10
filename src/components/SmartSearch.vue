<template>
  <div class="smart-search" ref="searchContainerRef">
    <div class="search-input-wrapper" @click="focusInput">
      <!-- 搜索图标 -->
      <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="8"/>
        <path d="M21 21l-4.35-4.35"/>
      </svg>

      <!--
        搜索内容区域 - 使用 contenteditable 替代 input
        原因：支持内联标签渲染（@xxx 显示为彩色标签块）
        普通文本和标签可以混合编辑，类似 QQ/IDE 的 @mention 功能
      -->
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

      <!-- 清除按钮 - 有内容时显示 -->
      <button v-if="hasContent" class="clear-btn" @click.stop="clearAll">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="18" y1="6" x2="6" y2="18"/>
          <line x1="6" y1="6" x2="18" y2="18"/>
        </svg>
      </button>

      <!-- 快捷键提示 - 无内容时显示 -->
      <span v-else class="shortcut-hint">Ctrl + F</span>
    </div>

    <!--
      @补全面板 - 跟随光标位置显示
      使用 fixed 定位，跟随光标坐标
      按 ↑↓ 选择，Enter/Tab 确认，Esc 关闭
    -->
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
          <!-- 标签显示 #，类型显示对应图标 -->
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

/**
 * 防抖工具函数
 * 用于优化搜索性能，避免每输入一个字符都触发搜索
 * @param fn 要防抖的函数
 * @param delay 延迟时间（毫秒）
 */
function debounce<T extends (...args: Parameters<T>) => void>(
  fn: T,
  delay: number
): (...args: Parameters<T>) => void {
  let timer: number | null = null;
  return (...args: Parameters<T>) => {
    if (timer) clearTimeout(timer);
    timer = window.setTimeout(() => fn(...args), delay);
  };
}

// ============ Props & Emits ============

interface Props {
  modelValue: string;    // 当前搜索查询字符串（包含 @标签 和 普通文本）
  history: string[];     // 搜索历史（当前未使用）
  items: ClipboardItem[]; // 剪贴板项目列表，用于获取可用标签
}

const props = defineProps<Props>();

const emit = defineEmits<{
  'update:modelValue': [string];  // 更新查询字符串
  search: [query: string];        // 触发搜索（防抖后）
  'search-commit': [query: string]; // 提交搜索（Enter 或失焦时）
  'clear-history': [string];
  'clear-all-history': [];
}>();

// ============ Refs & State ============

const editorRef = ref<HTMLDivElement>();        // contenteditable 编辑器引用
const searchContainerRef = ref<HTMLElement>();  // 搜索容器引用
const mentionListRef = ref<HTMLDivElement>();   // 补全面板列表引用（用于滚动定位）

const isComposing = ref(false);    // 是否正在输入法组合（中文输入时需要）

// 补全面板状态
const showMentionPanel = ref(false);          // 是否显示补全面板
const mentionFilter = ref('');                  // 当前 @ 后的过滤文本
const selectedMentionIndex = ref(0);          // 当前选中的补全项索引
const mentionPanelPosition = ref({ left: 0, top: 0 }); // 面板位置坐标

// 标签使用统计（用于排序）
const mentionUsageCount = ref<Record<string, number>>({});

// 常量
const TYPE_OPTIONS = ['文本', 'html', '图片', '文件', '文件夹'];

// 当前正在输入的 @mention 信息
const currentMentionRange = ref<Range | null>(null); // @xxx 的 Range 对象
const currentMentionNode = ref<Text | null>(null);   // @xxx 所在的文本节点

// 上次提交的查询（避免重复提交）
const lastCommittedQuery = ref('');

// ============ Computed ============

/** 编辑器是否有内容 */
const hasContent = computed(() => {
  const text = getTextContent();
  return text.trim().length > 0;
});

/** 占位符文本 */
const placeholderText = computed(() => {
  const text = getTextContent();
  return text.trim() ? '' : '搜索剪贴板... 使用 @标签名 或 @类型';
});

/** 补全面板的样式（fixed 定位） */
const mentionPanelStyle = computed(() => ({
  position: 'fixed' as const,
  left: `${mentionPanelPosition.value.left}px`,
  top: `${mentionPanelPosition.value.top}px`,
  zIndex: 1000,
}));

/** 从所有剪贴板项目中提取的标签集合 */
const allTags = computed(() => {
  const tags = new Set<string>();
  props.items.forEach(item => item.tags?.forEach(t => tags.add(t)));
  return Array.from(tags);
});

/** 所有可用的补全选项（类型 + 标签） */
const mentionOptions = computed(() => {
  const options: { value: string; category: 'type' | 'tag'; count: number }[] = [];

  // 添加类型选项
  TYPE_OPTIONS.forEach(type => {
    options.push({
      value: type,
      category: 'type',
      count: mentionUsageCount.value[type] || 0
    });
  });

  // 添加标签选项
  allTags.value.forEach(tag => {
    options.push({
      value: tag,
      category: 'tag',
      count: mentionUsageCount.value[tag] || 0
    });
  });

  return options;
});

/** 过滤和排序后的补全选项 */
const filteredMentionOptions = computed(() => {
  let options = mentionOptions.value;

  // 过滤掉已经选择的标签（避免重复添加）
  const existingTags = getExistingTags();
  options = options.filter(o => !existingTags.includes(o.value));

  // 根据输入文本过滤
  if (mentionFilter.value) {
    const f = mentionFilter.value.toLowerCase();
    options = options.filter(o => o.value.toLowerCase().includes(f));
  }

  // 按使用次数排序，次数相同按名称排序
  return options.sort((a, b) => {
    if (b.count !== a.count) return b.count - a.count;
    return a.value.localeCompare(b.value);
  });
});

// ============ Core Functions ============

/**
 * 从编辑器内容中获取已存在的标签列表
 * 用于过滤补全选项（已选的不再显示）
 */
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

/**
 * 从 contenteditable 编辑器提取纯文本内容
 * 将内联标签（<span class="inline-tag">）转换为 @xxx 格式
 * 用于搜索查询和外部通信
 */
function getTextContent(): string {
  const editor = editorRef.value;
  if (!editor) return '';

  let text = '';

  /**
   * 递归遍历节点树
   * - 文本节点：直接追加内容
   * - 内联标签元素：转换为 @xxx 格式
   * - 其他元素：递归遍历子节点
   * - DIV 元素（非编辑器本身）：追加换行符
   */
  const walk = (node: Node) => {
    if (node.nodeType === Node.TEXT_NODE) {
      text += node.textContent || '';
    } else if (node.nodeType === Node.ELEMENT_NODE) {
      const el = node as HTMLElement;
      if (el.classList.contains('inline-tag')) {
        // 内联标签：转换为 @xxx 格式
        const tagName = el.getAttribute('data-tag');
        if (tagName) text += `@${tagName}`;
      } else {
        node.childNodes.forEach(walk);
      }
      // 处理换行（用户按 Enter 会创建 div）
      if (el.tagName === 'DIV' && el !== editor) {
        text += '\n';
      }
    }
  };

  editor.childNodes.forEach(walk);
  return text;
}

/**
 * 设置编辑器内容
 * 将外部传入的 @xxx 格式转换为内联标签显示
 * @param tags 标签列表（类型和标签）
 * @param text 剩余文本内容
 */
function setEditorContent(tags: { value: string; type: 'type' | 'tag' }[], text: string) {
  const editor = editorRef.value;
  if (!editor) return;

  editor.innerHTML = '';

  // 添加标签为内联元素
  tags.forEach(tag => {
    const span = createInlineTagElement(tag.value, tag.type);
    editor.appendChild(span);
    editor.appendChild(document.createTextNode('\u00A0')); // 不间断空格，避免标签粘连
  });

  // 添加剩余文本
  if (text) {
    const textNode = document.createTextNode(text);
    editor.appendChild(textNode);
  }

  // 将光标移到内容末尾
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

/**
 * 创建内联标签元素
 * @param value 标签/类型名称
 * @param type 'type' 表示类型，'tag' 表示标签
 * @returns HTMLSpanElement 标签元素
 */
function createInlineTagElement(value: string, type: 'type' | 'tag'): HTMLSpanElement {
  const span = document.createElement('span');
  span.className = `inline-tag ${type === 'type' ? 'is-type' : ''}`;
  span.contentEditable = 'false'; // 禁止编辑，光标会自动跳过
  span.setAttribute('data-tag', value);
  span.setAttribute('data-type', type);
  span.textContent = `@${value}`;
  return span;
}

/**
 * 获取光标在文本中的索引位置
 * 用于计算光标位置和提及检测
 * @returns 字符索引（从0开始）
 */
function getCaretPosition(): number {
  const editor = editorRef.value;
  if (!editor) return 0;

  const sel = window.getSelection();
  if (!sel || sel.rangeCount === 0) return 0;

  const range = sel.getRangeAt(0);
  let pos = 0;

  // 递归遍历，找到目标节点位置
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
        // 标签按 @xxx 长度计算
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

/**
 * 获取光标的屏幕坐标
 * 用于定位补全面板，使其跟随 @ 符号显示
 * @returns { left, top } 坐标或 null
 */
function getCaretCoordinates(): { left: number; top: number } | null {
  const sel = window.getSelection();
  if (!sel || sel.rangeCount === 0) return null;

  const range = sel.getRangeAt(0);
  const rect = range.getBoundingClientRect();

  // 折叠的选区（光标）可能没有尺寸，尝试从 client rects 获取
  if (rect.width === 0 && rect.height === 0) {
    const rects = range.getClientRects();
    if (rects.length > 0) {
      const lastRect = rects[rects.length - 1];
      return {
        left: lastRect.left,
        top: lastRect.bottom + 4 // +4px 偏移，不紧贴光标
      };
    }
  }

  return {
    left: rect.left,
    top: rect.bottom + 4
  };
}

// ============ Mention Panel Logic ============

/**
 * 检测当前是否正在输入 @mention
 * 逻辑：
 * 1. 获取光标位置前的文本
 * 2. 找到最后一个 @ 符号
 * 3. 检查 @ 是否在开头或空格后（合法位置）
 * 4. 检查 @ 后是否有空格（已完成输入）
 * 5. 保存 Range 用于后续替换
 */
function checkMention() {
  const sel = window.getSelection();
  if (!sel || sel.rangeCount === 0) {
    showMentionPanel.value = false;
    return;
  }

  const range = sel.getRangeAt(0);
  const node = range.startContainer;

  // 只在文本节点中检测
  if (node.nodeType !== Node.TEXT_NODE) {
    showMentionPanel.value = false;
    return;
  }

  const text = node.textContent || '';
  const offset = range.startOffset;

  // 获取光标前的文本
  const textBeforeCursor = text.slice(0, offset);
  const lastAtIndex = textBeforeCursor.lastIndexOf('@');

  // 没有 @ 符号
  if (lastAtIndex === -1) {
    showMentionPanel.value = false;
    return;
  }

  // @ 不在开头且前面不是空格（不合法位置，如 abc@def）
  if (lastAtIndex > 0 && textBeforeCursor[lastAtIndex - 1] !== ' ') {
    showMentionPanel.value = false;
    return;
  }

  // @ 后面有空格（用户已完成输入，如 @tag ）
  const afterAt = textBeforeCursor.slice(lastAtIndex + 1);
  if (afterAt.includes(' ')) {
    showMentionPanel.value = false;
    return;
  }

  // 显示补全面板
  mentionFilter.value = afterAt;
  currentMentionNode.value = node as Text;

  // 保存 Range，后续用于替换 @xxx 为内联标签
  const mentionRange = document.createRange();
  mentionRange.setStart(node, lastAtIndex);
  mentionRange.setEnd(node, offset);
  currentMentionRange.value = mentionRange;

  showMentionPanel.value = true;
  selectedMentionIndex.value = 0;

  // 计算并设置面板位置
  nextTick(() => {
    const coords = getCaretCoordinates();
    if (coords) {
      mentionPanelPosition.value = coords;
    }
  });
}

/**
 * 选择补全项，将 @xxx 替换为内联标签
 * @param item 选中的补全项
 */
function selectMention(item: { value: string; category: 'type' | 'tag' }) {
  clearBlurTimer();

  const range = currentMentionRange.value;
  const node = currentMentionNode.value;
  const editor = editorRef.value;

  if (range && node && editor) {
    // 删除 @xxx 文本
    range.deleteContents();

    // 插入内联标签
    const tagSpan = createInlineTagElement(item.value, item.category);
    range.insertNode(tagSpan);

    // 在标签后添加空格
    const spaceNode = document.createTextNode('\u00A0');
    tagSpan.after(spaceNode);

    // 将光标移到空格后
    const sel = window.getSelection();
    const newRange = document.createRange();
    newRange.setStartAfter(spaceNode);
    newRange.collapse(true);
    sel?.removeAllRanges();
    sel?.addRange(newRange);
  }

  // 记录使用次数（用于排序）
  mentionUsageCount.value[item.value] = (mentionUsageCount.value[item.value] || 0) + 1;

  showMentionPanel.value = false;
  currentMentionRange.value = null;
  currentMentionNode.value = null;

  // 触发搜索提交
  updateSearchQuery(true);

  nextTick(() => {
    editorRef.value?.focus();
  });
}

/**
 * 在补全面板中导航（↑↓）
 * 导航时自动滚动使选中项可见
 */
const navigateMentionList = (direction: 'up' | 'down') => {
  if (direction === 'up') {
    selectedMentionIndex.value = Math.max(0, selectedMentionIndex.value - 1);
  } else {
    selectedMentionIndex.value = Math.min(
      filteredMentionOptions.value.length - 1,
      selectedMentionIndex.value + 1
    );
  }

  // 滚动选中项到可视区域
  nextTick(() => {
    const listEl = mentionListRef.value;
    if (!listEl) return;

    const activeItem = listEl.querySelector('.mention-item.active') as HTMLElement;
    if (!activeItem) return;

    const listRect = listEl.getBoundingClientRect();
    const itemRect = activeItem.getBoundingClientRect();

    // 项在可视区域上方，滚动到顶部可见
    if (itemRect.top < listRect.top) {
      activeItem.scrollIntoView({ block: 'start', behavior: 'smooth' });
    }
    // 项在可视区域下方，滚动到底部可见
    else if (itemRect.bottom > listRect.bottom) {
      activeItem.scrollIntoView({ block: 'end', behavior: 'smooth' });
    }
  });
};

/**
 * 获取类型对应的图标
 */
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

// ============ Event Handlers ============

/** 聚焦编辑器 */
const focusInput = () => {
  editorRef.value?.focus();
};

/** 处理聚焦事件 */
const handleFocus = () => {
  clearBlurTimer();
  checkMention();
};

/** 失焦计时器，用于延迟关闭面板（允许点击面板选择） */
let blurTimer: number | null = null;

const clearBlurTimer = () => {
  if (blurTimer) {
    clearTimeout(blurTimer);
    blurTimer = null;
  }
};

/** 处理失焦事件 */
const handleBlur = () => {
  // 失焦时提交搜索
  const fullQuery = getTextContent();
  if (fullQuery.trim() && fullQuery !== lastCommittedQuery.value) {
    lastCommittedQuery.value = fullQuery;
    emit('search-commit', fullQuery);
  }

  // 延迟关闭面板，给点击面板留出时间
  blurTimer = window.setTimeout(() => {
    showMentionPanel.value = false;
  }, 150);
};

/**
 * 更新搜索查询
 * modelValue 立即更新（用于显示）
 * search 事件防抖后触发（用于实际搜索，避免卡顿）
 * @param shouldCommit 是否立即提交（选择标签时）
 */
const updateSearchQuery = (shouldCommit = false) => {
  const fullQuery = getTextContent();

  emit('update:modelValue', fullQuery);

  if (shouldCommit) {
    // 选择标签时立即提交，不防抖（即使是空查询也要触发，用于清空搜索）
    emit('search-commit', fullQuery);
    return;
  }

  // 防抖搜索（150ms）
  debouncedSearch(fullQuery);
};

/** 防抖搜索函数（150ms 延迟） */
const debouncedSearch = debounce((query: string) => {
  // 始终触发搜索事件，让父组件处理空查询的情况（显示所有数据）
  emit('search', query);
}, 150);

/** 处理输入事件 */
const handleInput = () => {
  if (isComposing.value) return; // 输入法组合中不处理
  checkMention();
  updateSearchQuery();
};

/** 处理输入法组合结束 */
const handleCompositionEnd = () => {
  isComposing.value = false;
  checkMention();
  updateSearchQuery();
};

/** 清除所有内容 */
const clearAll = () => {
  const editor = editorRef.value;
  if (editor) {
    editor.innerHTML = '';
  }
  updateSearchQuery();
  editorRef.value?.focus();
};

/**
 * 处理编辑器点击
 * 点击标签时选中整个标签（方便删除）
 */
const handleEditorClick = (e: MouseEvent) => {
  const target = e.target as HTMLElement;
  if (target.classList.contains('inline-tag')) {
    // 选中整个标签
    const range = document.createRange();
    const sel = window.getSelection();
    range.selectNode(target);
    sel?.removeAllRanges();
    sel?.addRange(range);
  }
};

/**
 * 处理键盘事件
 * - 补全面板打开时：↑↓ 导航，Enter/Tab 选择，Esc 关闭
 * - Backspace：删除选中的标签或光标前的标签
 * - Enter：提交搜索
 */
const handleKeyDown = (e: KeyboardEvent) => {
  // 补全面板打开时的键盘导航
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

  // Backspace 删除标签
  if (e.key === 'Backspace') {
    const sel = window.getSelection();
    if (sel && sel.rangeCount > 0) {
      const range = sel.getRangeAt(0);

      // 情况1：标签被选中（范围不折叠）
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

      // 情况2：光标在节点开头，前面是标签
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

  // Enter 提交搜索
  if (e.key === 'Enter' && !e.shiftKey && !showMentionPanel.value) {
    e.preventDefault();
    const fullQuery = getTextContent();
    if (fullQuery.trim()) {
      emit('search-commit', fullQuery);
    }
  }

  // Escape 清空搜索（仅当有内容时）
  if (e.key === 'Escape' && !showMentionPanel.value) {
    const hasSearchContent = hasContent.value;
    if (hasSearchContent) {
      e.preventDefault();
      e.stopPropagation(); // 阻止事件传播到 window，避免触发窗口隐藏
      clearAll();
    }
    // 如果搜索框已为空，不处理，让父组件（ClipboardList）处理隐藏窗口
  }
};

// ============ Watchers & Lifecycle ============

/**
 * 监听外部 modelValue 变化
 * 将外部的 @xxx 格式转换为内联标签显示
 * 注意：补全面板打开时不更新（避免覆盖用户正在输入的内容）
 */
watch(() => props.modelValue, (newValue) => {
  if (showMentionPanel.value) return;

  // 解析查询为标签和文本
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

  // 如果内容变化，更新编辑器
  const currentText = getTextContent();
  if (currentText !== newValue) {
    setEditorContent(newTags, remainingText);
  }
}, { immediate: true });

onUnmounted(() => {
  if (blurTimer) clearTimeout(blurTimer);
});

// 暴露方法给父组件
defineExpose({ focus: focusInput });
</script>

<style scoped>
/* ============ 布局容器 ============ */

.smart-search {
  position: relative;
  width: 100%;
}

/* 搜索输入框包装器 */
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

/* 聚焦状态 - 容器内任意子元素获得焦点时自动生效 */
.search-input-wrapper:focus-within {
  background: #fff;
  border-color: #262626;
}

/* 搜索图标 */
.search-icon {
  width: 16px;
  height: 16px;
  color: #bfbfbf;
  flex-shrink: 0;
  margin-right: 8px;
}

/* ============ Contenteditable 编辑器 ============ */

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

/* 空内容时的占位符 */
.search-editor:empty::before {
  content: attr(placeholder);
  color: #8c8c8c;
  pointer-events: none;
}

.search-editor:focus {
  outline: none;
}

/* ============ 内联标签样式 ============ */

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

/* 类型标签（绿色） */
:deep(.inline-tag.is-type) {
  background: #f6ffed;
  border-color: #b7eb8f;
  color: #52c41a;
}

/* 标签悬停效果 */
:deep(.inline-tag:hover) {
  background: #bae7ff;
  border-color: #69c0ff;
}

:deep(.inline-tag.is-type:hover) {
  background: #d9f7be;
  border-color: #95de64;
}

/* 选中状态（用于删除） */
:deep(.inline-tag.selected) {
  background: #ff4d4f;
  border-color: #ff7875;
  color: #fff;
}

/* ============ 清除按钮 ============ */

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

/* ============ 快捷键提示 ============ */

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

/* ============ @补全面板 ============ */

/* 面板容器 - 固定定位跟随光标 */
.search-dropdown {
  background: #ffffff;
  border-radius: 6px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
  border: 1px solid #f0f0f0;
  overflow: hidden;
  min-width: 200px;
  max-width: 280px;
}

/* 面板头部 */
.dropdown-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 6px 12px;
  font-size: 11px;
  color: #8c8c8c;
  border-bottom: 1px solid #f0f0f0;
}

.hint {
  font-size: 10px;
  color: #bfbfbf;
}

/* 空状态 */
.dropdown-empty {
  padding: 16px;
  text-align: center;
  color: #bfbfbf;
  font-size: 12px;
}

/* ============ 补全列表项 ============ */

.mention-list {
  display: flex;
  flex-direction: column;
  padding: 4px 0;
  max-height: 240px;
  overflow-y: auto;
}

/* 列表项 */
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

/* 悬停效果 */
.mention-item:hover {
  background: #f5f5f5;
}

/* 选中效果（深色背景） */
.mention-item.active {
  background: #262626;
  color: #fff;
}

/* 图标 */
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

/* 名称 */
.mention-item-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-weight: 400;
}

/* 元信息（使用次数/类型） */
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
