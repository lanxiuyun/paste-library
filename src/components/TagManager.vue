<template>
  <Teleport to="body">
    <Transition name="fade">
      <div v-if="visible" class="tag-manager-overlay" @click="close">
        <Transition name="scale">
          <div
            v-if="visible"
            ref="tagManagerRef"
            class="tag-manager"
            @click.stop
          >
            <!-- 标题栏 -->
            <div class="tag-manager-header">
              <div class="header-title">
                <svg
                  class="title-icon"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                >
                  <path
                    d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"
                  />
                </svg>
                <h3>管理标签</h3>
              </div>
              <button class="close-btn" @click="close" aria-label="关闭">
                <svg
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                >
                  <line x1="18" y1="6" x2="6" y2="18" />
                  <line x1="6" y1="6" x2="18" y2="18" />
                </svg>
              </button>
            </div>

            <!-- 项目信息预览区 -->
            <div v-if="item" class="item-preview-section">
              <div class="item-preview-card">
                <!-- 类型图标或图片缩略图 -->
                <div class="preview-type-icon" :class="item.content_type">
                  <!-- 图片缩略图 -->
                  <img
                    v-if="item.content_type === 'image' && item.thumbnail_path"
                    :src="convertFileSrc(item.thumbnail_path)"
                    class="preview-thumbnail"
                    alt="图片缩略图"
                  />
                  <div
                    v-else-if="item.content_type === 'image'"
                    class="preview-icon-wrapper"
                  >
                    <svg
                      viewBox="0 0 24 24"
                      fill="none"
                      stroke="currentColor"
                      stroke-width="2"
                    >
                      <rect x="3" y="3" width="18" height="18" rx="2" />
                      <circle cx="8.5" cy="8.5" r="1.5" />
                      <path d="M21 15l-5-5L5 21" />
                    </svg>
                  </div>
                  <!-- 其他类型图标 -->
                  <svg
                    v-else-if="item.content_type === 'text'"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                  >
                    <path d="M4 7V4h16v3M4 11h16M4 15h16M4 19h16" />
                  </svg>
                  <svg
                    v-else-if="
                      item.content_type === 'file' ||
                      item.content_type === 'files'
                    "
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                  >
                    <path
                      d="M13 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V9z"
                    />
                    <path d="M13 2v7h7" />
                  </svg>
                  <svg
                    v-else-if="item.content_type === 'folder'"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                  >
                    <path
                      d="M22 19a2 2 0 01-2 2H4a2 2 0 01-2-2V5a2 2 0 012-2h5l2 3h9a2 2 0 012 2z"
                    />
                  </svg>
                  <svg
                    v-else
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                  >
                    <rect x="3" y="3" width="18" height="18" rx="2" />
                    <path d="M7 7h10M7 12h10M7 17h10" />
                  </svg>
                </div>

                <!-- 内容预览 -->
                <div class="preview-content">
                  <div class="preview-type">
                    {{ getContentTypeLabel(item.content_type) }}
                  </div>
                  <div class="preview-text" :title="item.content">
                    {{ getContentPreview(item) }}
                  </div>
                  <div class="preview-meta">
                    <span class="item-time">{{
                      formatTime(item.created_at)
                    }}</span>
                  </div>
                </div>
              </div>
            </div>

            <!-- 主体内容 -->
            <div class="tag-manager-body">
              <!-- 左侧：已选标签 + 创建 -->
              <div class="body-left">
                <!-- 已选标签区域 -->
                <div class="section selected-section">
                  <div class="section-header">
                    <span class="section-title">已选标签</span>
                    <span class="section-badge">{{ selectedTags.length }}</span>
                    <button
                      v-if="selectedTags.length > 0"
                      class="text-btn danger"
                      @click="clearAllSelected"
                    >
                      清除全部
                    </button>
                  </div>

                  <div class="selected-tags-container">
                    <TransitionGroup name="tag" tag="div" class="selected-tags">
                      <span
                        v-for="tag in selectedTags"
                        :key="tag"
                        class="tag-chip selected"
                        :style="getTagStyle(tag)"
                      >
                        <span
                          class="tag-dot"
                          :style="{ backgroundColor: getTagStyle(tag).color }"
                        ></span>
                        {{ tag }}
                        <button
                          class="remove-btn"
                          @click.stop="removeTag(tag)"
                          aria-label="移除标签"
                        >
                          <svg
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2.5"
                          >
                            <line x1="18" y1="6" x2="6" y2="18" />
                            <line x1="6" y1="6" x2="18" y2="18" />
                          </svg>
                        </button>
                      </span>
                    </TransitionGroup>

                    <!-- 空状态 -->
                    <div v-if="selectedTags.length === 0" class="empty-state">
                      <svg
                        class="empty-icon"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="1.5"
                      >
                        <path
                          d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"
                        />
                      </svg>
                      <span>暂无标签，从右侧选择或创建新标签</span>
                    </div>
                  </div>
                </div>

                <!-- 创建新标签区域（带补全提示） -->
                <div class="section create-section">
                  <div class="section-header">
                    <span class="section-title">创建新标签</span>
                  </div>
                  <div class="create-tag-box" ref="createBoxRef">
                    <div
                      class="create-input-wrapper"
                      :class="{
                        'has-error': createError,
                        focused: isInputFocused,
                      }"
                    >
                      <svg
                        class="input-icon"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                      >
                        <line x1="12" y1="5" x2="12" y2="19" />
                        <line x1="5" y1="12" x2="19" y2="12" />
                      </svg>
                      <input
                        ref="inputRef"
                        v-model="newTagName"
                        type="text"
                        placeholder="输入标签名称，按回车创建..."
                        maxlength="20"
                        @focus="handleInputFocus"
                        @blur="handleInputBlur"
                        @input="handleTagInput"
                        @keydown="handleTagKeyDown"
                      />
                      <span
                        class="char-count"
                        :class="{ 'near-limit': newTagName.length > 15 }"
                      >
                        {{ newTagName.length }}/20
                      </span>
                    </div>

                    <!-- 标签补全下拉面板 - 使用 Teleport 到 body 避免被任何父容器裁剪 -->
                    <Teleport to="body">
                      <Transition
                        :name="
                          autocompleteDirection === 'up'
                            ? 'slide-up'
                            : 'slide-down'
                        "
                      >
                        <div
                          v-if="
                            showTagAutocomplete &&
                            tagAutocompleteOptions.length > 0
                          "
                          class="tag-autocomplete-panel"
                          :class="{
                            'direction-up': autocompleteDirection === 'up',
                          }"
                          :style="autocompletePanelStyle"
                          @mousedown.prevent
                        >
                          <div class="autocomplete-header">
                            <span>匹配 "{{ newTagName.trim() }}"</span>
                            <span class="autocomplete-hint"
                              >↑↓ 选择 · Enter 确认</span
                            >
                          </div>
                          <div
                            ref="autocompleteListRef"
                            class="autocomplete-list"
                          >
                            <div
                              v-for="(tag, index) in tagAutocompleteOptions"
                              :key="tag.name"
                              class="autocomplete-item"
                              :class="{
                                active: selectedAutocompleteIndex === index,
                              }"
                              @click="selectAutocompleteTag(tag)"
                              @mouseenter="selectedAutocompleteIndex = index"
                            >
                              <span
                                class="item-dot"
                                :style="{
                                  backgroundColor: getTagStyle(tag.name).color,
                                }"
                              ></span>
                              <span class="item-name">{{ tag.name }}</span>
                              <span class="item-count"
                                >{{ tag.count }} 个条目</span
                              >
                              <svg
                                v-if="isTagSelected(tag.name)"
                                class="item-check"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2.5"
                              >
                                <polyline points="20 6 9 17 4 12" />
                              </svg>
                            </div>
                          </div>
                        </div>
                      </Transition>
                    </Teleport>

                    <Transition name="slide-down">
                      <div v-if="createError" class="error-message">
                        <svg
                          viewBox="0 0 24 24"
                          fill="none"
                          stroke="currentColor"
                          stroke-width="2"
                        >
                          <circle cx="12" cy="12" r="10" />
                          <line x1="12" y1="8" x2="12" y2="12" />
                          <line x1="12" y1="16" x2="12.01" y2="16" />
                        </svg>
                        {{ createError }}
                      </div>
                    </Transition>
                    <button
                      class="create-btn"
                      :disabled="!canCreateTag"
                      @click="createNewTag"
                    >
                      <svg
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                      >
                        <line x1="12" y1="5" x2="12" y2="19" />
                        <line x1="5" y1="12" x2="19" y2="12" />
                      </svg>
                      创建标签
                    </button>
                  </div>
                </div>
              </div>

              <!-- 右侧：所有可用标签 -->
              <div class="body-right">
                <div class="section all-tags-section">
                  <div class="section-header">
                    <span class="section-title">所有标签</span>
                    <span class="section-badge">{{ filteredTags.length }}</span>
                  </div>

                  <!-- 搜索框 -->
                  <div class="search-box">
                    <svg
                      class="search-icon"
                      viewBox="0 0 24 24"
                      fill="none"
                      stroke="currentColor"
                      stroke-width="2"
                    >
                      <circle cx="11" cy="11" r="8" />
                      <line x1="21" y1="21" x2="16.65" y2="16.65" />
                    </svg>
                    <input
                      v-model="searchQuery"
                      type="text"
                      placeholder="搜索标签..."
                    />
                    <button
                      v-if="searchQuery"
                      class="clear-search"
                      @click="searchQuery = ''"
                      aria-label="清除搜索"
                    >
                      <svg
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                      >
                        <line x1="18" y1="6" x2="6" y2="18" />
                        <line x1="6" y1="6" x2="18" y2="18" />
                      </svg>
                    </button>
                  </div>

                  <!-- 快捷筛选 -->
                  <div class="quick-filters">
                    <button
                      class="filter-chip"
                      :class="{ active: filterMode === 'all' }"
                      @click="filterMode = 'all'"
                    >
                      全部
                    </button>
                    <button
                      class="filter-chip"
                      :class="{ active: filterMode === 'popular' }"
                      @click="filterMode = 'popular'"
                    >
                      热门
                    </button>
                    <button
                      class="filter-chip"
                      :class="{ active: filterMode === 'recent' }"
                      @click="filterMode = 'recent'"
                    >
                      最近使用
                    </button>
                  </div>

                  <!-- 标签列表 -->
                  <div class="tags-scroll-area" ref="tagsScrollArea">
                    <TransitionGroup
                      name="tag-list"
                      tag="div"
                      class="tags-grid"
                    >
                      <button
                        v-for="tag in displayedTags"
                        :key="tag.name"
                        class="tag-chip available"
                        :class="{ 'is-selected': isTagSelected(tag.name) }"
                        :style="getTagStyle(tag.name)"
                        :disabled="isTagSelected(tag.name)"
                        @click="addTag(tag.name)"
                      >
                        <span
                          class="tag-dot"
                          :style="{
                            backgroundColor: getTagStyle(tag.name).color,
                          }"
                        ></span>
                        <span class="tag-name">{{ tag.name }}</span>
                        <span class="tag-count">{{ tag.count }}</span>
                        <svg
                          v-if="isTagSelected(tag.name)"
                          class="check-icon"
                          viewBox="0 0 24 24"
                          fill="none"
                          stroke="currentColor"
                          stroke-width="2.5"
                        >
                          <polyline points="20 6 9 17 4 12" />
                        </svg>
                      </button>
                    </TransitionGroup>

                    <!-- 空状态 -->
                    <div
                      v-if="displayedTags.length === 0"
                      class="empty-state tags-empty"
                    >
                      <svg
                        class="empty-icon"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="1.5"
                      >
                        <circle cx="11" cy="11" r="8" />
                        <line x1="21" y1="21" x2="16.65" y2="16.65" />
                      </svg>
                      <span>{{
                        searchQuery ? "未找到匹配的标签" : "暂无可用标签"
                      }}</span>
                      <button
                        v-if="searchQuery && canCreateTag"
                        class="create-suggestion"
                        @click="createAndAddTag"
                      >
                        创建 "{{ searchQuery.trim() }}" 标签
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- 底部按钮 -->
            <div class="tag-manager-footer">
              <div class="footer-hint">
                <svg
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                >
                  <circle cx="12" cy="12" r="10" />
                  <line x1="12" y1="16" x2="12" y2="12" />
                  <line x1="12" y1="8" x2="12.01" y2="8" />
                </svg>
                <span>点击标签即可添加或移除</span>
              </div>
              <div class="footer-actions">
                <button class="btn secondary" @click="close">取消</button>
                <button class="btn primary" @click="save">
                  <svg
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                  >
                    <polyline points="20 6 9 17 4 12" />
                  </svg>
                  保存 ({{ selectedTags.length }})
                </button>
              </div>
            </div>
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from "vue";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { getTagStyle } from "@/utils/tagColors";
import type { ClipboardItem, ClipboardContentType } from "@/types";

interface TagInfo {
  name: string;
  count: number;
  lastUsed?: number;
}

interface Props {
  visible: boolean;
  item: ClipboardItem | null;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  "update:visible": [value: boolean];
  save: [itemId: number, tags: string[]];
}>();

// 本地状态
const selectedTags = ref<string[]>([]);
const availableTags = ref<TagInfo[]>([]);
const newTagName = ref("");
const createError = ref("");
const isLoading = ref(false);
const searchQuery = ref("");
const filterMode = ref<"all" | "popular" | "recent">("all");
const isInputFocused = ref(false);
const inputRef = ref<HTMLInputElement | null>(null);
const tagsScrollArea = ref<HTMLElement | null>(null);
const createBoxRef = ref<HTMLElement | null>(null);
const autocompleteListRef = ref<HTMLElement | null>(null);
const tagManagerRef = ref<HTMLElement | null>(null);

// 标签补全状态
const showTagAutocomplete = ref(false);
const selectedAutocompleteIndex = ref(0);
let autocompleteBlurTimer: number | null = null;

// 补全面板位置
const autocompletePanelPosition = ref({ left: 0, top: 0, width: 0 });

// 面板展开方向
const autocompleteDirection = ref<"up" | "down">("down");

// 补全面板样式 - 使用 fixed 定位避免被裁剪
const autocompletePanelStyle = computed(() => ({
  position: "fixed" as const,
  left: `${autocompletePanelPosition.value.left}px`,
  top: `${autocompletePanelPosition.value.top}px`,
  width: `${autocompletePanelPosition.value.width}px`,
  zIndex: 10001, // 高于遮罩层 10000
}));

// 计算属性
const canCreateTag = computed(() => {
  const name = newTagName.value.trim();
  return (
    name.length > 0 && name.length <= 20 && !selectedTags.value.includes(name)
  );
});

// 标签补全选项（过滤掉已选择的，按使用次数排序）
const tagAutocompleteOptions = computed(() => {
  const query = newTagName.value.trim().toLowerCase();
  if (!query) return [];

  return availableTags.value
    .filter((tag) => !selectedTags.value.includes(tag.name)) // 排除已选择的
    .filter((tag) => tag.name.toLowerCase().includes(query)) // 匹配输入
    .sort((a, b) => b.count - a.count) // 按使用次数排序
    .slice(0, 6); // 最多显示6个
});

// 过滤后的标签
const filteredTags = computed(() => {
  let tags = [...availableTags.value];

  // 搜索过滤
  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase().trim();
    tags = tags.filter((tag) => tag.name.toLowerCase().includes(query));
  }

  // 模式过滤
  switch (filterMode.value) {
    case "popular":
      tags = tags.sort((a, b) => b.count - a.count).slice(0, 20);
      break;
    case "recent":
      tags = tags
        .sort((a, b) => (b.lastUsed || 0) - (a.lastUsed || 0))
        .slice(0, 20);
      break;
    default:
      // all: 按名称排序
      tags = tags.sort((a, b) => a.name.localeCompare(b.name, "zh-CN"));
  }

  return tags;
});

// 显示的标签（排除已选择的，更简洁）
const displayedTags = computed(() => {
  // 只显示未选中的标签
  return filteredTags.value.filter((tag) => !isTagSelected(tag.name));
});

// 监听弹窗显示状态
watch(
  () => props.visible,
  async (newVisible) => {
    if (newVisible && props.item) {
      // 弹窗打开时，初始化已选标签
      selectedTags.value = [...(props.item.tags || [])];
      newTagName.value = "";
      createError.value = "";
      searchQuery.value = "";
      filterMode.value = "all";
      showTagAutocomplete.value = false;
      // 加载所有可用标签
      await loadAllTags();
      // 聚焦搜索框
      nextTick(() => {
        inputRef.value?.focus();
      });
    }
  }
);

// 监听补全面板显示状态，更新位置
watch(showTagAutocomplete, (isShowing) => {
  if (isShowing) {
    // 使用 requestAnimationFrame 确保 DOM 完全渲染后再计算位置
    requestAnimationFrame(() => {
      updateAutocompletePosition();
    });
  }
});

// 加载所有标签
const loadAllTags = async () => {
  try {
    isLoading.value = true;
    const tags = await invoke<[string, number][]>("get_all_tags");
    availableTags.value = tags.map(([name, count]) => ({
      name,
      count,
      lastUsed: Date.now() - Math.random() * 86400000 * 30, // 模拟最近使用数据
    }));
  } catch (error) {
    console.error("Failed to load tags:", error);
  } finally {
    isLoading.value = false;
  }
};

// 判断标签是否已选中
const isTagSelected = (tagName: string): boolean => {
  return selectedTags.value.includes(tagName);
};

// 添加标签
const addTag = (tagName: string) => {
  if (!isTagSelected(tagName)) {
    selectedTags.value.push(tagName);
    // 滚动到顶部显示新添加的标签
    nextTick(() => {
      if (tagsScrollArea.value) {
        tagsScrollArea.value.scrollTop = 0;
      }
    });
  }
};

// 移除标签
const removeTag = (tagName: string) => {
  const index = selectedTags.value.indexOf(tagName);
  if (index > -1) {
    selectedTags.value.splice(index, 1);
  }
};

// ===== 标签补全相关函数 =====

/** 更新补全面板位置 - 智能判断向上或向下展开 */
const updateAutocompletePosition = () => {
  const wrapper = createBoxRef.value?.querySelector(
    ".create-input-wrapper"
  ) as HTMLElement;
  const tagManager = tagManagerRef.value;
  if (!wrapper || !tagManager) return;

  const rect = wrapper.getBoundingClientRect();
  const managerRect = tagManager.getBoundingClientRect();

  // 面板默认高度（约 220px）
  const panelHeight = 220;
  // 下方可用空间
  const spaceBelow = managerRect.bottom - rect.bottom;
  // 上方可用空间
  const spaceAbove = rect.top - managerRect.top;

  // 判断向下还是向上展开
  const shouldShowAbove = spaceBelow < panelHeight && spaceAbove > spaceBelow;
  autocompleteDirection.value = shouldShowAbove ? "up" : "down";

  autocompletePanelPosition.value = {
    left: rect.left,
    top: shouldShowAbove ? rect.top - panelHeight - 4 : rect.bottom + 4,
    width: rect.width,
  };
};

/** 处理输入框聚焦 */
const handleInputFocus = () => {
  isInputFocused.value = true;
  // 延迟更新位置，确保过渡动画完成
  requestAnimationFrame(() => {
    updateAutocompletePosition();
  });
  if (newTagName.value.trim()) {
    showTagAutocomplete.value = true;
    selectedAutocompleteIndex.value = 0;
  }
};

/** 清除失焦计时器 */
const clearAutocompleteBlurTimer = () => {
  if (autocompleteBlurTimer) {
    clearTimeout(autocompleteBlurTimer);
    autocompleteBlurTimer = null;
  }
};

/** 处理输入框失焦 */
const handleInputBlur = () => {
  clearAutocompleteBlurTimer();
  // 延迟关闭，给点击面板留出时间
  autocompleteBlurTimer = window.setTimeout(() => {
    isInputFocused.value = false;
    showTagAutocomplete.value = false;
  }, 150);
};

/** 处理输入事件 */
const handleTagInput = () => {
  createError.value = "";
  // 延迟更新位置，避免输入法或布局变化影响
  requestAnimationFrame(() => {
    updateAutocompletePosition();
  });
  if (newTagName.value.trim()) {
    showTagAutocomplete.value = true;
    selectedAutocompleteIndex.value = 0;
  } else {
    showTagAutocomplete.value = false;
  }
};

/** 选择补全标签 */
const selectAutocompleteTag = (tag: TagInfo) => {
  clearAutocompleteBlurTimer();

  if (!isTagSelected(tag.name)) {
    selectedTags.value.push(tag.name);
  }

  newTagName.value = "";
  showTagAutocomplete.value = false;

  // 保持输入框聚焦
  nextTick(() => {
    inputRef.value?.focus();
  });
};

/** 键盘导航补全列表 */
const navigateAutocomplete = (direction: "up" | "down") => {
  const max = tagAutocompleteOptions.value.length - 1;
  if (direction === "up") {
    selectedAutocompleteIndex.value = Math.max(
      0,
      selectedAutocompleteIndex.value - 1
    );
  } else {
    selectedAutocompleteIndex.value = Math.min(
      max,
      selectedAutocompleteIndex.value + 1
    );
  }

  // 滚动选中项到可视区域
  nextTick(() => {
    const listEl = autocompleteListRef.value;
    if (!listEl) return;

    const activeItem = listEl.querySelector(
      ".autocomplete-item.active"
    ) as HTMLElement;
    if (!activeItem) return;

    const listRect = listEl.getBoundingClientRect();
    const itemRect = activeItem.getBoundingClientRect();

    if (itemRect.top < listRect.top) {
      activeItem.scrollIntoView({ block: "start", behavior: "smooth" });
    } else if (itemRect.bottom > listRect.bottom) {
      activeItem.scrollIntoView({ block: "end", behavior: "smooth" });
    }
  });
};

/** 处理键盘事件 */
const handleTagKeyDown = (e: KeyboardEvent) => {
  // 补全面板打开时的键盘导航
  if (showTagAutocomplete.value && tagAutocompleteOptions.value.length > 0) {
    switch (e.key) {
      case "ArrowDown":
        e.preventDefault();
        e.stopPropagation();
        navigateAutocomplete("down");
        return;
      case "ArrowUp":
        e.preventDefault();
        e.stopPropagation();
        navigateAutocomplete("up");
        return;
      case "Enter":
        e.preventDefault();
        e.stopPropagation();
        const selected =
          tagAutocompleteOptions.value[selectedAutocompleteIndex.value];
        if (selected) {
          selectAutocompleteTag(selected);
        } else {
          createNewTag();
        }
        return;
      case "Escape":
        e.preventDefault();
        e.stopPropagation();
        showTagAutocomplete.value = false;
        return;
    }
  }

  // 没有补全面板时的回车创建
  if (e.key === "Enter") {
    e.preventDefault();
    e.stopPropagation();
    createNewTag();
  }
};

// 清除所有已选标签
const clearAllSelected = () => {
  selectedTags.value = [];
};

// 获取内容类型标签
const getContentTypeLabel = (type: ClipboardContentType): string => {
  const labels: Record<ClipboardContentType, string> = {
    text: "文本",
    html: "HTML",
    rtf: "富文本",
    image: "图片",
    file: "文件",
    folder: "文件夹",
    files: "多文件",
  };
  return labels[type] || type;
};

// 获取内容预览
const getContentPreview = (item: ClipboardItem): string => {
  if (!item) return "";

  switch (item.content_type) {
    case "text":
    case "html":
    case "rtf":
      // 返回纯文本内容或截断后的 content
      const text = item.text_content || item.content;
      return text.length > 80 ? text.slice(0, 80) + "..." : text;

    case "image":
      if (item.metadata?.width && item.metadata?.height) {
        return `图片 ${item.metadata.width}×${item.metadata.height}`;
      }
      return "图片";

    case "file":
      return item.metadata?.file_name || "文件";

    case "folder":
      return item.metadata?.folder_name || "文件夹";

    case "files":
      const count = item.file_paths?.length || 0;
      return `${count} 个文件`;

    default:
      return item.content.slice(0, 60);
  }
};

// 格式化时间
const formatTime = (timeStr: string): string => {
  const date = new Date(timeStr);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMs / 3600000);
  const diffDays = Math.floor(diffMs / 86400000);

  if (diffMins < 1) return "刚刚";
  if (diffMins < 60) return `${diffMins}分钟前`;
  if (diffHours < 24) return `${diffHours}小时前`;
  if (diffDays < 7) return `${diffDays}天前`;

  return date.toLocaleDateString("zh-CN", {
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
};

// 创建新标签
const createNewTag = () => {
  const name = newTagName.value.trim();
  if (!name) return;

  if (name.length > 20) {
    createError.value = "标签名称不能超过20个字符";
    return;
  }

  if (selectedTags.value.includes(name)) {
    createError.value = "该标签已存在";
    return;
  }

  // 添加到已选标签
  selectedTags.value.push(name);

  // 如果不在可用标签列表中，添加进去
  if (!availableTags.value.some((t) => t.name === name)) {
    availableTags.value.push({ name, count: 0, lastUsed: Date.now() });
  }

  newTagName.value = "";
  createError.value = "";

  // 聚焦回输入框
  nextTick(() => {
    inputRef.value?.focus();
  });
};

// 从搜索结果创建并添加标签
const createAndAddTag = () => {
  newTagName.value = searchQuery.value.trim();
  createNewTag();
  searchQuery.value = "";
};

// 关闭弹窗
const close = () => {
  emit("update:visible", false);
};

// 保存标签
const save = async () => {
  if (props.item) {
    try {
      await invoke("update_tags", {
        id: props.item.id,
        tags: selectedTags.value.length > 0 ? selectedTags.value : null,
      });
      emit("save", props.item.id, selectedTags.value);
      close();
    } catch (error) {
      console.error("Failed to update tags:", error);
    }
  }
};

onMounted(() => {
  if (props.visible) {
    loadAllTags();
  }
  // 监听窗口变化，更新补全面板位置
  window.addEventListener("resize", updateAutocompletePosition);
  window.addEventListener("scroll", updateAutocompletePosition, true);
});

onUnmounted(() => {
  if (autocompleteBlurTimer) {
    clearTimeout(autocompleteBlurTimer);
  }
  window.removeEventListener("resize", updateAutocompletePosition);
  window.removeEventListener("scroll", updateAutocompletePosition, true);
});
</script>

<style scoped>
/* ===== 基础变量 ===== */
.tag-manager {
  --color-primary: #0d9488;
  --color-primary-light: #14b8a6;
  --color-primary-bg: #f0fdfa;
  --color-cta: #f97316;
  --color-cta-hover: #ea580c;
  --color-text: #134e4a;
  --color-text-muted: #64748b;
  --color-text-light: #94a3b8;
  --color-border: #e2e8f0;
  --color-border-light: #f1f5f9;
  --color-white: #ffffff;
  --color-error: #ef4444;
  --color-error-bg: #fef2f2;
  --shadow-sm: 0 1px 2px 0 rgb(0 0 0 / 0.05);
  --shadow-md: 0 2px 4px -1px rgb(0 0 0 / 0.08),
    0 1px 2px -1px rgb(0 0 0 / 0.06);
  --shadow-lg: 0 8px 12px -2px rgb(0 0 0 / 0.08),
    0 3px 5px -2px rgb(0 0 0 / 0.06);
  --radius-sm: 6px;
  --radius-md: 8px;
  --radius-lg: 12px;
  --transition-fast: 120ms ease;
  --transition-normal: 180ms ease;
}

/* ===== 遮罩层动画 ===== */
.fade-enter-active,
.fade-leave-active {
  transition: opacity var(--transition-normal);
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* ===== 弹窗缩放动画 ===== */
.scale-enter-active,
.scale-leave-active {
  transition: all var(--transition-normal);
}

.scale-enter-from,
.scale-leave-to {
  opacity: 0;
  transform: scale(0.96) translateY(-8px);
}

/* ===== 标签动画 ===== */
.tag-enter-active,
.tag-leave-active {
  transition: all var(--transition-fast);
}

.tag-enter-from,
.tag-leave-to {
  opacity: 0;
  transform: scale(0.9) translateY(-4px);
}

.tag-list-enter-active,
.tag-list-leave-active {
  transition: all var(--transition-fast);
}

.tag-list-enter-from,
.tag-list-leave-to {
  opacity: 0;
  transform: scale(0.95);
}

/* ===== 下滑动画 ===== */
.slide-down-enter-active,
.slide-down-leave-active {
  transition: all var(--transition-fast);
}

.slide-down-enter-from,
.slide-down-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}

/* ===== 上滑动画（面板向上展开时使用） ===== */
.slide-up-enter-active,
.slide-up-leave-active {
  transition: all var(--transition-fast);
}

.slide-up-enter-from,
.slide-up-leave-to {
  opacity: 0;
  transform: translateY(8px);
}

/* ===== 遮罩层 ===== */
.tag-manager-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(15, 23, 42, 0.5);
  backdrop-filter: blur(3px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10000;
  -webkit-app-region: no-drag;
  app-region: no-drag;
  padding: 20px;
}

/* ===== 弹窗主体 ===== */
.tag-manager {
  background: var(--color-white);
  border-radius: var(--radius-lg);
  width: 90%;
  max-width: 100%;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: var(--shadow-lg);
  overflow: hidden;
}

/* ===== 标题栏 ===== */
.tag-manager-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 16px;
  border-bottom: 1px solid var(--color-border-light);
  background: var(--color-white);
}

.header-title {
  display: flex;
  align-items: center;
  gap: 8px;
}

.title-icon {
  width: 18px;
  height: 18px;
  color: var(--color-primary);
  flex-shrink: 0;
}

.header-title h3 {
  margin: 0;
  font-size: 15px;
  font-weight: 600;
  color: var(--color-text);
  letter-spacing: -0.2px;
}

.item-badge {
  font-size: 11px;
  font-weight: 500;
  color: var(--color-text-light);
  background: var(--color-border-light);
  padding: 2px 8px;
  border-radius: 20px;
}

/* ===== 项目信息预览区 ===== */
.item-preview-section {
  padding: 10px 16px;
  background: var(--color-primary-bg);
  border-bottom: 1px solid var(--color-border-light);
}

.item-preview-card {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  background: var(--color-white);
  border-radius: var(--radius-md);
  border: 1px solid var(--color-border);
}

.preview-type-icon {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-sm);
  flex-shrink: 0;
}

.preview-type-icon svg {
  width: 18px;
  height: 18px;
}

.preview-type-icon.text {
  background: #e0f2fe;
  color: #0284c7;
}

.preview-type-icon.image {
  background: #fce7f3;
  color: #be185d;
}

.preview-type-icon.file {
  background: #fef3c7;
  color: #b45309;
}

.preview-type-icon.folder {
  background: #d1fae5;
  color: #047857;
}

.preview-type-icon.files {
  background: #f3e8ff;
  color: #7c3aed;
}

.preview-type-icon.html,
.preview-type-icon.rtf {
  background: #dbeafe;
  color: #1d4ed8;
}

/* 图片缩略图 */
.preview-thumbnail {
  width: 100%;
  height: 100%;
  object-fit: cover;
  border-radius: var(--radius-sm);
}

/* 图标包装器 */
.preview-icon-wrapper {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.preview-icon-wrapper svg {
  width: 18px;
  height: 18px;
}

.preview-content {
  flex: 1;
  min-width: 0;
}

.preview-type {
  font-size: 11px;
  font-weight: 500;
  color: var(--color-primary);
  text-transform: uppercase;
  letter-spacing: 0.3px;
  margin-bottom: 3px;
}

.preview-text {
  font-size: 13px;
  color: var(--color-text);
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 1;
  -webkit-box-orient: vertical;
  overflow: hidden;
  word-break: break-word;
}

.preview-meta {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-top: 4px;
  font-size: 11px;
  color: var(--color-text-light);
}

.preview-meta .separator {
  opacity: 0.5;
}

.preview-meta .item-id {
  font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas,
    monospace;
  background: var(--color-border-light);
  padding: 1px 5px;
  border-radius: 3px;
  font-size: 10px;
}

.close-btn {
  width: 28px;
  height: 28px;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-muted);
  transition: all var(--transition-fast);
}

.close-btn:hover {
  background: var(--color-border-light);
  color: var(--color-text);
}

.close-btn svg {
  width: 16px;
  height: 16px;
}

/* ===== 主体内容区（双栏布局） ===== */
.tag-manager-body {
  display: flex;
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

.body-left {
  width: 45%;
  min-width: 280px;
  display: flex;
  flex-direction: column;
  border-right: 1px solid var(--color-border-light);
}

.body-right {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
}

/* ===== 通用区块 ===== */
.section {
  padding: 12px 16px;
}

.section-header {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 10px;
}

.section-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--color-text);
  letter-spacing: 0.3px;
}

.section-badge {
  font-size: 11px;
  font-weight: 600;
  color: var(--color-primary);
  background: var(--color-primary-bg);
  padding: 1px 6px;
  border-radius: 10px;
  min-width: 20px;
  text-align: center;
}

/* ===== 已选标签区域 ===== */
.selected-section {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  padding-bottom: 10px;
}

.selected-tags-container {
  flex: 1;
  min-height: 60px;
  max-height: 160px;
  overflow-y: auto;
}

.selected-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

/* ===== 创建新标签区域 ===== */
.create-section {
  border-top: 1px solid var(--color-border-light);
  background: var(--color-primary-bg);
}

.create-tag-box {
  display: flex;
  flex-direction: column;
  gap: 8px;
  position: relative;
}

/* ===== 标签补全下拉面板（全局样式，因 Teleport 到 body） ===== */
:global(.tag-autocomplete-panel) {
  background: #ffffff !important;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  box-shadow: 0 6px 10px -2px rgba(0, 0, 0, 0.08),
    0 3px 4px -2px rgba(0, 0, 0, 0.06);
  overflow: hidden;
  max-height: 180px;
}

:global(.tag-autocomplete-panel .autocomplete-header) {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 6px 10px;
  font-size: 10px;
  color: #64748b;
  background: #f8fafc;
  border-bottom: 1px solid #e2e8f0;
}

:global(.tag-autocomplete-panel .autocomplete-hint) {
  font-size: 9px;
  color: #94a3b8;
}

:global(.tag-autocomplete-panel .autocomplete-list) {
  max-height: 140px;
  overflow-y: auto;
  padding: 3px;
  background: #ffffff;
}

:global(.tag-autocomplete-panel .autocomplete-item) {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 8px;
  border-radius: 5px;
  cursor: pointer;
  transition: all 120ms ease;
  font-size: 12px;
  color: #134e4a;
  background: transparent;
  margin: 1px 0;
}

:global(.tag-autocomplete-panel .autocomplete-item:hover) {
  background: #f0fdfa;
}

:global(.tag-autocomplete-panel .autocomplete-item.active) {
  background: #0d9488 !important;
  color: #ffffff !important;
}

:global(.tag-autocomplete-panel .autocomplete-item.active .item-count) {
  color: rgba(255, 255, 255, 0.8);
}

:global(.tag-autocomplete-panel .item-dot) {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  flex-shrink: 0;
}

:global(.tag-autocomplete-panel .item-name) {
  flex: 1;
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

:global(.tag-autocomplete-panel .item-count) {
  font-size: 10px;
  color: #94a3b8;
  flex-shrink: 0;
}

:global(.tag-autocomplete-panel .item-check) {
  width: 12px;
  height: 12px;
  flex-shrink: 0;
}

/* 补全列表滚动条 */
:global(.tag-autocomplete-panel .autocomplete-list::-webkit-scrollbar) {
  width: 5px;
}

:global(.tag-autocomplete-panel .autocomplete-list::-webkit-scrollbar-track) {
  background: transparent;
}

:global(.tag-autocomplete-panel .autocomplete-list::-webkit-scrollbar-thumb) {
  background: #e2e8f0;
  border-radius: 3px;
}

:global(
    .tag-autocomplete-panel .autocomplete-list::-webkit-scrollbar-thumb:hover
  ) {
  background: #94a3b8;
}

/* 向上展开时的样式调整 */
:global(.tag-autocomplete-panel.direction-up) {
  border-radius: 8px;
}

:global(.tag-autocomplete-panel.direction-up .autocomplete-header) {
  border-bottom: none;
  border-top: 1px solid #e2e8f0;
  order: 1;
}

:global(.tag-autocomplete-panel.direction-up .autocomplete-list) {
  order: 0;
}

.create-input-wrapper {
  display: flex;
  align-items: center;
  gap: 8px;
  background: var(--color-white);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: 6px 10px;
  transition: all var(--transition-fast);
}

.create-input-wrapper.focused {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 2px rgba(13, 148, 136, 0.08);
}

.create-input-wrapper.has-error {
  border-color: var(--color-error);
  background: var(--color-error-bg);
}

.input-icon {
  width: 14px;
  height: 14px;
  color: var(--color-text-light);
  flex-shrink: 0;
}

.create-input-wrapper input {
  flex: 1;
  border: none;
  background: transparent;
  font-size: 13px;
  color: var(--color-text);
  outline: none;
  padding: 0;
  min-width: 0;
}

.create-input-wrapper input::placeholder {
  color: var(--color-text-light);
  font-size: 12px;
}

.char-count {
  font-size: 10px;
  color: var(--color-text-light);
  font-variant-numeric: tabular-nums;
  flex-shrink: 0;
}

.char-count.near-limit {
  color: var(--color-cta);
  font-weight: 500;
}

.error-message {
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 11px;
  color: var(--color-error);
  padding: 0 2px;
}

.error-message svg {
  width: 12px;
  height: 12px;
  flex-shrink: 0;
}

.create-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 5px;
  padding: 8px 14px;
  background: var(--color-primary);
  color: var(--color-white);
  border: none;
  border-radius: var(--radius-md);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.create-btn:hover:not(:disabled) {
  background: var(--color-primary-light);
}

.create-btn:disabled {
  background: var(--color-text-light);
  cursor: not-allowed;
  opacity: 0.6;
}

.create-btn svg {
  width: 12px;
  height: 12px;
}

/* ===== 所有标签区域 ===== */
.all-tags-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

/* 搜索框 */
.search-box {
  position: relative;
  margin-bottom: 10px;
}

.search-box input {
  width: 100%;
  padding: 8px 32px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  font-size: 13px;
  color: var(--color-text);
  background: var(--color-white);
  outline: none;
  transition: all var(--transition-fast);
}

.search-box input:focus {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 2px rgba(13, 148, 136, 0.08);
}

.search-box input::placeholder {
  color: var(--color-text-light);
  font-size: 12px;
}

.search-icon {
  position: absolute;
  left: 10px;
  top: 50%;
  transform: translateY(-50%);
  width: 14px;
  height: 14px;
  color: var(--color-text-light);
  pointer-events: none;
}

.clear-search {
  position: absolute;
  right: 8px;
  top: 50%;
  transform: translateY(-50%);
  width: 18px;
  height: 18px;
  border: none;
  background: var(--color-border-light);
  cursor: pointer;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-muted);
  transition: all var(--transition-fast);
}

.clear-search:hover {
  background: var(--color-border);
  color: var(--color-text);
}

.clear-search svg {
  width: 10px;
  height: 10px;
}

/* 快捷筛选 */
.quick-filters {
  display: flex;
  gap: 6px;
  margin-bottom: 10px;
  flex-wrap: wrap;
}

.filter-chip {
  padding: 4px 10px;
  border: 1px solid var(--color-border);
  background: var(--color-white);
  color: var(--color-text-muted);
  font-size: 11px;
  font-weight: 500;
  border-radius: 12px;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.filter-chip:hover {
  border-color: var(--color-primary-light);
  color: var(--color-primary);
}

.filter-chip.active {
  background: var(--color-primary);
  border-color: var(--color-primary);
  color: var(--color-white);
}

/* 标签滚动区域 */
.tags-scroll-area {
  flex: 1;
  overflow-y: auto;
  padding-right: 3px;
  margin-right: -3px;
  min-height: 100px;
  max-height: 220px;
}

.tags-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

/* ===== 标签芯片样式 ===== */
.tag-chip {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
  border: 1px solid transparent;
  user-select: none;
  white-space: nowrap;
}

.tag-dot {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  flex-shrink: 0;
}

/* 已选标签样式 */
.tag-chip.selected {
  cursor: default;
  padding-right: 6px;
}

.remove-btn {
  width: 14px;
  height: 14px;
  border: none;
  background: rgba(0, 0, 0, 0.08);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  padding: 0;
  margin-left: 2px;
  color: inherit;
  opacity: 0.7;
  transition: all var(--transition-fast);
  flex-shrink: 0;
}

.remove-btn:hover {
  opacity: 1;
  background: rgba(0, 0, 0, 0.15);
}

.remove-btn svg {
  width: 9px;
  height: 9px;
}

/* 可用标签样式 */
.tag-chip.available {
  position: relative;
  overflow: hidden;
}

.tag-chip.available:hover:not(:disabled) {
  transform: translateY(-1px);
}

.tag-chip.available:disabled {
  cursor: default;
  opacity: 0.7;
}

.tag-chip.available.is-selected {
  padding-right: 24px;
}

.tag-count {
  font-size: 10px;
  font-weight: 500;
  opacity: 0.7;
  background: rgba(255, 255, 255, 0.3);
  padding: 0 4px;
  border-radius: 8px;
  min-width: 16px;
  text-align: center;
}

.check-icon {
  position: absolute;
  right: 6px;
  width: 12px;
  height: 12px;
  color: currentColor;
}

/* ===== 空状态 ===== */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 24px 16px;
  color: var(--color-text-light);
  text-align: center;
  font-size: 12px;
  flex: 1;
}

.empty-icon {
  width: 32px;
  height: 32px;
  opacity: 0.4;
}

.empty-state.tags-empty {
  padding: 36px 16px;
}

.create-suggestion {
  margin-top: 6px;
  padding: 6px 12px;
  background: var(--color-primary);
  color: var(--color-white);
  border: none;
  border-radius: var(--radius-md);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.create-suggestion:hover {
  background: var(--color-primary-light);
}

/* ===== 底部按钮区 ===== */
.tag-manager-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 12px 16px;
  border-top: 1px solid var(--color-border-light);
  background: var(--color-primary-bg);
}

.footer-hint {
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 11px;
  color: var(--color-text-light);
}

.footer-hint svg {
  width: 12px;
  height: 12px;
  flex-shrink: 0;
}

.footer-actions {
  display: flex;
  gap: 8px;
}

.text-btn {
  padding: 3px 6px;
  border: none;
  background: transparent;
  font-size: 11px;
  font-weight: 500;
  cursor: pointer;
  border-radius: var(--radius-sm);
  transition: all var(--transition-fast);
  margin-left: auto;
}

.text-btn.danger {
  color: var(--color-error);
}

.text-btn.danger:hover {
  background: var(--color-error-bg);
}

.btn {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 7px 14px;
  border-radius: var(--radius-md);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
  border: none;
}

.btn.secondary {
  background: var(--color-white);
  color: var(--color-text-muted);
  border: 1px solid var(--color-border);
}

.btn.secondary:hover {
  color: var(--color-text);
  border-color: var(--color-primary-light);
  background: var(--color-white);
}

.btn.primary {
  background: var(--color-primary);
  color: var(--color-white);
}

.btn.primary:hover {
  background: var(--color-primary-light);
}

.btn svg {
  width: 12px;
  height: 12px;
}

/* ===== 滚动条样式 ===== */
.selected-tags-container::-webkit-scrollbar,
.tags-scroll-area::-webkit-scrollbar {
  width: 6px;
}

.selected-tags-container::-webkit-scrollbar-track,
.tags-scroll-area::-webkit-scrollbar-track {
  background: var(--color-border-light);
  border-radius: 3px;
  margin: 3px 0;
}

.selected-tags-container::-webkit-scrollbar-thumb,
.tags-scroll-area::-webkit-scrollbar-thumb {
  background: var(--color-text-light);
  border-radius: 3px;
  border: 1px solid var(--color-border-light);
  min-height: 30px;
}

.selected-tags-container::-webkit-scrollbar-thumb:hover,
.tags-scroll-area::-webkit-scrollbar-thumb:hover {
  background: var(--color-primary);
}

/* 滚动区域始终显示滚动条空间 */
.tags-scroll-area {
  scrollbar-gutter: stable;
}

/* Firefox 滚动条支持 */
.selected-tags-container,
.tags-scroll-area {
  scrollbar-width: thin;
  scrollbar-color: var(--color-text-light) var(--color-border-light);
}

/* ===== 响应式 ===== */
@media (max-width: 640px) {
  .tag-manager-body {
    flex-direction: column;
  }

  .body-left {
    width: 100%;
    border-right: none;
    border-bottom: 1px solid var(--color-border-light);
    max-height: 45%;
  }

  .tag-manager {
    width: 100%;
    max-height: 90vh;
    margin: 10px;
  }

  .tag-manager-overlay {
    padding: 8px;
  }
}
</style>
