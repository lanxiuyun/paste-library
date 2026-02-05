<template>
  <div class="clipboard-card" @click="handleClick">
    <div class="card-header">
      <div class="header-left">
        <span class="type-badge" :class="item.content_type">{{ typeLabel }}</span>
        <span class="char-count">{{ charCount }}个字符</span>
        <span class="time">{{ formattedTime }}</span>
      </div>
      <div class="header-right">
        <button 
          class="icon-btn" 
          :class="{ 'is-favorite': isFavorite }"
          @click.stop="handleToggleFavorite"
          title="收藏"
        >
          <svg v-if="isFavorite" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
          </svg>
          <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
          </svg>
        </button>
        <button class="icon-btn delete" @click.stop="handleDelete" title="删除">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"/>
          </svg>
        </button>
      </div>
    </div>
    <div class="card-content">
      <div class="content-text">{{ contentPreview }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import type { ClipboardItem } from '@/types';

interface Props {
  item: ClipboardItem;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  click: [item: ClipboardItem];
  delete: [id: number];
  toggleFavorite: [id: number, isFavorite: boolean];
}>();

const isFavorite = ref(false);

const typeLabel = computed(() => {
  switch (props.item.content_type) {
    case 'text':
      return '纯文本';
    case 'html':
      return 'HTML';
    case 'rtf':
      return '富文本';
    default:
      return '文本';
  }
});

const charCount = computed(() => {
  return props.item.content.length;
});

const contentPreview = computed(() => {
  let text = props.item.content;
  text = text.replace(/<[^>]*>/g, '');
  text = text.trim();
  return text || '(空内容)';
});

const formattedTime = computed(() => {
  const date = new Date(props.item.created_at);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffSec = Math.floor(diffMs / 1000);
  const diffMin = Math.floor(diffSec / 60);
  const diffHour = Math.floor(diffMin / 60);
  const diffDay = Math.floor(diffHour / 24);

  if (diffSec < 60) return '刚刚';
  if (diffMin < 60) return `${diffMin}分钟前`;
  if (diffHour < 24) return `${diffHour}小时前`;
  if (diffDay < 7) return `${diffDay}天前`;
  return date.toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' });
});

const handleClick = () => {
  emit('click', props.item);
};

const handleDelete = () => {
  emit('delete', props.item.id);
};

const handleToggleFavorite = () => {
  isFavorite.value = !isFavorite.value;
  emit('toggleFavorite', props.item.id, isFavorite.value);
};
</script>

<style scoped>
.clipboard-card {
  background: #fff;
  border-radius: 8px;
  padding: 12px 16px;
  margin-bottom: 12px;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.06), 0 1px 3px rgba(0, 0, 0, 0.1);
  border: 1px solid #f0f0f0;
  cursor: pointer;
  transition: all 0.2s ease;
}

.clipboard-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  border-color: #d9d9d9;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.type-badge {
  font-size: 12px;
  padding: 2px 8px;
  border-radius: 4px;
  font-weight: 500;
}

.type-badge.text {
  background: #f6ffed;
  color: #52c41a;
}

.type-badge.html {
  background: #e6f7ff;
  color: #1890ff;
}

.type-badge.rtf {
  background: #fff7e6;
  color: #fa8c16;
}

.char-count,
.time {
  font-size: 12px;
  color: #999;
}

.header-right {
  display: flex;
  gap: 4px;
  opacity: 0;
  transition: opacity 0.2s;
}

.clipboard-card:hover .header-right {
  opacity: 1;
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
  color: #999;
  transition: all 0.2s;
}

.icon-btn:hover {
  background: #f5f5f5;
  color: #666;
}

.icon-btn svg {
  width: 16px;
  height: 16px;
}

.icon-btn.is-favorite {
  color: #faad14;
  opacity: 1;
}

.icon-btn.delete:hover {
  color: #ff4d4f;
  background: #fff1f0;
}

.card-content {
  padding-top: 4px;
}

.content-text {
  font-size: 14px;
  line-height: 1.6;
  color: #333;
  word-break: break-all;
  white-space: pre-wrap;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>