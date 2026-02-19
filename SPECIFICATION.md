# Paste Library - 功能规格说明书

> **版本**: 1.0.0  
> **用途**: 完整功能规格，供重写开发参考  
> **最后更新**: 2026-02-19

---

## 目录

1. [项目概述](#一项目概述)
2. [功能模块总览](#二功能模块总览)
3. [核心剪贴板功能](#三核心剪贴板功能)
4. [UI 界面规格](#四ui-界面规格)
5. [交互操作规格](#五交互操作规格)
6. [标签系统](#六标签系统)
7. [粘贴队列](#七粘贴队列)
8. [抽屉编辑器](#八抽屉编辑器)
9. [设置面板](#九设置面板)
10. [系统托盘](#十系统托盘)
11. [窗口管理](#十一窗口管理)
12. [智能功能](#十二智能功能)
13. [数据备份](#十三数据备份)
14. [技术架构](#十四技术架构)
15. [数据模型](#十五数据模型)
16. [UI 设计规范](#十六ui-设计规范)
17. [开发建议](#十七开发建议)
18. [多设备局域网同步设计](#十八多设备局域网同步设计)
19. [功能优先级总结](#十九功能优先级总结)

---

## 一、项目概述

**Paste Library** 是一款现代化的剪贴板管理工具，实时监听系统剪贴板，保存历史记录，支持快速查询和管理。

### 1.1 技术栈

- **前端框架**: Vue 3 (Composition API + `<script setup>`)
- **语言**: TypeScript (严格模式)
- **构建工具**: Vite
- **桌面框架**: Tauri v2
- **后端语言**: Rust
- **数据库**: SQLite (via rusqlite)
- **插件依赖**:
  - `tauri-plugin-clipboard-x` - 剪贴板监听
  - `tauri-plugin-global-shortcut` - 全局快捷键
  - `tauri-plugin-tray` - 系统托盘
  - `tauri-plugin-autostart` - 开机自启

### 1.2 窗口设计

| 窗口 | 类型 | 特性 |
|------|------|------|
| **剪贴板窗口** | 弹出式 | 无边框、置顶、不在任务栏显示、失焦自动隐藏 |
| **设置窗口** | 正常窗口 | 有标题栏、可调整大小、居中显示 |

---

## 二、功能模块总览

### 2.1 功能优先级

| 优先级 | 功能 | 状态 |
|--------|------|------|
| **P0** | 核心剪贴板功能 | ✅ 必须实现 |
| **P0** | 基础交互与导航 | ✅ 必须实现 |
| **P1** | 搜索与过滤 | ✅ 必须实现 |
| **P1** | 标签系统 | ✅ 必须实现 |
| **P1** | 设置面板 | ✅ 必须实现 |
| **P1** | 系统托盘 | ✅ 必须实现 |
| **P2** | 粘贴队列 | ⭕ 可选 |
| **P2** | 抽屉编辑器 | ⭕ 可选 |
| **P2** | 智能功能 | ⭕ 可选 |
| **P3** | 虚拟滚动优化 | 📋 待实现 |
| **P3** | 多语言/主题 | 📋 待实现 |

### 2.2 实现建议

**第一阶段（MVP）**: 核心剪贴板 + 基础搜索 + 基础设置  
**第二阶段**: 标签系统 + 高级搜索 + 系统托盘  
**第三阶段**: 粘贴队列 + 抽屉编辑器 + 智能功能  
**第四阶段**: 性能优化 + 多语言 + 主题

---

## 三、核心剪贴板功能

### 3.1 支持的内容类型

| 类型 | 标识 | 存储方式 | 显示方式 |
|------|------|----------|----------|
| **纯文本** | `text` | 数据库存储 | 文本预览（最多3行） |
| **HTML** | `html` | 数据库存储 | 纯文本预览（HTML 标签剥离） |
| **RTF** | `rtf` | 数据库存储 | 纯文本预览 |
| **图片** | `image` | 文件系统缩略图 | 缩略图 + 尺寸信息 |
| **文件** | `file` | 路径存储 | 图标 + 文件路径 |
| **文件夹** | `folder` | 路径存储 | 文件夹图标 + 路径 |
| **多文件** | `files` | 路径列表 JSON | 前3个文件路径列表 |

### 3.2 数据流程

```
剪贴板变化
    │
    ├─→ 文本/HTML/RTF → 提取纯文本内容 → 计算 SHA256 → 存储到数据库
    │
    ├─→ 图片 → 获取图片数据 → 生成缩略图 → 保存到文件系统 → 存储路径到数据库
    │
    └─→ 文件/文件夹 → 获取路径列表 → 判断类型 → 存储路径+元数据到数据库
```

### 3.3 核心功能点

#### 3.3.1 实时监听
- 使用 `tauri-plugin-clipboard-x` 监听系统剪贴板变化
- 支持文本、HTML、RTF、图片、文件等多种格式
- 变化时自动触发数据处理和存储

#### 3.3.2 数据去重
- 基于内容 SHA256 哈希计算
- 重复内容更新 `created_at` 时间并置顶
- 可选：保留原始创建时间或更新时间

#### 3.3.3 缩略图处理
- **规格**: 200x200px, PNG 格式, 80% 质量
- **路径**: `{app_data}/thumbnails/{item_id}.png`
- **生成时机**: 图片类型剪贴板变化时
- **加载方式**: 使用 `convertFileSrc` 转换为可访问 URL

#### 3.3.4 数据持久化
- 数据库: SQLite
- 表结构见 [数据模型](#十五数据模型) 章节
- 支持最大历史数限制和自动清理

---

## 四、UI 界面规格

### 4.1 剪贴板窗口布局

```
┌─ Clipboard Window (800x600) ──────────────────────────┐
│  ┌─ Header ─────────────────────────────────────────┐ │
│  │ [搜索框]                     [设置] [关闭]        │ │
│  └──────────────────────────────────────────────────┘ │
│  ┌─ Tabs ───────────────────────────────────────────┐ │
│  │ [全部] [文本] [HTML] [图片] [文件]               │ │
│  └──────────────────────────────────────────────────┘ │
│  ┌─ List Area ──────────────────────────────────────┐ │
│  │ ┌─ Item 1 ─────────────────────────────────────┐ │ │
│  │ │ [text] 这是剪贴板内容预览...          [1]    │ │ │
│  │ │        100字符 · 10:30 · [标签1] [标签2]      │ │ │
│  │ └──────────────────────────────────────────────┘ │ │
│  │ ┌─ Item 2 ─────────────────────────────────────┐ │ │
│  │ │ [image] [缩略图] 1920x1080 · PNG      [2]    │ │ │
│  │ │        [工作] [截图]                          │ │ │
│  │ └──────────────────────────────────────────────┘ │ │
│  │ ...                                             │ │
│  └──────────────────────────────────────────────────┘ │
│  ┌─ Status Bar ─────────────────────────────────────┐ │
│  │ 共 100 条记录                                    │ │
│  └──────────────────────────────────────────────────┘ │
└───────────────────────────────────────────────────────┘
```

### 4.2 剪贴板列表项 (ClipboardItem)

#### 4.2.1 布局结构

```
┌─ Clipboard Item ──────────────────────────────────────────┐
│ ┌─ Main Area ───────────────────────────────────────────┐ │
│ │ [类型标签] [内容预览区]                      [序号]   │ │
│ │            [元信息: 字符数·时间·标签列表]              │ │
│ └───────────────────────────────────────────────────────┘ │
│ ┌─ Hover Actions (显示条件: hover) ─────────────────────┐ │
│ │ [详情] [添加到队列] [复制] [标签] [删除]              │ │
│ └───────────────────────────────────────────────────────┘ │
└───────────────────────────────────────────────────────────┘
```

#### 4.2.2 内容预览规则

| 类型 | 预览内容 | 最大行数 |
|------|----------|----------|
| text | 纯文本内容 | 3行 |
| html | HTML 剥离标签后的纯文本 | 3行 |
| rtf | 提取的纯文本 | 3行 |
| image | 缩略图图片 + 尺寸信息 | 自适应 |
| file | 文件图标 + 文件名 | 1行 |
| folder | 文件夹图标 + 文件夹名 | 1行 |
| files | 文件数量 + 前3个文件名 | 3行 |

#### 4.2.3 类型标签样式

| 类型 | 背景色 | 文字色 |
|------|--------|--------|
| text | `#fff2e8` | `#fa8c16` (橙色) |
| html | `#e6f7ff` | `#1890ff` (蓝色) |
| rtf | `#f6ffed` | `#52c41a` (绿色) |
| image | `#f6ffed` | `#52c41a` (绿色) |
| file | `#f9f0ff` | `#722ed1` (紫色) |
| folder | `#f9f0ff` | `#722ed1` (紫色) |
| files | `#fff0f6` | `#eb2f96` (粉色) |

#### 4.2.4 元信息显示

- **文本类**: `{字符数}字符 · {时间} · [标签列表]`
- **图片类**: `{宽度}x{高度} · {格式} · {时间} · [标签列表]`
- **文件类**: `{文件大小} · {时间} · [标签列表]`
- **文件夹类**: `{项目数}个项目 · {时间} · [标签列表]`
- **多文件类**: `{文件数}个文件 · {时间} · [标签列表]`

### 4.3 搜索功能

#### 4.3.1 搜索框

- **位置**: 可配置 (顶部/底部)
- **占位符**: "搜索剪贴板历史..."
- **清除按钮**: 输入不为空时显示 X 按钮
- **快捷键**: `/` 或 `Ctrl+F` 聚焦搜索框

#### 4.3.2 模糊搜索

- 支持拼音搜索（如输入 "hb" 匹配 "花瓣"）
- 支持首字母搜索
- 容错匹配（编辑距离）
- 搜索结果按时间倒序排列

#### 4.3.3 智能搜索语法

| 语法 | 示例 | 说明 |
|------|------|------|
| `@标签名` | `@工作` | 搜索包含该标签的项 |
| `@类型` | `@图片` | 按类型过滤（@文本/@html/@图片/@文件/@文件夹） |
| 组合搜索 | `设计 @工作` | 关键词 + 标签组合 |
| 组合搜索 | `@图片 @截图` | 类型 + 标签组合 |

#### 4.3.4 搜索建议面板

```
┌─ Search Suggestions ────────────────────────┐
│  最近搜索                                   │
│  ├─ 关键词1                          [×]    │
│  ├─ @工作                              [×]    │
│  └─ 清空历史                                │
│  ─────────────────────────────────────────  │
│  热门标签                                   │
│  [工作] [截图] [代码] [链接]                │
│  ─────────────────────────────────────────  │
│  搜索语法提示                               │
│  • @标签名 - 按标签搜索                     │
│  • @类型 - 按类型搜索 (@文本/@图片)         │
└─────────────────────────────────────────────┘
```

### 4.4 标签页过滤

| 标签页 | 显示内容 |
|--------|----------|
| **全部** | 所有类型的剪贴板项 |
| **文本** | text + html + rtf |
| **HTML** | 仅 html |
| **图片** | 仅 image |
| **文件** | file + files |

---

## 五、交互操作规格

### 5.1 鼠标操作

| 操作 | 默认行为 | 可配置 |
|------|----------|--------|
| **左键单击** | 复制 | ✅ 通过 `click_action` 设置 |
| **左键双击** | 粘贴 | ✅ 通过 `double_click_action` 设置 |
| **右键** | 打开上下文菜单 | ❌ |
| **拖拽** | 跨应用拖拽内容 | ❌ |

#### 5.1.1 动作说明

- **复制 (Copy)**: 仅将数据写入系统剪贴板
- **粘贴 (Paste)**: 复制数据 → 隐藏窗口 → 模拟快捷键粘贴到原焦点窗口
- **不操作 (None)**: 仅选中该项，不执行复制或粘贴

#### 5.1.2 点击动作配置

```typescript
// 单击动作
click_action: 'copy' | 'paste' | 'none'

// 双击动作
double_click_action: 'copy' | 'paste' | 'none'
```

#### 5.1.3 粘贴快捷键模拟

- 支持两种快捷键模式:
  - `ctrl_v` - Ctrl+V
  - `shift_insert` - Shift+Insert
- 粘贴流程:
  1. 将内容写入系统剪贴板
  2. 隐藏剪贴板窗口
  3. 延迟 100ms 后模拟粘贴快捷键
  4. 恢复到之前的焦点窗口

### 5.2 键盘导航

| 按键 | 功能 |
|------|------|
| `↑` / `↓` | 向上/向下选择 Item |
| `Enter` | 执行单击动作（默认复制） |
| `1-9` | 快速粘贴对应序号的 Item（需配合修饰键） |
| `Esc` | 关闭剪贴板窗口 |
| `/` 或 `Ctrl+F` | 聚焦搜索框 |
| `Ctrl+C` | 复制选中项 |
| `Ctrl+V` | 粘贴选中项 |
| `Delete` | 删除选中项（需确认） |

#### 5.2.1 数字键快捷粘贴

- 可配置修饰键: `ctrl` / `alt` / `shift` / `ctrl+shift` / `none`
- 默认: `Ctrl+1~9` 粘贴列表中对应位置的 Item
- 如果该位置无 Item，则不执行操作

### 5.3 上下文菜单

#### 5.3.1 菜单结构

```
┌─ Context Menu ──────────────────────┐
│  [图标] 复制                 Ctrl+C │
│  [图标] 粘贴                 Ctrl+V │
│  ────────────────────────────────── │
│  [图标] 添加到队列                  │
│  ────────────────────────────────── │
│       复制为纯文本                  │  (仅文本/HTML 显示)
│  ────────────────────────────────── │
│  [图标] 打开文件                    │  (图片/文件 显示)
│  [图标] 在文件夹中显示              │  (图片/文件/文件夹 显示)
│  ────────────────────────────────── │
│  [图标] 复制文件路径                │  (文件/文件夹/多文件 显示)
│  ────────────────────────────────── │
│  [图标] ☆ 收藏/取消收藏             │
│  [图标] 🏷️ 管理标签                  │
│  [图标] 删除                  Del   │
└─────────────────────────────────────┘
```

#### 5.3.2 菜单项可见性矩阵

| 菜单项 | text | html | rtf | image | file | folder | files |
|--------|------|------|-----|-------|------|--------|-------|
| 复制 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| 粘贴 | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ |
| 添加到队列 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| 复制为纯文本 | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| 打开文件 | ❌ | ❌ | ❌ | ✅ | ✅ | ❌ | ❌ |
| 在文件夹中显示 | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ | ❌ |
| 复制文件路径 | ❌ | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ |
| 管理标签 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| 删除 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

#### 5.3.3 右键菜单交互

- 右键 Item 时显示菜单
- 右键同时高亮选中该项
- 菜单显示位置: 鼠标位置
- 点击菜单外部或按 Esc 关闭菜单

---

## 六、标签系统

### 6.1 功能概述

标签系统用于替代传统的「收藏」功能，支持为一个剪贴板项关联多个标签。

### 6.2 数据结构

```typescript
interface Tag {
  id: string;        // 唯一标识 (UUID)
  name: string;      // 标签名称
  color: string;     // 标签颜色 (hex)
}
```

### 6.3 标签管理

#### 6.3.1 添加标签

- 通过右键菜单「管理标签」打开标签管理弹窗
- 弹窗显示所有已有标签和当前 Item 的标签
- 支持勾选/取消勾选已有标签
- 支持创建新标签

#### 6.3.2 标签管理弹窗

```
┌─ Tag Manager ─────────────────────────┐
│  管理标签                        [×]  │
│  ───────────────────────────────────  │
│  当前标签:                            │
│  [工作] [设计] [×]                    │
│  ───────────────────────────────────  │
│  所有标签:                            │
│  ☑ [工作]    ☐ [截图]                 │
│  ☑ [设计]    ☐ [代码]                 │
│  ☐ [链接]    ☐ [灵感]                 │
│  ───────────────────────────────────  │
│  创建新标签:                          │
│  [输入标签名称        ] [创建]        │
└───────────────────────────────────────┘
```

#### 6.3.3 标签显示

- Item 底部最多显示 3 个标签
- 超出显示 `+N` (如 `+2`)
- 标签颜色基于名称哈希从预设色彩中选取
- 标签可点击，点击后自动填入 `@标签名` 到搜索框

#### 6.3.4 预设标签颜色

```typescript
const TAG_COLORS = [
  { bg: '#fff2e8', text: '#fa8c16' },  // 橙色
  { bg: '#e6f7ff', text: '#1890ff' },  // 蓝色
  { bg: '#f6ffed', text: '#52c41a' },  // 绿色
  { bg: '#f9f0ff', text: '#722ed1' },  // 紫色
  { bg: '#fff0f6', text: '#eb2f96' },  // 粉色
  { bg: '#fff7e6', text: '#faad14' },  // 黄色
  { bg: '#f0f5ff', text: '#2f54eb' },  // 深蓝
  { bg: '#fcffe6', text: '#a0d911' },  // 青柠
  { bg: '#e6fffb', text: '#13c2c2' },  // 青色
  { bg: '#fff1f0', text: '#f5222d' },  // 红色
];
```

---

## 七、粘贴队列

### 7.1 功能概述

粘贴队列（购物车模式）允许用户批量选择多个剪贴板项，然后一键按顺序粘贴。

### 7.2 工作流程

1. 点击 Item 的「添加到队列」按钮或右键菜单
2. Item 被加入队列面板
3. 可在队列面板中调整顺序或删除
4. 点击「批量粘贴」按顺序粘贴所有内容
5. 粘贴完成后可选清空队列

### 7.3 队列面板

```
┌─ Paste Queue Panel ───────────────────┐
│  粘贴队列 (3)                    [×]  │
│  ───────────────────────────────────  │
│  ┌─ Queue Item 1 ─────────────────┐   │
│  │ [text] 内容预览...        [↑][↓][×]│
│  └─────────────────────────────────┘   │
│  ┌─ Queue Item 2 ─────────────────┐   │
│  │ [image] 图片预览...       [↑][↓][×]│
│  └─────────────────────────────────┘   │
│  ┌─ Queue Item 3 ─────────────────┐   │
│  │ [file] 文件名...          [↑][↓][×]│
│  └─────────────────────────────────┘   │
│  ───────────────────────────────────  │
│  分隔符: [换行 ▼]                     │
│  [清空队列]            [批量粘贴]     │
└───────────────────────────────────────┘
```

### 7.4 分隔符选项

| 选项 | 说明 |
|------|------|
| **换行** | 每个 Item 之间插入换行符 |
| **无** | 直接拼接，无分隔 |
| **自定义** | 用户输入自定义分隔符 |

### 7.5 批量粘贴流程

1. 验证队列不为空
2. 按顺序处理每个 Item:
   - 将内容写入系统剪贴板
   - 模拟粘贴快捷键
   - 如果设置了分隔符，先粘贴分隔符
   - 延迟 200ms 后处理下一个
3. 粘贴完成后可选清空队列

---

## 八、抽屉编辑器

### 8.1 功能概述

抽屉编辑器从右侧滑出，用于预览和编辑剪贴板项的详细内容。

### 8.2 触发方式

- 点击 Item 的「详情」按钮（Hover 显示）
- 右键菜单「编辑」（仅文本类型）

### 8.3 类型特定视图

#### 8.3.1 文本类型 (text/html/rtf)

```
┌─ Drawer Editor ─────────────────────────────┐
│  文本详情                              [×]  │
│  ─────────────────────────────────────────  │
│  ┌─ Editor ─────────────────────────────┐   │
│  │                                       │   │
│  │  [可编辑的文本区域]                   │   │
│  │                                       │   │
│  │  [支持多行文本输入]                   │   │
│  │                                       │   │
│  └────────────────────────────────────────┘   │
│  ─────────────────────────────────────────  │
│  字符数: 100 | 单词数: 20 | 行数: 5 | 大小: 1KB  │
│  ─────────────────────────────────────────  │
│  [复制] [粘贴] [保存为新项]                 │
└─────────────────────────────────────────────┘
```

**功能**:
- 可编辑的文本区域
- 实时统计: 字符数、单词数、行数、大小
- 操作按钮: 复制、粘贴、保存为新项

#### 8.3.2 图片类型

```
┌─ Drawer Editor ─────────────────────────────┐
│  图片预览                              [×]  │
│  ─────────────────────────────────────────  │
│  ┌─ Image Preview ──────────────────────┐   │
│  │                                       │   │
│  │     [大图预览，自适应宽度]            │   │
│  │                                       │   │
│  └────────────────────────────────────────┘   │
│  ─────────────────────────────────────────  │
│  尺寸: 1920x1080 | 格式: PNG | 大小: 2.5MB  │
│  ─────────────────────────────────────────  │
│  [复制] [粘贴]                              │
└─────────────────────────────────────────────┘
```

**功能**:
- 大图预览（自适应宽度）
- 图片信息: 尺寸、格式、文件大小
- 操作按钮: 复制、粘贴

#### 8.3.3 文件/文件夹类型

```
┌─ Drawer Editor ─────────────────────────────┐
│  文件详情                              [×]  │
│  ─────────────────────────────────────────  │
│  ┌─ File Info ───────────────────────────┐   │
│  │  [文件图标]                            │   │
│  │  文件名: document.pdf                  │   │
│  │  大小: 2.5MB                           │   │
│  │  路径: C:\Users\...\document.pdf       │   │
│  └────────────────────────────────────────┘   │
│  ─────────────────────────────────────────  │
│  [打开文件] [在文件夹中显示] [复制路径]     │
│  [复制] [粘贴]                              │
└─────────────────────────────────────────────┘
```

#### 8.3.4 多文件类型

```
┌─ Drawer Editor ─────────────────────────────┐
│  多文件详情 (5个文件)                  [×]  │
│  ─────────────────────────────────────────  │
│  ┌─ Files List ──────────────────────────┐   │
│  │  [图标] file1.jpg                     │   │
│  │  [图标] file2.png                     │   │
│  │  [图标] file3.txt                     │   │
│  │  [图标] file4.pdf                     │   │
│  │  [图标] file5.docx                    │   │
│  └────────────────────────────────────────┘   │
│  ─────────────────────────────────────────  │
│  [复制所有路径] [粘贴]                      │
└─────────────────────────────────────────────┘
```

### 8.4 复制文件路径格式

- 多文件路径使用换行符 (`\n`) 分隔
- 路径列表末尾不添加额外换行符
- 便于粘贴到编辑器或命令行

---

## 九、设置面板

### 9.1 窗口设计

- **尺寸**: 600x700，可调整大小
- **标题**: "Paste Library - 设置"
- **布局**: 左侧导航 (220px) + 右侧内容区

### 9.2 导航结构

| 导航项 | 内容分组 |
|--------|----------|
| **剪贴板** | 窗口设置、搜索设置、内容设置 |
| **历史记录** | 最大条数、自动清理 |
| **通用设置** | 开机自启 |
| **快捷键** | 唤醒快捷键、数字键修饰键 |
| **数据备份** | 导入导出、存储路径 |
| **关于** | 应用信息、打开剪贴板按钮 |

### 9.3 设置项详细规格

#### 9.3.1 剪贴板设置

**窗口设置**:

| 设置项 | 类型 | 默认值 | 选项 |
|--------|------|--------|------|
| `window_position` | 选择 | `remember` | `remember` / `center` / `cursor` |
| `smart_activate` | 开关 | `true` | true / false |
| `focus_search_on_activate` | 开关 | `false` | true / false |

- **窗口位置**:
  - `remember`: 记住上次关闭位置
  - `center`: 每次居中显示
  - `cursor`: 跟随鼠标光标位置

- **智能激活**: 如果激活时间与上次复制间隔 < 5 秒，自动:
  1. 清空搜索框
  2. 回到列表顶部
  3. 切换到「全部」标签页
  4. 聚焦搜索框（如果启用 `focus_search_on_activate`）

**搜索设置**:

| 设置项 | 类型 | 默认值 | 选项 |
|--------|------|--------|------|
| `search_position` | 选择 | `top` | `top` / `bottom` |

**内容设置**:

| 设置项 | 类型 | 默认值 | 选项 |
|--------|------|--------|------|
| `click_action` | 选择 | `copy` | `copy` / `paste` / `none` |
| `double_click_action` | 选择 | `paste` | `copy` / `paste` / `none` |
| `paste_shortcut` | 选择 | `ctrl_v` | `ctrl_v` / `shift_insert` |
| `hide_window_after_copy` | 开关 | `false` | true / false |
| `copy_as_plain_text` | 开关 | `false` | true / false |
| `confirm_delete` | 开关 | `true` | true / false |
| `auto_sort` | 开关 | `true` | true / false |

- **复制为纯文本**: HTML/RTF 复制时自动转换为纯文本
- **自动排序**: 重复内容置顶，内部复制保持位置

#### 9.3.2 历史记录设置

| 设置项 | 类型 | 默认值 | 范围 |
|--------|------|--------|------|
| `max_history_count` | 数字 | `1000` | 100 - 10000 |
| `auto_cleanup_days` | 选择 | `0` | `0` / `7` / `30` / `90` |

- `0` 表示永久保留
- 超出最大条数时自动删除最旧的记录

#### 9.3.3 通用设置

| 设置项 | 类型 | 默认值 |
|--------|------|--------|
| `auto_start` | 开关 | `false` |

#### 9.3.4 快捷键设置

| 设置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `hotkey` | 按键录制 | `Alt+V` | 唤醒剪贴板窗口快捷键 |
| `number_key_shortcut` | 选择 | `ctrl` | 数字键 1-9 修饰键 |

**按键录制**:
- 点击录制按钮后捕获键盘输入
- 支持修饰键: Ctrl, Alt, Shift, Win
- 支持字母键 A-Z, 数字键 0-9, F键 F1-F12
- 支持特殊键: Space, Enter, Escape, Tab, 方向键
- 修改后需重启应用生效

**数字键修饰键选项**:
- `none` - 直接按 1-9
- `ctrl` - Ctrl+1~9
- `alt` - Alt+1~9
- `shift` - Shift+1~9
- `ctrl+shift` - Ctrl+Shift+1~9

#### 9.3.5 数据备份

**存储路径显示**:
- 数据存储路径（可复制、可打开文件夹）
- 日志存储路径（仅显示）

**导入导出**:
- 导出数据: 将数据库导出为 JSON 文件
- 导入数据: 从 JSON 文件导入数据

### 9.4 设置存储

- 使用 Tauri 的存储 API 保存到本地文件
- 路径: `{app_data}/settings.json`
- 启动时自动加载
- 修改后实时生效（除快捷键需重启）

---

## 十、系统托盘

### 10.1 托盘图标

- 应用启动后常驻系统托盘
- 图标: 应用 Logo（浅色/深色适配）
- tooltip: "Paste Library"

### 10.2 托盘菜单

```
┌─ Tray Menu ────────────────┐
│  打开设置                  │
│  显示剪贴板        Alt+V   │
│  ───────────────────────── │
│  退出                      │
└────────────────────────────┘
```

### 10.3 托盘交互

- **左键双击**: 打开设置窗口
- **右键单击**: 显示托盘菜单
- **中键点击**（可选）: 显示/隐藏剪贴板窗口

### 10.4 关闭行为

- 点击剪贴板窗口关闭按钮 → 隐藏窗口（非退出）
- 点击设置窗口关闭按钮 → 隐藏窗口（非退出）
- 完全退出需通过托盘菜单「退出」或设置中的退出按钮

---

## 十一、窗口管理

### 11.1 剪贴板窗口

**窗口属性**:
- 无边框 (`decorations: false`)
- 不在任务栏显示 (`skip_taskbar: true`)
- 置顶显示 (`always_on_top: true`)
- 尺寸: 800x600（可调整）
- 最小尺寸: 400x300

**显示/隐藏**:
- 快捷键唤醒: `Alt+V`（可配置）
- 显示时:
  - 根据 `window_position` 设置决定位置
  - 应用智能激活逻辑
  - 聚焦搜索框（如果配置）
- 隐藏时:
  - 记住当前窗口位置（如果 `window_position: remember`）
  - 取消当前选中项

**自动隐藏**:
- 窗口失焦时自动隐藏
- 延迟 200ms 后执行（避免切换窗口时误触发）

### 11.2 设置窗口

**窗口属性**:
- 正常窗口 (`decorations: true`)
- 标题: "Paste Library - 设置"
- 尺寸: 600x700
- 居中显示

**显示**:
- 托盘菜单「打开设置」
- 首次运行自动显示
- 设置窗口唯一（重复打开聚焦现有窗口）

### 11.3 首次运行检测

- 检查设置文件是否存在
- 如果不存在，认为首次运行:
  - 显示设置窗口（而非剪贴板窗口）
  - 引导用户配置基本设置

---

## 十二、智能功能

### 12.1 内部/外部复制区分

**目的**: 区分应用内复制和系统剪贴板复制，避免重复记录

**实现**:
- 应用内复制时设置标记 `is_internal_copy: true`
- 监听器检查标记:
  - `true`: 内部复制，更新现有项的时间戳但不改变位置
  - `false`: 系统复制，新增记录或置顶现有项

### 12.2 智能激活

**触发条件**:
- 窗口显示时间 - 上次复制时间 < 5 秒

**执行操作**:
1. 清空搜索框
2. 滚动到列表顶部
3. 切换到「全部」标签页
4. 聚焦搜索框（如果 `focus_search_on_activate: true`）

### 12.3 搜索自动回顶

- 搜索文本变化时自动滚动到列表顶部
- 确保用户看到最新的搜索结果

### 12.4 图片加载重试

- 缩略图加载失败时自动重试
- 最多重试 5 次
- 显示加载状态占位符

---

## 十三、数据备份

### 13.1 导出数据

- 格式: JSON
- 包含: 所有剪贴板记录、设置、标签
- 路径: 用户选择保存位置

### 13.2 导入数据

- 支持导入 JSON 格式的备份文件
- 合并策略:
  - 已存在的记录: 保留较新的版本
  - 新记录: 直接添加
  - 标签: 合并标签列表

### 13.3 存储路径显示

| 路径 | 说明 | 操作 |
|------|------|------|
| 数据存储路径 | SQLite 数据库位置 | 复制路径、打开文件夹 |
| 缩略图路径 | 图片缩略图存储位置 | 打开文件夹 |
| 日志路径 | 应用日志文件位置 | 打开文件夹 |

---

## 十四、技术架构

### 14.1 技术栈

| 层级 | 技术 |
|------|------|
| **前端** | Vue 3 + TypeScript + Vite |
| **桌面框架** | Tauri v2 |
| **后端** | Rust |
| **数据库** | SQLite (rusqlite) |
| **状态管理** | Vue Composition API (ref/reactive) |
| **样式** | Scoped CSS + CSS Variables |

### 14.2 文件结构

```
src/
├── components/               # Vue 组件
│   ├── ClipboardItem.vue     # 剪贴板项卡片
│   ├── ClipboardList.vue     # 剪贴板列表（主界面）
│   ├── ContextMenu.vue       # 右键菜单
│   ├── DrawerEditor.vue      # 抽屉编辑器
│   ├── PasteQueuePanel.vue   # 粘贴队列面板
│   ├── SettingsPanel.vue     # 设置面板
│   ├── SmartSearch.vue       # 智能搜索组件
│   ├── TagManager.vue        # 标签管理弹窗
│   └── DragHandle.vue        # 窗口拖拽手柄
├── composables/              # 可复用逻辑
│   ├── useClipboard.ts       # 剪贴板监听逻辑
│   ├── usePasteQueue.ts      # 粘贴队列状态
│   ├── useSettings.ts        # 设置管理
│   ├── useSmartSearch.ts     # 智能搜索逻辑
│   └── useWindow.ts          # 窗口管理
├── types/                    # TypeScript 类型
│   └── index.ts
├── utils/                    # 工具函数
│   └── tagColors.ts          # 标签颜色生成
├── styles/                   # 全局样式
├── App.vue                   # 设置窗口入口
├── ClipboardView.vue         # 剪贴板窗口入口
└── main.ts                   # 应用入口

src-tauri/src/                # Rust 后端
├── lib.rs                    # 主入口 + Tauri 命令
├── clipboard.rs              # 剪贴板管理逻辑
├── models.rs                 # 数据模型
├── storage.rs                # SQLite 数据库操作
├── fuzzy_search.rs           # 模糊搜索
├── window_manager.rs         # 窗口管理
├── tray_manager.rs           # 系统托盘
├── shortcut_manager.rs       # 快捷键管理
└── prevent_default.rs        # 阻止默认行为
```

### 14.3 Tauri 命令列表

#### 剪贴板操作

| 命令 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `add_clipboard_item` | `text`, `html`, `is_internal_copy` | `ClipboardItem` | 添加文本/HTML |
| `add_clipboard_item_extended` | `content_type`, `content`, `file_paths`, `thumbnail_path`, `metadata`, `is_internal_copy` | `ClipboardItem` | 添加图片/文件 |
| `get_clipboard_history` | `limit`, `offset` | `GetHistoryResponse` | 获取历史记录 |
| `search_clipboard_history` | `query`, `limit` | `ClipboardItem[]` | 搜索历史 |
| `delete_clipboard_item` | `id` | `boolean` | 删除单条 |
| `clear_clipboard_history` | - | `boolean` | 清空历史 |
| `toggle_favorite` | `id` | `boolean` | 切换收藏 |
| `update_item_tags` | `id`, `tags` | `ClipboardItem` | 更新标签 |
| `create_tag` | `name` | `Tag` | 创建新标签 |

#### 设置操作

| 命令 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `get_settings` | - | `AppSettings` | 获取设置 |
| `save_settings` | `settings` | `boolean` | 保存设置 |
| `get_storage_paths` | - | `StoragePaths` | 获取存储路径 |

#### 文件操作

| 命令 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `open_file` | `path` | `boolean` | 打开文件 |
| `show_in_folder` | `path` | `boolean` | 在文件夹中显示 |
| `export_data` | `path` | `boolean` | 导出数据 |
| `import_data` | `path` | `boolean` | 导入数据 |

#### 窗口操作

| 命令 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `toggle_clipboard_window` | - | `boolean` | 切换剪贴板窗口 |
| `hide_clipboard_window` | - | `boolean` | 隐藏剪贴板窗口 |
| `show_clipboard_window` | - | `boolean` | 显示剪贴板窗口 |

---

## 十五、数据模型

### 15.1 TypeScript 类型定义

```typescript
/**
 * 剪贴板内容类型
 */
export type ClipboardContentType =
  | 'text'
  | 'html'
  | 'rtf'
  | 'image'
  | 'file'
  | 'folder'
  | 'files';

/**
 * 剪贴板元数据
 */
export interface ClipboardMetadata {
  // 图片相关
  width?: number;
  height?: number;
  format?: string;

  // 文件相关
  file_name?: string;
  file_size?: number;
  mime_type?: string;

  // 文件夹相关
  folder_name?: string;
  item_count?: number;
}

/**
 * 标签定义
 */
export interface Tag {
  id: string;
  name: string;
  color: string;
}

/**
 * 剪贴板历史记录项
 */
export interface ClipboardItem {
  id: number;
  content_type: ClipboardContentType;
  content: string;              // 文本内容或路径
  created_at: string;           // ISO 8601 格式
  content_hash: string;         // SHA256 哈希
  metadata?: ClipboardMetadata;
  file_paths?: string[];        // 文件路径列表
  thumbnail_path?: string;      // 缩略图路径
  tags?: string[];              // 标签 ID 列表
}

/**
 * 应用设置
 */
export interface AppSettings {
  // 历史记录设置
  max_history_count: number;
  auto_cleanup_days: number;

  // 窗口设置
  window_position: 'remember' | 'center' | 'cursor';
  window_pos_x?: number;
  window_pos_y?: number;

  // 智能激活
  smart_activate: boolean;
  focus_search_on_activate: boolean;

  // 音效设置
  copy_sound: boolean;

  // 搜索设置
  search_position: 'top' | 'bottom';

  // 内容设置
  click_action: 'copy' | 'paste' | 'none';
  double_click_action: 'copy' | 'paste' | 'none';
  paste_shortcut: 'ctrl_v' | 'shift_insert';
  hide_window_after_copy: boolean;
  image_ocr: boolean;
  copy_as_plain_text: boolean;
  paste_as_plain_text: boolean;
  confirm_delete: boolean;
  auto_sort: boolean;

  // 通用设置
  hotkey: string;
  auto_start: boolean;

  // 快捷键设置
  number_key_shortcut: string;
}

/**
 * 获取历史记录响应
 */
export interface GetHistoryResponse {
  items: ClipboardItem[];
  total: number;
}
```

### 15.2 数据库表结构

```sql
-- 剪贴板历史表
CREATE TABLE clipboard_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content_type TEXT NOT NULL,
    content TEXT NOT NULL,
    content_hash TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL,
    metadata TEXT,              -- JSON 格式
    file_paths TEXT,            -- JSON 数组
    thumbnail_path TEXT,
    tags TEXT                   -- JSON 数组，标签 ID 列表
);

-- 标签表
CREATE TABLE tags (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    color TEXT NOT NULL
);

-- 索引
CREATE INDEX idx_content_hash ON clipboard_history(content_hash);
CREATE INDEX idx_created_at ON clipboard_history(created_at);
CREATE INDEX idx_content_type ON clipboard_history(content_type);
```

---

## 十六、UI 设计规范

### 16.1 颜色规范

```css
/* 主色调 */
--primary: #262626;           /* 主按钮、激活状态 */
--primary-hover: #404040;     /* 主按钮悬停 */
--primary-light: #595959;     /* 次要按钮 */

/* 背景色 */
--bg-page: #f5f5f5;           /* 页面背景 */
--bg-card: #ffffff;           /* 卡片背景 */
--bg-sidebar: #ffffff;        /* 侧边栏背景 */
--bg-hover: #f5f5f5;          /* 悬停背景 */

/* 文字色 */
--text-primary: #262626;      /* 主要文字 */
--text-secondary: #595959;    /* 次要文字 */
--text-desc: #8c8c8c;         /* 描述文字 */
--text-disabled: #bfbfbf;     /* 禁用文字 */

/* 边框色 */
--border: #d9d9d9;            /* 输入框边框 */
--border-light: #e8e8e8;      /* 分割线 */
--border-hover: #262626;      /* 悬停边框 */

/* 状态色 */
--danger: #ff4d4f;            /* 危险操作 */
--danger-hover: #ff7875;      /* 危险操作悬停 */
--danger-bg: #fff2f0;         /* 危险按钮背景 */
--success: #52c41a;           /* 成功状态 */
--warning: #faad14;           /* 警告状态 */
--info: #1890ff;              /* 信息状态 */

/* 搜索高亮 */
--search-highlight: #fff566;  /* 关键词高亮背景 */
```

### 16.2 间距规范

| 名称 | 值 | 用途 |
|------|-----|------|
| xs | 4px | 图标间距 |
| sm | 8px | 组件内部间距 |
| md | 12px | 列表项内边距 |
| lg | 16px | 卡片间距 |
| xl | 24px | 分组间距 |
| xxl | 32px | 页面边距 |

### 16.3 字体规范

```css
font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 
             'Helvetica Neue', Arial, sans-serif;
```

| 用途 | 字号 | 字重 |
|------|------|------|
| 页面标题 | 16px | 600 |
| 分组标题 | 16px | 600 |
| 设置项标题 | 14px | 500 |
| 正文 | 14px | 400 |
| 描述文字 | 12px | 400 |
| 辅助文字 | 11px | 400 |

### 16.4 组件规范

#### 16.4.1 按钮

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

#### 16.4.2 开关

```css
.switch {
  width: 44px;
  height: 24px;
  border-radius: 24px;
  background: #d9d9d9;
  transition: 0.2s;
  cursor: pointer;
}

.switch.active {
  background: var(--primary);
}

.switch-thumb {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: white;
  transform: translateX(3px);
  transition: 0.2s;
}

.switch.active .switch-thumb {
  transform: translateX(23px);
}
```

#### 16.4.3 输入框

```css
.input {
  height: 32px;
  padding: 6px 12px;
  border-radius: 4px;
  border: 1px solid var(--border);
  font-size: 13px;
  outline: none;
}

.input:focus {
  border-color: var(--primary);
}

.input::placeholder {
  color: var(--text-disabled);
}
```

#### 16.4.4 下拉选择

```css
.select {
  padding: 6px 28px 6px 10px;
  border: 1px solid var(--border);
  border-radius: 4px;
  font-size: 13px;
  background: white;
  min-width: 120px;
  cursor: pointer;
}

.select:hover {
  border-color: var(--border-hover);
}
```

### 16.5 布局规范

#### 16.5.1 设置面板

```
┌─ Settings Panel ────────────────────────────────┐
│ ┌─ Sidebar ─┐ ┌─ Content Area ─────────────────┐│
│ │           │ │                                  ││
│ │  220px    │ │        flex: 1                   ││
│ │           │ │                                  ││
│ │ [导航项]  │ │  [分组标题]                      ││
│ │ [导航项]  │ │                                  ││
│ │ [导航项]  │ │  ┌─ Setting Item ──────────────┐ ││
│ │           │ │  │ 标题                 [控件] │ ││
│ │           │ │  │ 描述                         │ ││
│ │           │ │  └─────────────────────────────┘ ││
│ │           │ │                                  ││
│ └───────────┘ └──────────────────────────────────┘│
└───────────────────────────────────────────────────┘
```

**间距**:
- 侧边栏 padding: 12px
- 内容区 padding: 24px 32px
- 分组间距: 24px
- 设置项内边距: 16px 0

#### 16.5.2 剪贴板列表项

```
┌─ Clipboard Item ────────────────────────────┐
│ padding: 12px 16px                          │
│ ┌─ Type Tag ─┐ ┌─ Content ───────────────┐ │
│ │            │ │                          │ │
│ │   40px     │ │        flex: 1           │ │
│ │            │ │                          │ │
│ └────────────┘ └──────────────────────────┘ │
└─────────────────────────────────────────────┘
```

### 16.6 动画规范

| 动画 | 时长 | 缓动函数 |
|------|------|----------|
| 按钮悬停 | 200ms | ease |
| 开关切换 | 200ms | ease |
| 抽屉滑入 | 300ms | cubic-bezier(0.4, 0, 0.2, 1) |
| 菜单显示 | 150ms | ease-out |
| 列表项悬停 | 150ms | ease |

---

## 十七、开发建议

### 17.1 开发顺序建议

**Phase 1 - 核心功能 (MVP)**
1. 项目搭建 (Tauri + Vue + TypeScript)
2. SQLite 数据库和基础模型
3. 剪贴板监听（仅文本）
4. 基础列表 UI
5. 基础搜索
6. 设置存储

**Phase 2 - 功能完善**
1. 图片类型支持 + 缩略图
2. 文件/文件夹类型支持
3. 上下文菜单
4. 系统托盘
5. 全局快捷键

**Phase 3 - 高级功能**
1. 标签系统
2. 智能搜索语法
3. 粘贴队列
4. 抽屉编辑器
5. 数据备份

**Phase 4 - 优化**
1. 虚拟滚动（性能优化）
2. 多语言支持
3. 主题切换
4. 单元测试

### 17.2 注意事项

1. **类型安全**: 使用 TypeScript 严格模式，避免 `any` 类型
2. **错误处理**: 所有异步操作使用 try-catch，提供用户友好的错误提示
3. **性能**: 大数据量时使用虚拟滚动，图片懒加载
4. **跨平台**: 考虑 Windows/macOS/Linux 的差异（路径分隔符、快捷键等）
5. **状态管理**: 使用 Vue Composition API，避免过度使用全局状态
6. **样式隔离**: 使用 Scoped CSS，避免样式污染

### 17.3 测试清单

- [ ] 剪贴板监听（所有类型）
- [ ] 数据去重逻辑
- [ ] 搜索功能（模糊、拼音、语法）
- [ ] 标签系统（增删改查）
- [ ] 快捷键（录制、触发）
- [ ] 窗口管理（显示/隐藏/位置）
- [ ] 设置保存和恢复
- [ ] 数据导入导出
- [ ] 系统托盘功能
- [ ] 粘贴模拟（Ctrl+V/Shift+Insert）

---

## 十八、多设备局域网同步设计

> **适用范围**: 家庭/办公室局域网环境  
> **使用场景**: Windows、macOS、Linux 桌面端互相同步；移动端手动拉取  
> **状态**: Phase 2 功能（非 MVP 必须）

### 18.1 设计前提

**移动端限制说明**:
现代移动操作系统（iOS 14+ / Android 10+）禁止后台持续监控剪贴板：
- **iOS**: 后台无法访问剪贴板，前台访问会弹出系统提示
- **Android**: 仅当应用在前台或是默认输入法时才能读取剪贴板

**同步能力矩阵**:

| 场景 | 桌面端 | 移动端 |
|------|--------|--------|
| 桌面 → 桌面 | ✅ 自动实时同步 | - |
| 桌面 → 移动端 | ⚠️ 推送（需移动端前台） | ✅ 接收 |
| 移动端 → 桌面 | - | ❌ 需手动触发 |
| 移动端 → 移动端 | - | ❌ 不支持 |

**结论**: 桌面端享受完整双向同步，移动端作为"被动接收+手动发送"的补充。

### 18.2 架构设计

```
┌─ Windows ─┐      mDNS 自动发现      ┌─ macOS ─┐
│ Desktop   │◄─────────────────────►│ Desktop │
│ (全功能)   │                         │ (全功能) │
└─────┬─────┘                         └────┬────┘
      │                                    │
      │         TCP 长连接通信              │
      └──────────────┬─────────────────────┘
                     │
              ┌──────▼──────┐
              │   iOS App   │  ← 仅接收 + 手动发送
              │ (受限功能)   │
              └─────────────┘
                     │
              ┌──────▼──────┐
              │ Android App │  ← 仅接收 + 手动发送
              │ (受限功能)   │
              └─────────────┘
```

**核心技术选型**:
- **设备发现**: mDNS (Bonjour/Zeroconf)
- **通信协议**: TCP + 自定义轻量协议
- **安全方案**: 6 位配对码 + AES-256-GCM 对称加密
- **数据传输**: JSON 消息格式

### 18.3 设备发现 (mDNS)

**选用 mDNS 理由**:
- 跨平台支持好（Windows/macOS/Linux/iOS/Android）
- 成熟稳定（AirPlay、Spotify Connect 都在用）
- 无需配置 IP，自动发现同网段设备

**Rust 实现** (使用 `mdns-sd` crate):

```rust
use mdns_sd::{ServiceDaemon, ServiceInfo};

const SERVICE_TYPE: &str = "_paste-lib._tcp.local.";
const PORT: u16 = 8765;

/// 启动 mDNS 服务
pub fn start_mdns_service(device_id: &str, device_name: &str) {
    let mdns = ServiceDaemon::new().expect("Failed to create mDNS daemon");
    
    // 注册本机服务
    let service_info = ServiceInfo::new(
        SERVICE_TYPE,
        device_name,                      // 如 "My-Laptop"
        &format!("{}.local.", device_name),
        "",                               // 自动获取 IP
        PORT,
        &[
            ("device_id", device_id),
            ("platform", std::env::consts::OS),
            ("version", "1.0.0"),
        ][..],
    ).expect("valid service info");
    
    mdns.register(service_info).expect("Failed to register");
    
    // 浏览其他设备
    let receiver = mdns.browse(SERVICE_TYPE)
        .expect("Failed to browse");
    
    std::thread::spawn(move || {
        while let Ok(event) = receiver.recv() {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    println!("发现设备: {} @ {}:{}", 
                        info.get_fullname(),
                        info.get_address(),
                        info.get_port());
                    // 添加到可用设备列表
                }
                ServiceEvent::ServiceRemoved(_, fullname) => {
                    println!("设备离线: {}", fullname);
                    // 从设备列表移除
                }
                _ => {}
            }
        }
    });
}
```

### 18.4 通信协议

**协议设计原则**:
- 比 WebSocket 简单（无需 HTTP 握手）
- 比 HTTP 高效（长连接保持）
- 易于实现心跳保活

**消息格式**:

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SyncMessage {
    // 心跳保活
    Ping,
    Pong,
    
    // 身份验证
    Auth { device_id: String, pairing_code_hash: String },
    AuthSuccess,
    AuthFailed { reason: String },
    
    // 剪贴板数据
    ClipboardItem {
        uuid: String,                    // 全局唯一 ID
        content_type: String,            // text/html/image/file
        content_hash: String,            // SHA256 用于去重
        content: String,                 // 文本内容或元数据 JSON
        thumbnail_url: Option<String>,   // 大文件缩略图 URL
        created_at: String,              // ISO 8601
        device_id: String,               // 来源设备
    },
    
    // 批量同步
    SyncRequest { since: String },      // 请求某时间后的数据
    SyncResponse { items: Vec<ClipboardItem> },
    
    // 通知
    NewItem { item: ClipboardItem },    // 广播新剪贴板项
    ItemDeleted { uuid: String },       // 删除通知
}

// TCP 消息格式: [4字节长度][JSON 内容]
pub async fn send_message(stream: &mut TcpStream, msg: &SyncMessage) -> Result<()> {
    let json = serde_json::to_string(msg)?;
    let len = json.len() as u32;
    
    stream.write_all(&len.to_be_bytes()).await?;
    stream.write_all(json.as_bytes()).await?;
    Ok(())
}

pub async fn receive_message(stream: &mut TcpStream) -> Result<Option<SyncMessage>> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).await?;
    let len = u32::from_be_bytes(len_buf) as usize;
    
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).await?;
    
    Ok(serde_json::from_slice(&buf).ok())
}
```

### 18.5 安全方案

**方案选择**: 6 位配对码 + 对称加密

**理由**:
- 同一 WiFi 内的设备才能发现（第一层保护）
- 配对码确保只有授权设备能连接（第二层保护）
- AES-256 加密防止局域网嗅探（第三层保护）
- 无需证书管理，用户体验简单

**实现代码**:

```rust
use argon2::{Argon2, PasswordHasher};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use rand::RngCore;

/// 从配对码派生密钥
pub fn derive_key(pairing_code: &str) -> [u8; 32] {
    let salt = b"paste-lib-salt-v1"; // 固定 salt
    let argon2 = Argon2::default();
    let mut key = [0u8; 32];
    argon2.hash_password_into(
        pairing_code.as_bytes(),
        salt,
        &mut key,
    ).expect("Key derivation failed");
    key
}

/// 加密数据
pub fn encrypt(plaintext: &[u8], key: &[u8; 32]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext)
        .expect("Encryption failed");
    
    // 格式: [nonce (12 bytes)][ciphertext]
    [nonce.as_slice(), &ciphertext].concat()
}

/// 解密数据
pub fn decrypt(ciphertext: &[u8], key: &[u8; 32]) -> Option<Vec<u8>> {
    if ciphertext.len() < 12 {
        return None;
    }
    
    let (nonce, encrypted) = ciphertext.split_at(12);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    
    cipher.decrypt(Nonce::from_slice(nonce), encrypted).ok()
}
```

### 18.6 防循环机制

```rust
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// 记录最近处理的 UUID，防止 A→B→C→A 循环
static SEEN_UUIDS: Lazy<Mutex<LruCache<String, ()>>> = 
    Lazy::new(|| Mutex::new(LruCache::new(NonZeroUsize::new(1000).unwrap())));

/// 处理接收到的剪贴板项
pub async fn handle_incoming_item(item: ClipboardItem, from_device: &str) {
    let mut seen = SEEN_UUIDS.lock().unwrap();
    
    // 检查是否已处理过（防止循环）
    if seen.get(&item.uuid).is_some() {
        return;
    }
    
    // 记录已见
    seen.put(item.uuid.clone(), ());
    
    // 保存到本地数据库
    save_to_local_db(&item);
    
    // 转发给其他设备（除了来源）
    drop(seen); // 释放锁
    broadcast_to_other_peers(&item, from_device).await;
}

/// 广播给其他 peer
async fn broadcast_to_other_peers(item: &ClipboardItem, exclude_device: &str) {
    let peers = get_connected_peers().await;
    
    for (device_id, mut stream) in peers {
        if device_id != exclude_device {
            let msg = SyncMessage::NewItem { item: item.clone() };
            let _ = send_message(&mut stream, &msg).await;
        }
    }
}
```

### 18.7 移动端实现策略

#### iOS App

**功能限制**:
- 无法后台持续监听剪贴板
- 前台访问剪贴板会弹出系统提示
- 后台任务最多 30 秒处理时间

**建议实现**:

1. **手动同步模式** (主要)
```
打开 iOS App
    ↓
点击「同步」按钮
    ↓
发现局域网内桌面端
    ↓
拉取最新剪贴板历史
    ↓
显示列表，点击复制到手机剪贴板
```

2. **分享扩展** (辅助)
- iOS 系统分享菜单 → 选择「Paste Library」→ 发送到桌面端

3. **推送通知** (可选增强)
- 桌面端复制 → 本地推送 → 手机点击 → 自动复制到剪贴板

#### Android App

**功能限制**:
- Android 10+ 仅前台可读取剪贴板
- 后台服务可能被系统杀死

**建议实现**:

类似 iOS，主要采用手动同步模式：
- 打开 App → 发现桌面端 → 拉取历史
- Android 分享菜单发送到桌面端

### 18.8 数据模型扩展

为支持同步，数据库需要添加以下字段：

```sql
-- 剪贴板历史表扩展
ALTER TABLE clipboard_history ADD COLUMN uuid TEXT UNIQUE;
ALTER TABLE clipboard_history ADD COLUMN device_id TEXT;
ALTER TABLE clipboard_history ADD COLUMN sync_status TEXT DEFAULT 'local';
ALTER TABLE clipboard_history ADD COLUMN vector_clock TEXT; -- JSON 格式
ALTER TABLE clipboard_history ADD COLUMN updated_at TEXT;

-- 同步状态枚举
-- 'local': 仅本地，未同步
-- 'pending': 等待同步
-- 'synced': 已同步到所有设备
-- 'conflict': 冲突待解决

-- 设备表（记录已配对设备）
CREATE TABLE sync_devices (
    device_id TEXT PRIMARY KEY,
    device_name TEXT NOT NULL,
    platform TEXT, -- windows/macos/linux/ios/android
    last_seen TEXT, -- ISO 8601
    is_trusted BOOLEAN DEFAULT 0,
    pairing_code_hash TEXT
);

-- 创建索引
CREATE INDEX idx_sync_status ON clipboard_history(sync_status, updated_at);
CREATE INDEX idx_device_id ON clipboard_history(device_id);
```

**Rust 模型更新**:

```rust
#[derive(Debug, Clone)]
pub struct ClipboardItem {
    pub id: i64,                          // 本地自增 ID
    pub uuid: String,                     // UUIDv7（全局唯一）
    pub content_type: ClipboardContentType,
    pub content: String,
    pub content_hash: String,
    pub device_id: String,                // 创建设备
    pub vector_clock: HashMap<String, u64>, // 用于冲突检测
    pub sync_status: SyncStatus,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub enum SyncStatus {
    Local,      // 仅本地
    Pending,    // 等待同步
    Synced,     // 已同步
    Conflict,   // 冲突
}
```

### 18.9 实施路线图

| 阶段 | 内容 | 时间 | 优先级 |
|------|------|------|--------|
| **Phase 1** | mDNS 设备发现 | 2-3 天 | 🔴 高 |
| **Phase 2** | TCP 通信协议 + 配对验证 | 2-3 天 | 🔴 高 |
| **Phase 3** | 剪贴板广播 + 防循环 | 2-3 天 | 🔴 高 |
| **Phase 4** | 配对码加密 | 1-2 天 | 🟡 中 |
| **Phase 5** | iOS 基础 App（手动拉取） | 3-5 天 | 🟡 中 |
| **Phase 6** | Android 基础 App（手动拉取） | 2-3 天 | 🟡 中 |
| **Phase 7** | 分享扩展/推送通知（可选） | 2-3 天 | 🟢 低 |

**总计**: 约 2-3 周完成完整局域网同步

### 18.10 优缺点分析

**优势**:
- ✅ 无需云服务器，零运营成本
- ✅ 局域网内延迟极低（< 10ms）
- ✅ 数据不出本地网络，隐私性极高
- ✅ 实现复杂度低于云同步
- ✅ 适合家庭/办公室场景

**局限**:
- ❌ 设备必须在同一 WiFi（或互通 VLAN）
- ❌ 移动端只能手动同步（无法后台）
- ❌ 无法跨网络（如公司 VPN、不同地点）
- ❌ 设备发现偶尔不稳定（mDNS 问题）
- ❌ 多设备同时编辑可能产生冲突

### 18.11 UI 设计

#### 桌面端 - 设备管理面板

```
┌─ LAN Sync Panel ───────────────────────────┐
│  局域网同步                             [?] │
│  ─────────────────────────────────────────  │
│  本机信息:                                  │
│  名称: My-Laptop                      [编辑] │
│  ID: ABC123...                         [复制] │
│  ─────────────────────────────────────────  │
│  配对码: [123456]                    [刷新] │
│  ─────────────────────────────────────────  │
│  已连接设备 (2):                            │
│  ├─ 🖥️ MacBook-Pro    [在线] [断开]        │
│  └─ 📱 iPhone-12      [离线] [删除]        │
│  ─────────────────────────────────────────  │
│  同步设置:                                  │
│  ☑ 自动接收来自其他设备的剪贴板             │
│  ☑ 自动发送到其他设备                       │
│  ☐ 仅同步文本（忽略图片/文件）              │
│  最大同步项目: [100 ▼]                      │
└────────────────────────────────────────────┘
```

#### 移动端 - 同步界面

```
┌─ Mobile Sync ──────────────────────────────┐
│  同步剪贴板                            [⚙️] │
│  ─────────────────────────────────────────  │
│  [🔄 刷新]                                  │
│  ─────────────────────────────────────────  │
│  发现设备 (2):                              │
│  ├─ 🖥️ My-Laptop    192.168.1.5    [同步]  │
│  └─ 🖥️ MacBook-Pro  192.168.1.8    [同步]  │
│  ─────────────────────────────────────────  │
│  最近同步 (10):                             │
│  ├─ 这是剪贴板内容...     10:30   [复制]   │
│  ├─ [图片] screenshot.png  10:25   [复制]   │
│  └─ https://example.com   10:20   [复制]   │
└────────────────────────────────────────────┘
```

### 18.12 配置文件

```json
{
  "lan_sync": {
    "enabled": true,
    "device_name": "My-Laptop",
    "device_id": "uuid-v7-generated",
    "pairing_code": "123456",
    "port": 8765,
    "auto_accept": false,
    "sync_filters": {
      "text": true,
      "image": true,
      "file": false,
      "min_length": 1,
      "max_length": 100000
    },
    "peers": [
      {
        "device_id": "peer-uuid",
        "device_name": "MacBook-Pro",
        "trusted": true,
        "last_sync": "2026-02-19T10:30:00Z"
      }
    ]
  }
}
```

### 18.13 Cargo.toml 依赖

```toml
[dependencies]
# mDNS 发现
mdns-sd = "0.10"

# 异步网络
 tokio = { version = "1", features = ["net", "rt", "sync", "time"] }

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 加密
aes-gcm = "0.10"
argon2 = "0.5"
rand = "0.8"

# 工具
lru = "0.12"
once_cell = "1.19"
uuid = { version = "1.7", features = ["v7", "serde"] }
```

### 18.14 项目结构

```
src-tauri/src/
├── lan_sync/
│   ├── mod.rs              # 导出
│   ├── discovery.rs        # mDNS 设备发现
│   ├── protocol.rs         # TCP 通信协议
│   ├── crypto.rs           # 加密/解密
│   ├── sync_engine.rs      # 同步逻辑
│   ├── peer_manager.rs     # 设备管理
│   └── mobile_bridge.rs    # 移动端适配
├── lib.rs
└── main.rs
```

---

## 十九、功能优先级总结

### 必须实现 (MVP)
- 本地剪贴板管理（文本/图片/文件）
- 搜索与标签
- 设置面板
- 系统托盘

### 重要功能 (Phase 2)
- **局域网同步** ← 本章节内容
- 数据备份
- 粘贴队列
- 抽屉编辑器

### 可选功能 (Phase 3+)
- 移动端自动推送
- 虚拟滚动优化
- 多语言支持
- 主题切换

---

**文档结束**

> 此文档为 Paste Library 的完整功能规格说明书，重写开发时请参考此文档确保功能一致性。
> 
> **更新记录**:
> - 2026-02-19: 添加第十八章「多设备局域网同步设计」
