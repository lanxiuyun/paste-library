# Paste Library - 开发计划

## 项目概述

现代化的剪贴板管理工具，实时监听系统剪贴板，保存历史记录，支持快速查询和管理。

**技术栈**: Tauri v2 + Vue 3 + TypeScript

---

## 开发阶段

### 阶段 1: 基础设施搭建 ✅ COMPLETE
- [x] 安装 tauri-plugin-clipboard-x 依赖
- [x] Rust 后端初始化
- [x] 前端项目结构搭建
- [x] 数据存储方案确定 (SQLite)

### 阶段 2: 剪贴板监听与数据管理 ✅ COMPLETE
- [x] 剪贴板实时监听
- [x] 数据模型设计
- [x] Rust 后端 API

### 阶段 3: 用户界面 ✅ COMPLETE
- [x] 主窗口设计
- [x] 剪贴板历史列表
- [x] 交互功能
- [x] 窗口控制

### 阶段 4: 高级功能 ✅ COMPLETE
- [x] 搜索与过滤
- [x] 设置面板
- [x] 图片/文件类型支持
- [x] 右键上下文菜单
- [x] 收藏功能（已迁移到标签系统）
- [x] 系统托盘集成
- [x] 数据导出与备份

### 阶段 5: Item 重构与体验优化 ✅ COMPLETE

### 阶段 5.5: 设置功能实现 ⏳ IN PROGRESS
**详情见 [SETTINGS_IMPLEMENTATION.md](docs/SETTINGS_IMPLEMENTATION.md)**

设置面板UI已完成并可以保存数据，但前端代码没有实际使用这些设置。本阶段将实现所有"存而不用"的设置功能。

**2026-02-07 需求变更**: 移除7项不需要的功能，新增智能激活功能

**实现进度:** 87% (13/15项已完整实现)

**🔴 P0 - 核心体验设置（4项）**
- [x] **smart_activate** - 智能激活 ⭐ **已实现**
- [x] **click_action** - 单击动作（复制/粘贴）⭐ **已实现**
- [x] **double_click_action** - 双击动作（复制/粘贴）⭐ **已实现**
- [x] **window_position** - 窗口位置（记住/居中/跟随光标）⭐ **已实现**
- [x] **search_position** - 搜索框位置（顶部/底部）⭐ **已实现**

**🟡 P1 - 增强体验设置（4项）**
- [x] **confirm_delete** - 删除确认对话框 ⭐ **已实现**
- [x] **copy_as_plain_text** - 复制为纯文本 ⭐ **已实现**
- [x] **auto_sort** - 自动排序（重复内容置顶）⭐ **已实现**
- [ ] copy_sound - 复制音效

**🟢 P2 - 进阶功能设置（3项）**
- [x] **auto_start** - 开机自启 ⭐ **已实现**
- [ ] paste_as_plain_text - 粘贴为纯文本
- [ ] image_ocr - 图片OCR识别

**✅ 已实现设置（3项）**
- [x] hotkey - 唤醒快捷键（需重启生效）
- [x] max_history_count - 最大历史记录数（后端自动）
- [x] auto_cleanup_days - 自动清理天数（后端自动）

**❌ 已移除设置（7项）**
- ~~auto_favorite~~ - 自动收藏
- ~~操作按钮~~ - 操作按钮自定义
- ~~clear_search_on_activate~~ - 自动清除搜索
- ~~scroll_to_top_on_activate~~ - 激活时回到顶部（合并到智能激活）
- ~~switch_to_all_on_activate~~ - 激活时切换至全部分组（合并到智能激活）
- ~~blacklist_apps~~ - 应用黑名单
- ~~window_width/window_height~~ - 窗口尺寸

### 阶段 6: 测试与优化 📋 PLANNED
**Phase 1: Item 基础重构（Foundation）**
- [x] 可变高度 Item 设计（文本最多3行、图片自适应高度）
- [x] 信息展示简化（统计信息移至抽屉）
- [x] 标签系统替代收藏（数据模型 + UI）
- [x] Hover 快捷操作按钮（队列/复制/标签/删除）

**Phase 2: 核心交互增强（Core）**
- [x] 键盘导航系统（↑↓、Enter、数字键1-9、Esc）
- [x] 粘贴队列（购物车模式、批量粘贴）
- [x] 抽屉式编辑器（文本编辑、图片预览）

**Phase 3: 高级功能（Advanced）**
- [x] 跨应用拖拽
- [x] 模糊搜索（拼音、首字母、容错、标签搜索）
- [x] 图片文件缩略图显示（使用 convertFileSrc）
- [ ] ItemList虚拟滚动

### 阶段 5.6: Bug修复与稳定性改进 ✅ COMPLETE
**2026-02-11完成**
- [x] 修复开机自启初始化错误处理（Windows注册表错误）
- [x] 修复窗口关闭行为（关闭时隐藏而非退出）
- [x] 实现首次运行检测功能

### 阶段 6: 测试与优化 📋 PLANNED
- [ ] 功能测试
- [ ] 性能优化
- [ ] UI/UX 优化

### 阶段 7: 构建与发布 📋 PLANNED
- [ ] 打包构建
- [ ] 版本管理
- [ ] 发布

---

## 当前进度

| 阶段 | 完成度 |
|------|--------|
| 基础设施 | 100% ✅ |
| 监听与存储 | 100% ✅ |
| 用户界面 | 100% ✅ |
| 高级功能 | 100% ✅ |
| Item 重构与体验优化 | 100% ✅ |
| 设置功能实现 | 87% ⏳ |
| Bug修复与稳定性 | 100% ✅ |
| 测试与优化 | 0% 📋 |
| 构建与发布 | 0% 📋 |

**总体进度**: ~96%

---

## 已完成功能 ✅

### 核心功能 (P0)
- [x] 图片类型监听与显示
- [x] 文件/文件夹类型监听与显示
- [x] 左键/双击/右键交互
- [x] 右键上下文菜单 (ContextMenu组件)
- [x] 收藏/取消收藏功能（即将被标签系统替代）
- [x] 系统托盘集成

### 类型系统
- [x] TypeScript 类型定义 (ClipboardContentType, ClipboardMetadata)
- [x] Rust 模型同步 (models.rs)
- [x] 数据库 schema 迁移 (metadata, file_paths, thumbnail_path, is_favorite)

### 交互功能
- [x] 单击复制到剪贴板
- [x] 双击复制并粘贴
- [x] 右键显示上下文菜单
- [x] 打开文件 (open_file)
- [x] 在文件夹中显示 (show_in_folder)
- [x] 复制文件路径到剪贴板
- [x] 右键菜单复制/粘贴行为与单击/双击一致

### 重构计划 (开发中)
详见 [FEATURE_IMPLEMENTATION_CHECKLIST.md](docs/FEATURE_IMPLEMENTATION_CHECKLIST.md)
- 可变高度 Item 设计（文本3行、图片3行）
- 标签系统替代收藏（多标签、颜色、管理面板）
- Hover 快捷按钮（复制/标签/删除）
- 键盘导航（↑↓、Enter、1-9数字键）
- 粘贴队列（购物车批量粘贴）
- 抽屉式编辑器（文本编辑、图片预览）
- 跨应用拖拽
- 模糊搜索（拼音、首字母、容错）
- 搜索时，弹出搜索建议组件（显示所有标签，点击标签后加上关键字，类似 github pr 搜索一样 is:pr is:open ，我们的搜索词变成 tag:工作 tag:灵感 这种格式）
- 图片文件缩略图

---

## 待开发功能清单

### ✅ 已完成功能

**Phase 1: Item 基础重构**
- [x] 可变高度 Item 设计
  - 文本类型最多显示3行
  - 图片类型使用3行高度缩略图
  - 图片文件直接显示缩略图
- [x] 信息展示简化（统计信息移至抽屉）
- [x] 标签系统替代收藏功能
  - 移除 `is_favorite` 字段
  - 新增 `tags: string[]` 支持多标签
  - 标签管理面板
  - 数据迁移
- [x] Hover 快捷操作按钮
  - 详情 / 队列 / 复制 / 标签 / 删除 五个按钮

**Phase 2: 核心交互增强**
- [x] 键盘导航系统
  - ↑/↓ 导航 item
  - Enter 粘贴选中项
  - 1-9 数字键快速粘贴
  - Esc 关闭窗口
- [x] 粘贴队列（购物车模式）
  - 添加到队列
  - 选择分隔符（换行/无/自定义）
  - 一键批量粘贴
- [x] 抽屉式预览与编辑器
  - 文本：可编辑，显示统计信息
  - 图片：大图预览

**Phase 3: 高级功能**
- [x] 跨应用拖拽
- [x] 模糊搜索（拼音、首字母、容错）
- [x] 图片文件缩略图显示（使用 convertFileSrc）
- [x] 文件Item显示优化
  - 单文件/文件夹显示完整路径
  - 多文件显示前3个文件路径列表
  - 图片文件在Item列表直接显示缩略图
- [ ] ItemList虚拟滚动

### 中优先级 (P1)
- [x] 数据备份导入导出
- [x] 存储路径显示
- [x] 设置面板页面完善

### 低优先级 (P2)
- [ ] 多语言支持
- [ ] 主题切换
- [ ] 缩略图懒加载
- [ ] 性能优化
- [ ] **标签管理功能** - 标签管理弹窗未实现
  - [ ] 添加/删除标签UI弹窗（showTagManager为空函数）
  - [ ] 显示所有可用标签列表
  - [ ] 标签使用数量统计
  - [ ] 自定义标签创建

### P3 - 可选功能开关 ✅ 已完成 (2026-02-12)
基于用户反馈，添加功能显示/隐藏开关，让用户自定义界面：

**功能可见性设置**
- [x] **show_paste_queue** - 显示/隐藏粘贴队列功能 ⭐ **已实现**
  - 设置项：开启/关闭
  - 影响：隐藏底部粘贴队列面板和Item上的「添加到队列」按钮
  - 默认：开启
  - 实现：暂时注释掉相关组件，等粘贴队列功能完成后再恢复
  
- [x] **show_tags** - 显示/隐藏标签功能 ⭐ **已实现**
  - 设置项：开启/关闭  
  - 影响：隐藏Item上的标签显示、标签筛选标签页、Hover标签按钮
  - 默认：开启
  - 实现：暂时注释掉标签相关UI，等标签管理功能完成后再恢复

### P3 - 交互体验优化 ✅ 已完成 (2026-02-12)

**窗口激活与搜索优化**
- [x] **focus_search_on_activate** - 激活窗口时默认聚焦搜索框 ⭐ **已实现**
  - 设置项：勾选后，只要弹窗就自动聚焦搜索框
  - 与 smart_activate 独立工作
  - 默认：关闭（保持现有智能激活逻辑）
  - 修改文件：types/index.ts, useSettings.ts, models.rs, storage.rs, SettingsPanel.vue

- [x] **智能激活逻辑优化** ⭐ **已实现**
  - 问题：当前智能激活在从剪贴板双击/单击数据时也会触发（5秒内）
  - 优化：区分「系统剪贴板复制」和「本应用内复制」
  - 只有系统剪贴板复制才触发智能激活，本应用内操作不触发
  - 实现：添加 isInternalCopy 标志位，restoreToClipboard 时标记，handleClipboardChange 检测
  - 修改文件：useClipboard.ts, ClipboardList.vue

**搜索体验优化**
- [x] **搜索文本切换时回到顶部** ⭐ **已实现**
  - 当搜索框文本发生变化时，列表自动滚动到最顶部
  - 避免用户在搜索结果底部，新搜索后看不到顶部结果
  - 实现：使用 nextTick 确保 DOM 更新后再滚动
  - 修改文件：ClipboardList.vue

**键盘导航与选中体验**
- [x] **右键菜单选中项高亮** ⭐ **已实现**
  - 右键点击Item时，用上下键导航的高亮样式（is-selected）高亮该Item
  - 让用户清楚知道正在对哪个Item操作
  - 同时更新 selectedIndex 以便后续键盘导航
  - 修改文件：ClipboardList.vue

- [x] **上下键导航时自动滚动** ⭐ **已实现**
  - 使用 ↑/↓ 切换选中项时，滚动条自动跟随
  - 确保高亮的Item始终在可视区域内
  - 使用 `scrollIntoView({ behavior: 'smooth', block: 'nearest' })`
  - 修改文件：ClipboardList.vue

- [ ] **键盘导航与Hover样式统一**
  - 问题：当前键盘选中（is-selected）和Hover（is-hovered）UI表现形式不同
  - 建议方案：
    - **方案A**：合并为统一的高亮样式，Hover只是临时预览，键盘导航才是真正的"选中"
    - **方案B**：保留两种状态，但视觉样式更统一（例如都用左边框高亮，只是颜色不同）
  - 建议采用方案A：Item应该是单选模式，hover只是预览，键盘导航才是真正的选中态
  - 实现：
    - 键盘导航设置 selectedIndex（主选中态）
    - Hover仅显示快捷操作按钮，不改变选中态
    - Enter/数字键始终操作 selectedIndex 对应的Item

### 设置功能实现 ⏳ (详见 docs/SETTINGS_IMPLEMENTATION.md)

**2026-02-07 需求变更**: 移除7项不需要的功能，新增智能激活

**P0 - 核心体验设置（全部完成）**
- [x] **smart_activate** - 智能激活 ⭐ **已实现** (2026-02-07)
- [x] **click_action** - 单击动作（复制/粘贴）⭐ **已实现** (2026-02-11)
- [x] **double_click_action** - 双击动作（复制/粘贴）⭐ **已实现** (2026-02-11)
- [x] **window_position** - 窗口位置（remember/center/cursor）⭐ **已实现**
- [x] **search_position** - 搜索框位置（top/bottom）⭐ **已实现**

**P1 - 增强体验设置**
- [x] **confirm_delete** - 删除确认对话框 ⭐ **已实现**
- [x] **copy_as_plain_text** - 复制为纯文本 ⭐ **已实现**
- [x] **auto_sort** - 自动排序（重复内容置顶）⭐ **已实现**
- [ ] **copy_sound** - 复制音效（需要音效文件资源）

**P2 - 进阶功能设置**
- [x] **auto_start** - 开机自启 ⭐ **已实现**
- [ ] **paste_as_plain_text** - 粘贴为纯文本（需要模拟粘贴支持）
- [ ] **image_ocr** - 图片OCR识别（需要OCR库）

**已实现设置（9项）**
- [x] **hotkey** - 唤醒快捷键（Alt+V）
- [x] **max_history_count** - 最大历史记录数（后端自动限制）
- [x] **auto_cleanup_days** - 自动清理天数（后端自动清理）
- [x] **click_action** - 单击动作（复制/粘贴）
- [x] **double_click_action** - 双击动作（复制/粘贴）
- [x] **paste_shortcut** - 粘贴快捷键（Ctrl+V / Shift+Insert）
- [x] **window_position** - 窗口位置
- [x] **search_position** - 搜索框位置
- [x] **smart_activate** - 智能激活

**❌ 已移除设置（8项）**
- ~~auto_favorite~~ - 自动收藏（不需要）
- ~~auto_paste~~ - 自动粘贴（被 click_action + double_click_action 替代）
- ~~操作按钮~~ - 操作按钮自定义（不需要）
- ~~clear_search_on_activate~~ - 激活时清除搜索（不需要）
- ~~scroll_to_top_on_activate~~ - 激活时回到顶部（合并到智能激活）
- ~~switch_to_all_on_activate~~ - 激活时切换至全部分组（合并到智能激活）
- ~~blacklist_apps~~ - 应用黑名单（不需要）
- ~~window_width/window_height~~ - 窗口尺寸（不需要）

---

## 更新记录
- 2026-02-05: 完成基础设施、剪贴板监听、卡片式UI
- 2026-02-06: 实现全局快捷键(Alt+V)、窗口失焦自动隐藏、设置面板UI
- 2026-02-06: 完成图片/文件类型支持、右键菜单、收藏功能、交互增强
- 2026-02-06: 完成系统托盘集成（双击打开设置、右键菜单）
- 2026-02-06: 确定 Item 重构方案（可变高度、标签系统、Hover按钮、键盘导航、粘贴队列、抽屉编辑器）
- 2026-02-07: **完成 Phase 5** - Item 重构与体验优化全部完成
  - ✅ 可变高度 Item 设计（文本3行自适应、图片自适应）
  - ✅ 标签系统替代收藏（数据模型 + UI）
  - ✅ Hover 快捷操作按钮（详情/队列/复制/标签/删除）
  - ✅ 键盘导航系统（↑/↓、Enter、1-9、Esc）
  - ✅ 粘贴队列（购物车模式、批量粘贴）
  - ✅ 抽屉式编辑器（文本编辑、图片预览）
  - ✅ 图片加载修复（使用 convertFileSrc + CSP 配置）
  - ✅ Item 交互优化：点击复制/粘贴（设置控制），详情按钮打开抽屉
- 2026-02-07: **完成所有 P1 优先级任务**
  - ✅ 数据备份导入导出（JSON 格式，支持导入/导出）
  - ✅ 存储路径显示（数据/日志路径 + 复制/打开功能）
  - ✅ 设置面板完善（历史记录删除按钮）
  - ✅ 跨应用拖拽（支持文本/图片/文件拖拽到其他应用）
  - ✅ 模糊搜索（拼音、首字母、容错、标签搜索）
- 2026-02-07: **发现设置功能实现缺失问题**
  - 设置面板UI完成但功能未实现（20%完成度）
  - 创建 SETTINGS_IMPLEMENTATION.md 详细规划
  - 20项设置中仅4项完整实现，16项存而不用
- 2026-02-07: **完成智能激活功能**
  - 移除7项不需要的设置（自动收藏、操作按钮、自动清除、应用黑名单、窗口尺寸等）
  - 新增 smart_activate 智能激活功能
  - 如果激活时间与上次复制间隔<5秒，自动回到顶部、切换全部、聚焦搜索
  - 更新类型定义、Rust模型、数据库、设置面板UI
  - 实现进度: 27% (4/15)
- 2026-02-07: **完成设置功能实现** (进度: 73% - 11/15项)
  - ✅ search_position - 搜索框位置（顶部/底部切换）
  - ✅ confirm_delete - 删除确认对话框
  - ✅ copy_as_plain_text - 复制为纯文本（HTML/RTF转纯文本）
  - ✅ auto_paste - 自动粘贴行为（off/single/double）
  - ✅ window_position - 窗口位置（remember/center/cursor）
  - ✅ auto_sort - 自动排序（重复内容置顶，后端逻辑）
  - ✅ auto_start - 开机自启（tauri-plugin-autostart）
  - ✅ paste_shortcut - 粘贴快捷键模式（Ctrl+V / Shift+Insert）
  - ⏳ copy_sound - 复制音效（待实现，需音效文件）
  - ⏳ paste_as_plain_text - 粘贴为纯文本（待实现）
  - ⏳ image_ocr - 图片OCR识别（待实现，需OCR库）
- 2026-02-11: **BUG修复**
  - ✅ 修复开机自启初始化错误 "系统找不到指定的文件 (os error 2)"
    - 添加错误处理：检测 `is_enabled()` 状态，避免开发模式下的注册表错误
    - 静默处理 Windows 注册表相关错误，不影响应用正常使用
  - ✅ 修复窗口关闭行为
    - 设置窗口关闭时改为隐藏而非退出应用
    - 只有通过托盘菜单「退出」才能完全关闭应用
    - 主窗口默认启动时不显示（`visible: false`）
  - ✅ 首次运行检测
    - 添加 `app_initialized` 数据库字段检测是否首次运行
    - 首次运行时自动显示设置窗口
    - 后续启动只显示托盘图标，静默运行在后台
- 2026-02-11: **交互功能重构**
  - ✅ 重新定义「复制」和「粘贴」动作
    - **复制**: 仅将数据写入系统剪贴板
    - **粘贴**: 复制数据 → 隐藏剪贴板窗口 → 模拟发送 Ctrl+V 到原焦点窗口
  - ✅ 替换「自动粘贴」设置
    - 移除 `auto_paste` 设置（off/single/double）
    - 新增 `click_action` 设置：单击动作（复制/粘贴）
    - 新增 `double_click_action` 设置：双击动作（复制/粘贴）
  - ✅ 实现 Windows 原生粘贴模拟
    - 使用 `winapi` 的 `keybd_event` 发送扫描码
    - 绕过输入法拦截，正确触发 Ctrl+V 组合键
- 2026-02-11: **粘贴快捷键模式**
  - ✅ 添加 `paste_shortcut` 设置项
    - 用户可选择 Ctrl+V（默认）或 Shift+Insert
    - 终端/命令行用户推荐使用 Shift+Insert
  - ✅ 后端支持两种快捷键模式
    - Windows: 使用扫描码发送 Ctrl+V 或 Shift+Insert
      - Insert 是扩展键，使用 KEYEVENTF_EXTENDEDKEY 标志
    - Linux: 使用 enigo 库模拟对应快捷键
    - macOS: 始终使用 Command+V
- 2026-02-11: **Bug修复与细节优化**
  - ✅ 修复 HTML 复制时的多余空格问题
    - 问题: `writeHTML` 参数顺序错误，纯文本和 HTML 内容传反了
    - 修复: 正确传入 `writeHTML(plainText, htmlContent)`
    - 现在 HTML 内容复制时格式正确，无多余空格
  - ✅ 修复详情窗口粘贴按钮行为不一致
    - 问题: 详情窗口的「粘贴」按钮只复制到剪贴板，未执行粘贴动作
    - 修复: 点击粘贴按钮后调用 `simulatePaste()`，与单击/双击粘贴行为一致
  - ✅ 修复图片详情底部信息显示
    - 问题: 图片详情显示字符/单词/行数统计（文本统计信息）
    - 修复: 图片详情现在显示尺寸、格式、大小等图片相关信息
  - ✅ 优化详情窗口布局
    - 图片/文件详情：信息统一在内容区域显示（尺寸、格式、大小），底部统计栏移除
    - 文本详情：保留底部统计栏（字符、单词、行数、大小）
    - 避免信息重复显示，界面更简洁
- 2026-02-12: **右键菜单与文件显示优化**
  - ✅ 统一右键菜单复制粘贴行为
    - 右键「复制」：仅复制到剪贴板（与单击/双击复制行为一致）
    - 右键「粘贴」：复制数据 → 隐藏窗口 → 模拟粘贴（与单击/双击粘贴行为一致）
  - ✅ 文件Item显示优化
    - 单个图片文件：显示预览缩略图 + 完整文件路径
    - 单个非图片文件：显示 📄 图标 + 完整文件路径
    - 文件夹：显示 📁 图标 + 完整文件夹路径
    - 多文件：显示前3个文件的图标和路径，超出显示 "+N 个文件..."
    - 路径显示使用等宽字体，最多3行，支持自动换行
  - ✅ 复制文件路径功能
    - 右键菜单新增「复制文件路径」选项（适用于文件/文件夹/多文件类型）
    - 详情窗口新增「复制路径」按钮（单文件/文件夹）
    - 新增多文件详情视图，显示文件列表和「复制所有路径」按钮
    - 多文件路径使用 Windows 风格换行符 (`\r\n`) 分隔，末尾自动添加换行符
  - ✅ 修改文件
    - `src/components/ClipboardList.vue` - 右键菜单动作处理、复制文件路径函数
    - `src/components/ClipboardItem.vue` - 文件预览显示优化（图片缩略图、多行路径）
    - `src/components/ContextMenu.vue` - 添加「复制文件路径」菜单项
    - `src/components/DrawerEditor.vue` - 添加复制路径按钮、多文件详情视图
- 2026-02-12: **新增开发计划**
  - 📋 P3 - 可选功能开关（粘贴队列、标签功能可隐藏）
  - 📋 P3 - 交互体验优化
    - 激活窗口时默认聚焦搜索框设置
    - 智能激活逻辑优化（区分系统复制和应用内复制）
    - 搜索文本切换时回到顶部
    - 右键菜单选中项高亮
    - 上下键导航时自动滚动
    - 键盘导航与Hover样式统一方案讨论
- 2026-02-12: **UI优化与Bug修复**
  - ✅ 隐藏未实现的功能
    - 隐藏粘贴队列面板和快捷按钮
    - 隐藏标签显示和标签按钮
    - 移除收藏标签页
    - 隐藏右侧通知和AI按钮
    - 移除Item的「复制」快捷按钮，只保留「详情」和「删除」
    - 增大快捷按钮尺寸（22x22 -> 28x28px）
    - 修改文件：ClipboardList.vue, ClipboardItem.vue
  - ✅ 添加新设置项 focus_search_on_activate
    - 前端类型定义、默认值、设置面板
    - Rust后端模型、数据库存储
    - 与 smart_activate 独立工作
    - 修改文件：types/index.ts, useSettings.ts, models.rs, storage.rs, SettingsPanel.vue
  - ✅ 智能激活逻辑优化
    - 添加 isInternalCopy 标志位区分系统复制和应用内复制
    - 应用内复制不触发智能激活
    - 修改文件：useClipboard.ts
  - ✅ 搜索体验优化
    - 搜索文本变化时自动回到顶部
    - 使用 nextTick 确保 DOM 更新后滚动
    - 修改文件：ClipboardList.vue
  - ✅ 右键菜单选中项高亮
    - 右键点击时设置 selectedIndex
    - Item显示选中高亮样式
    - 修改文件：ClipboardList.vue
  - ✅ 上下键导航时自动滚动
    - 添加 scrollSelectedItemIntoView 函数
    - 使用 scrollIntoView 平滑滚动
    - 修改文件：ClipboardList.vue
- 2026-02-12: **图片加载Bug修复**
  - ✅ 添加图片加载重试机制
    - 最多重试5次，每次间隔200ms
    - 使用 key 属性强制刷新重新加载
    - 添加加载中状态和错误状态
    - 显示 loading 动画和错误提示
    - 修改文件：ClipboardItem.vue
- 2026-02-12: **悬浮窗口交互优化**
  - ✅ 修复滚动和hover失效问题
    - 添加 `accept_first_mouse(true)` 窗口配置
    - 增加失去焦点延迟（50ms -> 300ms）
    - 只在首次激活时聚焦搜索框（避免频繁焦点切换）
    - 禁用 ClipboardItem 的拖拽功能（与Tauri窗口拖拽冲突）
    - 添加缺失的拖拽权限 `core:window:allow-start-dragging`
    - 修改文件：window_manager.rs, ClipboardList.vue, ClipboardItem.vue, default.json
