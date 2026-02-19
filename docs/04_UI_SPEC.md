# 04 - UI/UX 设计规范

> **范围**: 颜色、间距、字体、组件、布局  
> **目标**: 保持视觉一致性  
> **最后更新**: 2026-02-19

---

## 一、颜色规范

### 1.1 主色调

```css
--primary: #262626;           /* 主按钮、激活状态 */
--primary-hover: #404040;     /* 主按钮悬停 */
--primary-light: #595959;     /* 次要按钮 */
```

### 1.2 背景色

```css
--bg-page: #f5f5f5;           /* 页面背景 */
--bg-card: #ffffff;           /* 卡片背景 */
--bg-sidebar: #ffffff;        /* 侧边栏背景 */
--bg-hover: #f5f5f5;          /* 悬停背景 */
--bg-active: #e6f7ff;         /* 选中背景 */
```

### 1.3 文字色

```css
--text-primary: #262626;      /* 主要文字 */
--text-secondary: #595959;    /* 次要文字 */
--text-desc: #8c8c8c;         /* 描述文字 */
--text-disabled: #bfbfbf;     /* 禁用文字 */
```

### 1.4 边框色

```css
--border: #d9d9d9;            /* 输入框边框 */
--border-light: #e8e8e8;      /* 分割线 */
--border-hover: #262626;      /* 悬停边框 */
```

### 1.5 状态色

```css
--danger: #ff4d4f;            /* 危险操作 */
--danger-hover: #ff7875;      /* 危险操作悬停 */
--danger-bg: #fff2f0;         /* 危险按钮背景 */
--success: #52c41a;           /* 成功状态 */
--warning: #faad14;           /* 警告状态 */
--info: #1890ff;              /* 信息状态 */
```

### 1.6 类型标签色

| 类型 | 背景色 | 文字色 |
|------|--------|--------|
| text | `#fff2e8` | `#fa8c16` |
| html | `#e6f7ff` | `#1890ff` |
| rtf | `#f6ffed` | `#52c41a` |
| image | `#f6ffed` | `#52c41a` |
| file | `#f9f0ff` | `#722ed1` |
| folder | `#f9f0ff` | `#722ed1` |
| files | `#fff0f6` | `#eb2f96` |

### 1.7 搜索高亮

```css
--search-highlight: #fff566;  /* 关键词高亮背景 */
```

---

## 二、间距规范

### 2.1 间距变量

| 名称 | 值 | 用途 |
|------|-----|------|
| `--space-xs` | 4px | 图标间距、紧凑元素 |
| `--space-sm` | 8px | 组件内部间距 |
| `--space-md` | 12px | 列表项内边距 |
| `--space-lg` | 16px | 卡片间距、分组间距 |
| `--space-xl` | 24px | 页面边距、分组间距 |
| `--space-xxl` | 32px | 大模块间距 |

### 2.2 组件间距

**剪贴板列表项**:
- padding: 12px 16px
- 元素间距: 8px
- 类型标签与内容间距: 12px

**设置面板**:
- 侧边栏 padding: 12px
- 内容区 padding: 24px 32px
- 分组间距: 24px
- 设置项间距: 0（使用分割线）
- 设置项内边距: 16px 0

**弹窗/面板**:
- padding: 16px-24px
- 按钮组间距: 12px
- 表单元素间距: 16px

---

## 三、字体规范

### 3.1 字体族

```css
font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 
             'Helvetica Neue', Arial, 'Noto Sans', sans-serif;
```

### 3.2 字号规范

| 用途 | 字号 | 字重 | 行高 |
|------|------|------|------|
| 页面标题 | 16px | 600 | 1.4 |
| 分组标题 | 16px | 600 | 1.4 |
| 设置项标题 | 14px | 500 | 1.5 |
| 正文 | 14px | 400 | 1.5 |
| 描述文字 | 12px | 400 | 1.5 |
| 辅助文字 | 11px | 400 | 1.4 |
| 标签文字 | 12px | 500 | 1 |

---

## 四、组件规范

### 4.1 按钮

**主按钮**:
```css
.btn-primary {
  padding: 8px 16px;
  background: var(--primary);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-primary:hover {
  background: var(--primary-hover);
}
```

**次要按钮**:
```css
.btn-secondary {
  padding: 8px 16px;
  background: white;
  color: var(--text-secondary);
  border: 1px solid var(--border);
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary:hover {
  border-color: var(--primary);
  color: var(--primary);
}
```

**危险按钮**:
```css
.btn-danger {
  padding: 8px 16px;
  background: var(--danger-bg);
  color: var(--danger);
  border: 1px solid #ffccc7;
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
}

.btn-danger:hover {
  background: #ffccc7;
}
```

**图标按钮**:
```css
.btn-icon {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  color: var(--text-secondary);
}

.btn-icon:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}
```

### 4.2 开关 (Switch)

```css
.switch {
  width: 44px;
  height: 24px;
  border-radius: 24px;
  background: #d9d9d9;
  position: relative;
  cursor: pointer;
  transition: background 0.2s;
}

.switch.active {
  background: var(--primary);
}

.switch-thumb {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: white;
  position: absolute;
  top: 3px;
  left: 3px;
  transition: transform 0.2s;
}

.switch.active .switch-thumb {
  transform: translateX(20px);
}
```

### 4.3 输入框

**基础输入框**:
```css
.input {
  height: 32px;
  padding: 6px 12px;
  border-radius: 4px;
  border: 1px solid var(--border);
  font-size: 13px;
  outline: none;
  transition: border-color 0.2s;
}

.input:focus {
  border-color: var(--primary);
}

.input::placeholder {
  color: var(--text-disabled);
}
```

**搜索框**:
```css
.search-input {
  height: 36px;
  padding: 6px 32px 6px 12px;
  border-radius: 6px;
  border: 1px solid var(--border);
  font-size: 14px;
  background: white;
}

.search-input:focus {
  border-color: var(--primary);
}

.search-clear-btn {
  position: absolute;
  right: 8px;
  top: 50%;
  transform: translateY(-50%);
}
```

### 4.4 下拉选择

```css
.select {
  padding: 6px 28px 6px 10px;
  border: 1px solid var(--border);
  border-radius: 4px;
  font-size: 13px;
  background: white;
  min-width: 120px;
  cursor: pointer;
  appearance: none;
  background-image: url('arrow-down.svg');
  background-repeat: no-repeat;
  background-position: right 8px center;
}

.select:hover {
  border-color: var(--border-hover);
}
```

### 4.5 标签 (Tag)

```css
.tag {
  display: inline-flex;
  align-items: center;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
}

.tag-removable {
  padding-right: 4px;
}

.tag-remove-btn {
  margin-left: 4px;
  width: 14px;
  height: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
}

.tag-remove-btn:hover {
  background: rgba(0,0,0,0.1);
}
```

### 4.6 类型标签

```css
.type-tag {
  display: inline-flex;
  align-items: center;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 500;
  text-transform: uppercase;
}

.type-tag-text { background: #fff2e8; color: #fa8c16; }
.type-tag-html { background: #e6f7ff; color: #1890ff; }
.type-tag-rtf { background: #f6ffed; color: #52c41a; }
.type-tag-image { background: #f6ffed; color: #52c41a; }
.type-tag-file { background: #f9f0ff; color: #722ed1; }
.type-tag-folder { background: #f9f0ff; color: #722ed1; }
.type-tag-files { background: #fff0f6; color: #eb2f96; }
```

### 4.7 列表项

```css
.list-item {
  padding: 12px 16px;
  background: white;
  border-bottom: 1px solid var(--border-light);
  cursor: pointer;
  transition: background 0.15s;
}

.list-item:hover {
  background: var(--bg-hover);
}

.list-item.selected {
  background: var(--bg-active);
}

.list-item-content {
  display: flex;
  align-items: flex-start;
  gap: 12px;
}

.list-item-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 4px;
  font-size: 12px;
  color: var(--text-desc);
}
```

### 4.8 上下文菜单

```css
.context-menu {
  min-width: 180px;
  background: white;
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
  padding: 4px 0;
}

.context-menu-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 12px;
  height: 32px;
  cursor: pointer;
  font-size: 13px;
}

.context-menu-item:hover {
  background: var(--bg-hover);
}

.context-menu-divider {
  height: 1px;
  background: var(--border-light);
  margin: 4px 0;
}

.context-menu-shortcut {
  font-size: 12px;
  color: var(--text-disabled);
}
```

---

## 五、布局规范

### 5.1 剪贴板窗口

```
┌─ Clipboard Window (800x600) ──────────────────────────┐
│  ┌─ Header (48px) ──────────────────────────────────┐ │
│  │ [搜索框]                     [设置] [关闭]        │ │
│  └──────────────────────────────────────────────────┘ │
│  ┌─ Tabs (40px) ────────────────────────────────────┐ │
│  │ [全部] [文本] [HTML] [图片] [文件]               │ │
│  └──────────────────────────────────────────────────┘ │
│  ┌─ List Area (flex:1) ──────────────────────────────┐ │
│  │  可滚动列表                                       │ │
│  └──────────────────────────────────────────────────┘ │
│  ┌─ Status Bar (32px) ──────────────────────────────┐ │
│  │  共 100 条记录                                    │ │
│  └──────────────────────────────────────────────────┘ │
└───────────────────────────────────────────────────────┘
```

### 5.2 设置面板

```
┌─ Settings Panel (600x700) ──────────────────────────┐
│ ┌─ Sidebar (220px) ┌─ Content Area (flex:1) ──────┐│
│ │                  │                                ││
│ │ [剪贴板]         │  [窗口设置]                    ││
│ │ [历史记录]       │  ─────────────────────────    ││
│ │ [通用设置]       │  设置项 1              [开关] ││
│ │ [快捷键]         │  描述文字                      ││
│ │ [数据备份]       │  ─────────────────────────    ││
│ │ [关于]           │  设置项 2              [选择] ││
│ │                  │  描述文字                      ││
│ └──────────────────┘                                ││
│                     └────────────────────────────────┘│
└───────────────────────────────────────────────────────┘
```

### 5.3 剪贴板列表项

```
┌─ Clipboard Item ──────────────────────────────────┐
│ padding: 12px 16px                                │
│ ┌─ Type Tag ─┐ ┌─ Content Area ─────────────────┐ │
│ │            │ │                                 │ │
│ │  40px      │ │  [内容预览]                     │ │
│ │  square    │ │                                 │ │
│ │            │ │  [元信息: 时间 · 标签]          │ │
│ └────────────┘ │                                 │ │
│                └─────────────────────────────────┘ │
└────────────────────────────────────────────────────┘
```

---

## 六、动画规范

### 6.1 过渡动画

| 动画 | 时长 | 缓动函数 |
|------|------|----------|
| 按钮悬停 | 200ms | ease |
| 开关切换 | 200ms | ease |
| 颜色变化 | 150ms | ease |
| 列表项悬停 | 150ms | ease |

### 6.2 入场动画

| 动画 | 时长 | 缓动函数 |
|------|------|----------|
| 抽屉滑入 | 300ms | cubic-bezier(0.4, 0, 0.2, 1) |
| 菜单显示 | 150ms | ease-out |
| 弹窗显示 | 200ms | ease-out |
| Toast 显示 | 300ms | ease-out |

### 6.3 CSS 示例

```css
/* 抽屉滑入 */
.drawer-enter {
  transform: translateX(100%);
}

.drawer-enter-active {
  transform: translateX(0);
  transition: transform 300ms cubic-bezier(0.4, 0, 0.2, 1);
}

/* 淡入 */
.fade-enter {
  opacity: 0;
}

.fade-enter-active {
  opacity: 1;
  transition: opacity 200ms ease;
}

/* 缩放 */
.scale-enter {
  transform: scale(0.95);
  opacity: 0;
}

.scale-enter-active {
  transform: scale(1);
  opacity: 1;
  transition: all 200ms ease;
}
```

---

## 七、响应式说明

### 7.1 窗口尺寸适配

**剪贴板窗口**:
- 默认: 800x600
- 最小: 400x300
- 最大: 无限制

**设置窗口**:
- 默认: 600x700
- 最小: 400x500
- 可调整大小

### 7.2 内容适配

- 列表项宽度: 100% 自适应
- 搜索框宽度: 100% 自适应
- 设置项: 水平布局，移动端可改为垂直

---

## 八、图标规范

### 8.1 图标尺寸

| 用途 | 尺寸 |
|------|------|
| 工具栏图标 | 20px |
| 按钮图标 | 16px |
| 列表图标 | 14px |
| 类型标签图标 | 12px |

### 8.2 图标库

推荐使用:
- **Lucide Icons** (推荐) - 简洁现代
- **Heroicons** -  Tailwind 风格
- **Phosphor Icons** - 多风格可选

---

## 相关文档

- [01_OVERVIEW.md](./01_OVERVIEW.md) - 项目概述
- [02_PRD.md](./02_PRD.md) - 产品需求
- [03_ARCHITECTURE.md](./03_ARCHITECTURE.md) - 技术架构
- [05_DATA_MODEL.md](./05_DATA_MODEL.md) - 数据模型
- [06_IMPLEMENTATION.md](./06_IMPLEMENTATION.md) - 实施计划
