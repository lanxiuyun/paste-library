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
import { computed } from 'vue';
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
// 根据 Test.md 规范：
// - 文本：复制、粘贴、标签、删除
// - HTML/RTF：复制、粘贴、标签、复制为纯文本、粘贴为纯文本、删除
// - 图片：复制、粘贴、标签、打开文件、在文件夹中显示、复制文件路径、删除
// - 单文件：复制、粘贴、标签、打开文件、在文件夹中显示、复制文件路径、删除
// - 多文件：复制、粘贴、标签、复制文件路径、删除
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
  {
    key: 'tag',
    label: '标签',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z"/><line x1="7" y1="7" x2="7.01" y2="7"/></svg>',
  },
  {
    key: 'copyPlain',
    label: '复制为纯文本',
    visibleFor: ['html', 'rtf'], // 只对 HTML/RTF 显示，文本类型不需要
  },
  {
    key: 'pastePlain',
    label: '粘贴为纯文本',
    visibleFor: ['html', 'rtf'], // 只对 HTML/RTF 显示
  },
  {
    key: 'openFile',
    label: '打开文件',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/><path d="M14 2v6h6"/></svg>',
    visibleFor: ['image', 'file'], // 多文件不显示，需要特殊处理
  },
  {
    key: 'showInFolder',
    label: '在文件夹中显示',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 01-2 2H4a2 2 0 01-2-2V5a2 2 0 012-2h5l2 3h9a2 2 0 012 2z"/></svg>',
    visibleFor: ['image', 'file', 'folder'], // 多文件不显示，需要特殊处理
  },
  {
    key: 'copyFilePath',
    label: '复制文件路径',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/></svg>',
    visibleFor: ['image', 'file', 'folder', 'files'], // 图片也需要
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
  const isMultiFiles = contentType === 'files' && (props.item.file_paths?.length || 0) > 1;

  // 第一步：过滤掉不可见的菜单项
  const filtered = menuItems.filter(item => {
    // 分割线特殊处理 - 先不过滤，后面统一处理
    if (item.type === 'divider') {
      return true;
    }

    // 多文件不显示"打开文件"和"在文件夹中显示"
    if (isMultiFiles && (item.key === 'openFile' || item.key === 'showInFolder')) {
      return false;
    }

    // 如果没有指定 visibleFor，则对所有类型可见
    if (!item.visibleFor) {
      return true;
    }

    // 检查当前类型是否在可见列表中
    return item.visibleFor.includes(contentType);
  });

  // 第二步：移除所有分割线，然后重新根据剩余的菜单项添加分割线
  const withoutDividers = filtered.filter(item => item.type !== 'divider');

  // 第三步：重新构建带分割线的菜单
  const result: MenuItem[] = [];

  // 第一组：复制、粘贴、标签
  const group1 = ['copy', 'paste', 'tag'];
  const group1Items = withoutDividers.filter(item => group1.includes(item.key));
  if (group1Items.length > 0) {
    result.push(...group1Items);
    result.push({ key: 'divider_group1', type: 'divider', label: '' });
  }

  // 第二组：复制/粘贴为纯文本（仅HTML/RTF）
  const group2 = ['copyPlain', 'pastePlain'];
  const group2Items = withoutDividers.filter(item => group2.includes(item.key));
  if (group2Items.length > 0) {
    result.push(...group2Items);
    result.push({ key: 'divider_group2', type: 'divider', label: '' });
  }

  // 第三组：文件操作（打开文件、在文件夹中显示、复制文件路径）
  const group3 = ['openFile', 'showInFolder', 'copyFilePath'];
  const group3Items = withoutDividers.filter(item => group3.includes(item.key));
  if (group3Items.length > 0) {
    result.push(...group3Items);
    result.push({ key: 'divider_group3', type: 'divider', label: '' });
  }

  // 第四组：删除
  const deleteItem = withoutDividers.find(item => item.key === 'delete');
  if (deleteItem) {
    result.push(deleteItem);
  }

  return result;
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
