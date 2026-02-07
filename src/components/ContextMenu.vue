<template>
  <Teleport to="body">
    <div
      v-if="visible"
      class="context-menu-overlay"
      @click="close"
      @contextmenu.prevent="close"
    >
      <div
        class="context-menu"
        :style="menuStyle"
        @click.stop
      >
        <template v-for="menuItem in visibleMenuItems" :key="menuItem.key">
          <!-- 分割线 -->
          <div v-if="menuItem.type === 'divider'" class="menu-divider" />
          
          <!-- 菜单项 -->
          <div
            v-else
            class="menu-item"
            :class="{ danger: menuItem.danger }"
            @click="handleAction(menuItem.key)"
          >
            <span class="menu-icon" v-html="menuItem.icon" />
            <span class="menu-label">{{ menuItem.label }}</span>
            <span v-if="menuItem.shortcut" class="menu-shortcut">
              {{ menuItem.shortcut }}
            </span>
          </div>
        </template>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, watch } from 'vue';
import type { ClipboardItem, ClipboardContentType } from '@/types';

interface MenuItem {
  key: string;
  label: string;
  icon?: string;
  shortcut?: string;
  danger?: boolean;
  type?: 'item' | 'divider';
  visibleFor?: ClipboardContentType[];
}

interface Props {
  visible: boolean;
  position: { x: number; y: number };
  item: ClipboardItem | null;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  'update:visible': [value: boolean];
  action: [action: string, item: ClipboardItem];
}>();

// 菜单项定义
const menuItems: MenuItem[] = [
  {
    key: 'copy',
    label: '复制',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2"/><path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/></svg>',
    shortcut: 'Ctrl+C',
  },
  {
    key: 'paste',
    label: '粘贴',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M16 4h2a2 2 0 012 2v14a2 2 0 01-2 2H6a2 2 0 01-2-2V6a2 2 0 012-2h2"/><rect x="8" y="2" width="8" height="4" rx="1"/></svg>',
    shortcut: 'Ctrl+V',
    visibleFor: ['text', 'html', 'rtf', 'image', 'file'],
  },
  { key: 'divider1', type: 'divider', label: '' },
  {
    key: 'queue',
    label: '添加到队列',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/></svg>',
  },
  {
    key: 'copyPlain',
    label: '复制为纯文本',
    visibleFor: ['text', 'html', 'rtf'],
  },
  { key: 'divider2', type: 'divider', label: '' },
  {
    key: 'openFile',
    label: '打开文件',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/><path d="M14 2v6h6"/></svg>',
    visibleFor: ['image', 'file'],
  },
  {
    key: 'showInFolder',
    label: '在文件夹中显示',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 01-2 2H4a2 2 0 01-2-2V5a2 2 0 012-2h5l2 3h9a2 2 0 012 2z"/></svg>',
    visibleFor: ['image', 'file', 'folder'],
  },
  { key: 'divider3', type: 'divider', label: '' },
  {
    key: 'favorite',
    label: '收藏',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/></svg>',
  },
  {
    key: 'delete',
    label: '删除',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 6h18"/><path d="M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"/></svg>',
    shortcut: 'Del',
    danger: true,
  },
];

// 根据内容类型过滤可见菜单项
const visibleMenuItems = computed(() => {
  if (!props.item) return [];
  
  const contentType = props.item.content_type;
  
  return menuItems.filter(item => {
    // 分割线特殊处理
    if (item.type === 'divider') {
      return true;
    }
    // 如果没有指定 visibleFor，则对所有类型可见
    if (!item.visibleFor) {
      return true;
    }
    // 检查当前类型是否在可见列表中
    return item.visibleFor.includes(contentType);
  }).filter((item, index, arr) => {
    // 移除连续的分割线和首尾的分割线
    if (item.type === 'divider') {
      const prev = arr[index - 1];
      const next = arr[index + 1];
      if (!prev || prev.type === 'divider') return false;
      if (!next || next.type === 'divider') return false;
    }
    return true;
  });
});

// 计算菜单位置，确保不超出视口
const menuStyle = computed(() => {
  const menuWidth = 200;
  const menuHeight = 300;
  
  let x = props.position.x;
  let y = props.position.y;
  
  // 防止超出右边界
  if (x + menuWidth > window.innerWidth) {
    x = window.innerWidth - menuWidth - 10;
  }
  
  // 防止超出下边界
  if (y + menuHeight > window.innerHeight) {
    y = window.innerHeight - menuHeight - 10;
  }
  
  return {
    left: `${x}px`,
    top: `${y}px`,
  };
});

const close = () => {
  emit('update:visible', false);
};

const handleAction = (action: string) => {
  if (props.item) {
    emit('action', action, props.item);
  }
  close();
};

// 更新收藏按钮标签
watch(() => props.item, (newItem) => {
  const favoriteItem = menuItems.find(m => m.key === 'favorite');
  if (favoriteItem && newItem) {
    favoriteItem.label = newItem.tags?.includes('收藏') ? '取消收藏' : '收藏';
  }
}, { immediate: true });
</script>

<style scoped>
.context-menu-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 9999;
}

.context-menu {
  position: fixed;
  min-width: 180px;
  background: #fff;
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  padding: 4px 0;
  z-index: 10000;
}

.menu-item {
  display: flex;
  align-items: center;
  height: 32px;
  padding: 0 12px;
  cursor: pointer;
  transition: background-color 0.15s;
}

.menu-item:hover {
  background-color: #f5f5f5;
}

.menu-item.danger {
  color: #ff4d4f;
}

.menu-item.danger:hover {
  background-color: #fff2f0;
}

.menu-icon {
  width: 16px;
  height: 16px;
  margin-right: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #8c8c8c;
}

.menu-item.danger .menu-icon {
  color: #ff4d4f;
}

.menu-icon :deep(svg) {
  width: 14px;
  height: 14px;
}

.menu-label {
  flex: 1;
  font-size: 13px;
  color: #262626;
}

.menu-item.danger .menu-label {
  color: #ff4d4f;
}

.menu-shortcut {
  font-size: 12px;
  color: #8c8c8c;
  margin-left: 24px;
}

.menu-divider {
  height: 1px;
  background-color: #f0f0f0;
  margin: 4px 0;
}
</style>
