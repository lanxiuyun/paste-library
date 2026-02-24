<template>
  <Teleport to="body">
    <div
      v-if="visible"
      class="tag-manager-overlay"
      @click="close"
    >
      <div
        class="tag-manager"
        @click.stop
      >
        <!-- 标题栏 -->
        <div class="tag-manager-header">
          <h3>管理标签</h3>
          <button class="close-btn" @click="close">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18"/>
              <line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
          </button>
        </div>

        <!-- 已选标签 -->
        <div class="selected-tags-section">
          <div class="section-title">已选标签</div>
          <div class="selected-tags">
            <span
              v-for="tag in selectedTags"
              :key="tag"
              class="tag-chip selected"
              :style="getTagStyle(tag)"
            >
              {{ tag }}
              <button class="remove-btn" @click="removeTag(tag)">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <line x1="18" y1="6" x2="6" y2="18"/>
                  <line x1="6" y1="6" x2="18" y2="18"/>
                </svg>
              </button>
            </span>
            <span v-if="selectedTags.length === 0" class="no-tags-hint">
              暂无标签，从下方选择或创建新标签
            </span>
          </div>
        </div>

        <!-- 创建新标签 -->
        <div class="create-tag-section">
          <div class="section-title">创建新标签</div>
          <div class="create-tag-input">
            <input
              v-model="newTagName"
              type="text"
              placeholder="输入标签名称..."
              maxlength="20"
              @keyup.enter="createNewTag"
            />
            <button
              class="create-btn"
              :disabled="!canCreateTag"
              @click="createNewTag"
            >
              创建
            </button>
          </div>
          <div v-if="createError" class="error-message">{{ createError }}</div>
        </div>

        <!-- 所有可用标签 -->
        <div class="all-tags-section">
          <div class="section-title">
            所有标签
            <span class="tag-count">({{ availableTags.length }})</span>
          </div>
          <div class="tags-list">
            <span
              v-for="tag in availableTags"
              :key="tag.name"
              class="tag-chip"
              :class="{ selected: isTagSelected(tag.name), disabled: isTagSelected(tag.name) }"
              :style="getTagStyle(tag.name)"
              @click="addTag(tag.name)"
            >
              {{ tag.name }}
              <span class="tag-usage">({{ tag.count }})</span>
            </span>
            <span v-if="availableTags.length === 0" class="no-tags-hint">
              暂无可用标签
            </span>
          </div>
        </div>

        <!-- 底部按钮 -->
        <div class="tag-manager-footer">
          <button class="btn cancel" @click="close">取消</button>
          <button class="btn confirm" @click="save">保存</button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getTagStyle } from '@/utils/tagColors';
import type { ClipboardItem } from '@/types';

interface TagInfo {
  name: string;
  count: number;
}

interface Props {
  visible: boolean;
  item: ClipboardItem | null;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  'update:visible': [value: boolean];
  'save': [itemId: number, tags: string[]];
}>();

// 本地状态
const selectedTags = ref<string[]>([]);
const availableTags = ref<TagInfo[]>([]);
const newTagName = ref('');
const createError = ref('');
const isLoading = ref(false);

// 计算属性
const canCreateTag = computed(() => {
  const name = newTagName.value.trim();
  return name.length > 0 && name.length <= 20 && !selectedTags.value.includes(name);
});

// 监听弹窗显示状态
watch(() => props.visible, async (newVisible) => {
  if (newVisible && props.item) {
    // 弹窗打开时，初始化已选标签
    selectedTags.value = [...(props.item.tags || [])];
    newTagName.value = '';
    createError.value = '';
    // 加载所有可用标签
    await loadAllTags();
  }
});

// 加载所有标签
const loadAllTags = async () => {
  try {
    isLoading.value = true;
    const tags = await invoke<[string, number][]>('get_all_tags');
    availableTags.value = tags.map(([name, count]) => ({ name, count }));
  } catch (error) {
    console.error('Failed to load tags:', error);
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
  }
};

// 移除标签
const removeTag = (tagName: string) => {
  const index = selectedTags.value.indexOf(tagName);
  if (index > -1) {
    selectedTags.value.splice(index, 1);
  }
};

// 创建新标签
const createNewTag = () => {
  const name = newTagName.value.trim();
  if (!name) return;

  if (name.length > 20) {
    createError.value = '标签名称不能超过20个字符';
    return;
  }

  if (selectedTags.value.includes(name)) {
    createError.value = '该标签已存在';
    return;
  }

  // 添加到已选标签
  selectedTags.value.push(name);
  
  // 如果不在可用标签列表中，添加进去
  if (!availableTags.value.some(t => t.name === name)) {
    availableTags.value.push({ name, count: 0 });
  }

  newTagName.value = '';
  createError.value = '';
};

// 关闭弹窗
const close = () => {
  emit('update:visible', false);
};

// 保存标签
const save = async () => {
  if (props.item) {
    try {
      await invoke('update_tags', {
        id: props.item.id,
        tags: selectedTags.value.length > 0 ? selectedTags.value : null,
      });
      emit('save', props.item.id, selectedTags.value);
      close();
    } catch (error) {
      console.error('Failed to update tags:', error);
    }
  }
};

onMounted(() => {
  if (props.visible) {
    loadAllTags();
  }
});
</script>

<style scoped>
.tag-manager-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10000;
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.tag-manager {
  background: #fff;
  border-radius: 12px;
  width: 420px;
  max-width: 90vw;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  overflow: hidden;
}

/* 标题栏 */
.tag-manager-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid #f0f0f0;
}

.tag-manager-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #262626;
}

.close-btn {
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

.close-btn:hover {
  background: #f5f5f5;
  color: #262626;
}

.close-btn svg {
  width: 16px;
  height: 16px;
}

/* 内容区域 */
.tag-manager > div:not(.tag-manager-header):not(.tag-manager-footer) {
  padding: 16px 20px;
}

.section-title {
  font-size: 13px;
  font-weight: 500;
  color: #8c8c8c;
  margin-bottom: 10px;
  display: flex;
  align-items: center;
  gap: 6px;
}

.tag-count {
  font-weight: normal;
  color: #bfbfbf;
}

/* 已选标签 */
.selected-tags-section {
  border-bottom: 1px solid #f0f0f0;
}

.selected-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  min-height: 32px;
}

/* 创建新标签 */
.create-tag-section {
  border-bottom: 1px solid #f0f0f0;
}

.create-tag-input {
  display: flex;
  gap: 8px;
}

.create-tag-input input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #d9d9d9;
  border-radius: 6px;
  font-size: 14px;
  outline: none;
  transition: all 0.2s;
}

.create-tag-input input:focus {
  border-color: #1890ff;
  box-shadow: 0 0 0 2px rgba(24, 144, 255, 0.2);
}

.create-tag-input input::placeholder {
  color: #bfbfbf;
}

.create-btn {
  padding: 8px 16px;
  background: #1890ff;
  color: #fff;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.create-btn:hover:not(:disabled) {
  background: #40a9ff;
}

.create-btn:disabled {
  background: #d9d9d9;
  cursor: not-allowed;
}

.error-message {
  margin-top: 8px;
  font-size: 12px;
  color: #ff4d4f;
}

/* 所有标签 */
.all-tags-section {
  flex: 1;
  overflow-y: auto;
  max-height: 200px;
}

.tags-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

/* 标签样式 */
.tag-chip {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  border-radius: 12px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  border: 1px solid transparent;
  user-select: none;
}

.tag-chip:not(.selected):not(.disabled):hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
}

.tag-chip.selected {
  cursor: default;
}

.tag-chip.disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.tag-chip .remove-btn {
  width: 14px;
  height: 14px;
  border: none;
  background: transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  padding: 0;
  margin-left: 2px;
  opacity: 0.7;
  transition: all 0.15s;
}

.tag-chip .remove-btn:hover {
  opacity: 1;
  background: rgba(0, 0, 0, 0.1);
}

.tag-chip .remove-btn svg {
  width: 10px;
  height: 10px;
}

.tag-usage {
  font-size: 11px;
  font-weight: normal;
  opacity: 0.7;
}

.no-tags-hint {
  font-size: 13px;
  color: #bfbfbf;
  font-style: italic;
}

/* 底部按钮 */
.tag-manager-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 16px 20px;
  border-top: 1px solid #f0f0f0;
  background: #fafafa;
}

.btn {
  padding: 8px 20px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
}

.btn.cancel {
  background: #fff;
  color: #595959;
  border: 1px solid #d9d9d9;
}

.btn.cancel:hover {
  color: #262626;
  border-color: #1890ff;
}

.btn.confirm {
  background: #1890ff;
  color: #fff;
}

.btn.confirm:hover {
  background: #40a9ff;
}

/* 滚动条样式 */
.all-tags-section::-webkit-scrollbar {
  width: 4px;
}

.all-tags-section::-webkit-scrollbar-track {
  background: transparent;
}

.all-tags-section::-webkit-scrollbar-thumb {
  background: #d9d9d9;
  border-radius: 2px;
}

.all-tags-section::-webkit-scrollbar-thumb:hover {
  background: #bfbfbf;
}
</style>
