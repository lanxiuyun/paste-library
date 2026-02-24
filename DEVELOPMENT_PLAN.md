# Paste Library - 开发计划

> **Version**: 0.2.0 | **Last Updated**: 2026-02-13

## 项目概述

Paste Library 是一款现代化的剪贴板管理工具，实时监听系统剪贴板，保存历史记录，支持快速查询和管理。

**技术栈**: Tauri v2 + Vue 3 + TypeScript + SQLite

---

## 开发阶段总览

| 阶段 | 状态 | 进度 |
|------|------|------|
| 1. 基础设施搭建 | ✅ 完成 | 100% |
| 2. 核心功能实现 | ✅ 完成 | 100% |
| 3. 体验优化 | ✅ 完成 | 100% |
| 4. 设置功能实现 | ✅ 完成 | 100% |
| 5. Bug修复与稳定性 | ✅ 完成 | 100% |
| 6. 测试与优化 | 📋 计划中 | 0% |
| 7. 构建与发布 | 📋 计划中 | 0% |

**总体进度**: ~98%

---

## 功能清单

### ✅ 已完成功能

#### 核心剪贴板功能 (P0)
- [x] 实时剪贴板监听（文本/HTML/RTF/图片/文件/文件夹/多文件）
- [x] SQLite 数据持久化存储
- [x] 基于 SHA256 的自动去重
- [x] 可变高度 Item 设计（文本3行、图片自适应）
- [x] 图片缩略图生成与显示
- [x] 文件/文件夹路径显示与预览

#### 交互与导航 (P0)
- [x] 单击/双击/右键交互（可配置动作）
- [x] 右键上下文菜单（复制/粘贴/打开/删除/标签）
- [x] 键盘导航（↑/↓、Enter、1-9数字键、Esc）
- [x] Hover 快捷操作按钮（详情/删除）
- [x] 跨应用拖拽支持
- [x] 自动滚动保持选中项可见

#### 搜索与过滤 (P1)
- [x] 模糊搜索（拼音、首字母、容错）
- [x] **智能搜索语法** - `@标签名`、`@类型`（@文本、@html、@图片、@文件）
- [x] **组合搜索** - 支持 `关键词 @标签` 组合方式
- [x] **搜索建议下拉** - 最近搜索历史、热门标签云
- [x] **关键词高亮** - 搜索结果中匹配的关键词高亮显示
- [x] 标签搜索支持
- [x] 标签页过滤（全部/文本/图片/文件）
- [x] 搜索框位置可配置（顶部/底部）
- [x] 清除按钮（X）快速清空搜索

#### 标签系统 (P1)
- [x] 多标签支持（替代收藏功能）
- [x] 标签管理弹窗（添加/删除/创建新标签）
- [x] 标签颜色自动生成
- [x] Item 底部标签显示

#### 粘贴队列 (P1)
- [x] 购物车模式批量粘贴
- [x] 分隔符自定义（换行/无/自定义）
- [x] 一键批量粘贴
- [x] 功能开关控制（可隐藏）

#### 抽屉编辑器 (P1)
- [x] 文本编辑与实时保存
- [x] 图片大图预览
- [x] 多文件详情视图
- [x] 统计信息显示

#### 设置面板 (P1)
- [x] 剪贴板设置（窗口位置、搜索位置、内容设置）
- [x] 历史记录设置（最大条数、自动清理）
- [x] 通用设置（开机自启）
- [x] 快捷键设置（唤醒快捷键、数字键修饰键）
- [x] 数据备份（导入/导出 JSON）
- [x] 存储路径显示

#### 系统托盘 (P1)
- [x] 托盘图标集成
- [x] 双击打开设置
- [x] 右键菜单（打开设置/显示剪贴板/退出）
- [x] 关闭到托盘（非退出）

#### 窗口管理 (P1)
- [x] 全局快捷键 Alt+V 唤醒
- [x] 窗口失焦自动隐藏
- [x] 窗口位置可配置（记住/居中/跟随光标）
- [x] 首次运行检测

#### 高级功能 (P1)
- [x] 智能激活（5秒内复制则清空搜索/回到顶部/聚焦搜索）
- [x] 内部/外部复制区分（内部复制保持位置）
- [x] 复制为纯文本（HTML/RTF 转纯文本）
- [x] 粘贴快捷键可配置（Ctrl+V / Shift+Insert）
- [x] 复制后隐藏窗口
- [x] 删除确认对话框

#### 文件操作 (P1)
- [x] 打开文件/文件夹
- [x] 在文件夹中显示
- [x] 复制文件路径
- [x] 多文件路径批量复制

### 📋 待开发功能

#### P2 - 进阶功能
- [ ] 图片 OCR 识别（需 OCR 库）
- [ ] ItemList 虚拟滚动（性能优化）

#### P3 - 可选增强
- [ ] 多语言支持（i18n）
- [ ] 高级搜索过滤器（日期范围）
- [ ] 数据同步（跨设备）

#### P4 - 发布准备
- [ ] 功能测试覆盖
- [ ] 性能优化
- [ ] UI/UX 细节优化
- [ ] 打包构建流程
- [ ] 自动更新测试
- [ ] 版本管理与发布

---

## 设置功能详细状态

### 已实现设置（19项）

| 设置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `hotkey` | string | "Alt+V" | 唤醒剪贴板窗口快捷键 |
| `max_history_count` | number | 1000 | 最大历史记录数 |
| `auto_cleanup_days` | number | 0 | 自动清理天数（0=永久） |
| `window_position` | string | "remember" | 窗口位置（remember/center/cursor） |
| `smart_activate` | boolean | true | 智能激活（5秒内复制则优化体验） |
| `focus_search_on_activate` | boolean | false | 激活窗口时自动聚焦搜索框 |
| `search_position` | string | "top" | 搜索框位置（top/bottom） |
| `click_action` | string | "copy" | 单击动作（copy/paste/none） |
| `double_click_action` | string | "paste" | 双击动作（copy/paste/none） |
| `paste_shortcut` | string | "ctrl_v" | 粘贴快捷键（ctrl_v/shift_insert） |
| `hide_window_after_copy` | boolean | false | 复制后隐藏窗口 |
| `copy_as_plain_text` | boolean | false | 复制为纯文本 |
| `confirm_delete` | boolean | true | 删除确认对话框 |
| `auto_sort` | boolean | true | 自动排序（重复内容置顶） |
| `auto_start` | boolean | false | 开机自启 |
| `number_key_shortcut` | string | "ctrl" | 数字键1-9快速粘贴修饰键 |
| `show_paste_queue` | boolean | true | 显示粘贴队列功能 |
| `show_tags` | boolean | true | 显示标签功能 |

### 待实现设置（3项）

| 设置项 | 状态 | 说明 |
|--------|------|------|
| `image_ocr` | ⏳ 待实现 | 图片 OCR 识别（需 OCR 库） |
| `theme` | ⏳ 待实现 | 主题切换（light/dark/system） |

---

## 文件结构

```
src/
├── components/           # Vue 组件
│   ├── ClipboardItem.vue    # 剪贴板项卡片
│   ├── ClipboardList.vue    # 剪贴板列表（主界面）
│   ├── ContextMenu.vue      # 右键菜单
│   ├── DrawerEditor.vue     # 抽屉编辑器
│   ├── PasteQueuePanel.vue  # 粘贴队列面板
│   ├── SettingsPanel.vue    # 设置面板
│   ├── SmartSearch.vue      # 智能搜索组件（含建议下拉）
│   ├── TagManager.vue       # 标签管理弹窗
│   └── DragHandle.vue       # 窗口拖拽手柄
├── composables/          # 可复用逻辑
│   ├── useClipboard.ts      # 剪贴板监听
│   ├── usePasteQueue.ts     # 粘贴队列状态
│   ├── useSettings.ts       # 设置管理
│   ├── useSmartSearch.ts    # 智能搜索逻辑
│   └── useWindow.ts         # 窗口管理
├── types/               # TypeScript 类型
│   └── index.ts
├── utils/               # 工具函数
│   └── tagColors.ts         # 标签颜色生成
├── App.vue              # 设置窗口入口
├── ClipboardView.vue    # 剪贴板窗口入口
└── main.ts              # 应用入口

src-tauri/src/           # Rust 后端
├── lib.rs               # 主入口 + Tauri 命令
├── clipboard.rs         # 剪贴板管理逻辑
├── models.rs            # 数据模型
├── storage.rs           # SQLite 数据库操作
├── fuzzy_search.rs      # 模糊搜索
├── window_manager.rs    # 窗口管理
├── tray_manager.rs      # 系统托盘
├── shortcut_manager.rs  # 快捷键管理
└── prevent_default.rs   # 阻止默认行为
```

---

## 更新记录

### 2026-02-13 - 智能搜索功能完整实现
- 新增 `useSmartSearch.ts` composable - 智能搜索语法解析器
  - 支持 `@标签名` 按标签搜索
  - 支持 `@类型` 按类型搜索（@文本、@html、@图片、@文件、@文件夹）
  - 支持组合搜索：`关键词 @标签`、`关键词 @类型`
  - 本地搜索历史管理（localStorage）
- 新增 `SmartSearch.vue` 组件
  - 搜索框带清除按钮（X）
  - 搜索建议下拉面板
  - 热门标签云显示
  - 最近搜索历史（可删除单条/清空全部）
  - 搜索语法提示
  - 键盘导航支持（↑/↓/Enter/Esc）
- 更新 `ClipboardList.vue`
  - 集成智能搜索功能
  - 前端本地过滤（结合Tab标签页和智能搜索）
- 更新 `ClipboardItem.vue`
  - 关键词高亮显示（黄色背景标记）
  - 标签可点击，点击后自动填入 `@标签名` 到搜索框

### 2026-02-13 - 标签功能完整实现
- 创建 TagManager 标签管理弹窗组件
- Item 底部标签显示（最多3个，超出显示+N）
- 标签颜色系统（基于哈希的预设色彩）
- 右键菜单标签按钮修复

### 2026-02-13 - 剪贴板功能优化
- 文本标签页显示所有文本类型（text/html/rtf）
- 区分内部/外部复制逻辑
- 智能激活时清空搜索框
- 单击/双击添加"不操作"选项
- 添加复制后隐藏窗口设置

### 2026-02-12 - UI优化与Bug修复
- 隐藏未实现的功能（粘贴队列、标签按钮）
- 添加 `focus_search_on_activate` 设置
- 智能激活逻辑优化（区分系统复制和应用内复制）
- 搜索文本切换时回到顶部
- 右键菜单选中项高亮
- 上下键导航时自动滚动
- 图片加载重试机制（5次重试）

### 2026-02-12 - 右键菜单与文件显示优化
- 统一右键菜单复制粘贴行为
- 文件 Item 显示优化（缩略图/路径）
- 复制文件路径功能

### 2026-02-11 - 交互功能重构
- 重新定义「复制」和「粘贴」动作
- 替换「自动粘贴」设置为 click_action + double_click_action
- 实现 Windows 原生粘贴模拟（扫描码）
- 添加粘贴快捷键模式（Ctrl+V / Shift+Insert）

### 2026-02-11 - Bug修复
- 修复开机自启初始化错误处理
- 修复窗口关闭行为（关闭到托盘）
- 实现首次运行检测

### 2026-02-07 - 设置功能实现
- 完成智能激活功能
- 实现窗口位置、搜索位置、内容设置等
- 移除7项不需要的设置

### 2026-02-07 - Phase 5 完成
- 可变高度 Item 设计
- 标签系统替代收藏
- Hover 快捷操作按钮
- 键盘导航系统
- 粘贴队列
- 抽屉式编辑器
- 模糊搜索
- 跨应用拖拽

---

## 下一步计划

1. **测试覆盖** - 添加单元测试和集成测试
2. **性能优化** - 实现虚拟滚动优化大列表性能
3. **国际化** - 添加多语言支持框架
4. **主题系统** - 实现暗色模式
5. **发布准备** - 完善构建流程和自动更新

---

## 参考文档

- [功能规格说明](docs/FEATURE_SPEC.md) - 详细功能定义
- [技术设计方案](docs/TECH_DESIGN.md) - 类型定义和架构
- [UI设计规范](docs/UI_DESIGN.md) - 颜色和组件规范
- [AGENTS.md](AGENTS.md) - 开发规范和指南
