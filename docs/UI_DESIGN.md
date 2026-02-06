# Paste Library - UI 设计规范

## 颜色规范

### 主色调
```css
--primary: #262626;           /* 主按钮、激活状态 */
--primary-hover: #404040;     /* 主按钮悬停 */
--primary-light: #595959;     /* 次要按钮 */
```

### 背景色
```css
--bg-page: #f5f5f5;           /* 页面背景 */
--bg-card: #ffffff;           /* 卡片背景 */
--bg-sidebar: #ffffff;        /* 侧边栏背景 */
--bg-hover: #f5f5f5;          /* 悬停背景 */
```

### 文字色
```css
--text-primary: #262626;      /* 主要文字 */
--text-secondary: #595959;    /* 次要文字 */
--text-desc: #8c8c8c;         /* 描述文字 */
--text-disabled: #bfbfbf;     /* 禁用文字 */
```

### 边框色
```css
--border: #d9d9d9;            /* 输入框边框 */
--border-light: #e8e8e8;      /* 分割线 */
--border-hover: #262626;      /* 悬停边框 */
```

### 状态色
```css
--danger: #ff4d4f;            /* 危险操作 */
--danger-hover: #ff7875;      /* 危险操作悬停 */
--danger-bg: #fff2f0;         /* 危险按钮背景 */
--danger-border: #ffccc7;     /* 危险按钮边框 */

--success: #52c41a;           /* 成功状态 */
--warning: #faad14;           /* 警告状态 */
--info: #1890ff;              /* 信息状态 */
```

---

## 组件规范

### 开关 (Switch)
```css
.switch {
  width: 44px;
  height: 24px;
  border-radius: 24px;
  background: #d9d9d9;
  transition: 0.2s;
}

.switch.active {
  background: #262626;
}

.switch-thumb {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: white;
  transform: translateX(3px);
}

.switch.active .switch-thumb {
  transform: translateX(23px);
}
```

### 输入框 (Input)
```css
.input {
  height: 32px;
  padding: 6px 12px;
  border-radius: 4px;
  border: 1px solid var(--border);
  font-size: 13px;
}

.input:focus {
  border-color: var(--primary);
  outline: none;
}
```

### 数字输入+单位
```css
.number-with-unit {
  display: flex;
  align-items: center;
  border: 1px solid var(--border);
  border-radius: 4px;
}

.number-with-unit input {
  width: 60px;
  border: none;
  text-align: center;
}

.number-with-unit .unit {
  padding: 0 12px;
  border-left: 1px solid var(--border);
}
```

### 按钮

**主按钮**
```css
.btn-primary {
  padding: 8px 16px;
  background: var(--primary);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 13px;
}

.btn-primary:hover {
  background: var(--primary-hover);
}
```

**次要按钮**
```css
.btn-secondary {
  padding: 8px 16px;
  background: white;
  color: var(--text-secondary);
  border: 1px solid var(--border);
  border-radius: 6px;
  font-size: 13px;
}

.btn-secondary:hover {
  border-color: var(--primary);
  color: var(--primary);
}
```

**危险按钮**
```css
.btn-danger {
  padding: 8px 16px;
  background: var(--danger-bg);
  color: var(--danger);
  border: 1px solid var(--danger-border);
  border-radius: 6px;
  font-size: 13px;
}

.btn-danger:hover {
  background: #ffccc7;
}
```

### 下拉选择
```css
.select {
  padding: 6px 28px 6px 10px;
  border: 1px solid var(--border);
  border-radius: 4px;
  font-size: 13px;
  background: white;
  min-width: 120px;
}
```

### 图标卡片按钮
```css
.icon-card {
  width: 120px;
  height: 100px;
  border-radius: 8px;
  border: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 8px;
  cursor: pointer;
}

.icon-card:hover {
  border-color: var(--primary);
}

.icon-card .icon {
  font-size: 24px;
}

.icon-card .label {
  font-size: 13px;
}
```

---

## 布局规范

### 设置面板布局
```
┌─────────────────────────────────────┐
│  Sidebar  │      Content Area       │
│   220px   │        flex: 1          │
│           │                         │
│  [导航项]  │  [分组标题]             │
│  [导航项]  │                         │
│  [导航项]  │  ┌─ 设置项 ──────────┐  │
│           │  │ 标题        [控件] │  │
│           │  │ 描述               │  │
│           │  └───────────────────┘  │
│           │                         │
│           │  [底部操作栏]           │
└─────────────────────────────────────┘
```

### 剪贴板列表项布局
```
┌─ Clipboard Item ──────────────────────────────┐
│ [类型标签] [内容预览]                    [序号] │
│            [元信息: 字符数·时间·收藏]           │
└───────────────────────────────────────────────┘
```

---

## 间距规范

### 设置面板
- 侧边栏 padding: 12px
- 内容区 padding: 24px 32px
- 分组间距: 24px
- 设置项间距: 0 (使用分割线)
- 设置项内边距: 16px 0

### 通用间距
- xs: 4px
- sm: 8px
- md: 12px
- lg: 16px
- xl: 24px
- xxl: 32px

---

## 字体规范

### 字体族
```css
font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 
             'Helvetica Neue', Arial, sans-serif;
```

### 字号
- 页面标题: 16px, font-weight: 600
- 分组标题: 16px, font-weight: 600
- 设置项标题: 14px, font-weight: 500
- 描述文字: 12px
- 辅助文字: 11px

---

## 类型标签颜色

| 类型 | 背景色 | 文字色 |
|------|--------|--------|
| text | #fff2e8 | #fa8c16 |
| html | #e6f7ff | #1890ff |
| rtf | #f6ffed | #52c41a |
| image | #f6ffed | #52c41a |
| file | #f9f0ff | #722ed1 |
| folder | #f9f0ff | #722ed1 |
| files | #fff0f6 | #eb2f96 |

---

## 上下文菜单规范

```
┌─ Context Menu ────────────────┐
│  [图标] 复制            Ctrl+C │
│  [图标] 粘贴            Ctrl+V │
│  ─────────────────────────────│
│       复制为纯文本            │
│  ─────────────────────────────│
│  [图标] 打开文件              │
│  [图标] 在文件夹中显示        │
│  ─────────────────────────────│
│  [图标] ☆ 收藏               │
│  [图标] 删除              Del  │
└───────────────────────────────┘
```

**样式**
- 最小宽度: 180px
- 背景: white
- 阴影: 0 4px 12px rgba(0,0,0,0.15)
- 圆角: 6px
- 菜单项高度: 32px
- 菜单项 padding: 0 12px
- 分割线 margin: 4px 0
