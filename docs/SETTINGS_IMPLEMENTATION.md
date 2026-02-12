# Paste Library - 未实现设置功能清单

## 概述

设置面板UI已完成，大部分设置项可以保存到数据库，但前端代码没有实际使用这些设置。本文档列出所有"存而不用"的设置功能及其实现方案。

**2026-02-07 需求变更说明**:
- ❌ 移除功能：自动收藏、操作按钮、自动清除、应用黑名单、窗口尺寸
- ❌ 移除原激活时回到顶部/切换分组功能
- ✅ 新增功能：**智能激活** - 如果激活时间与上次复制时间间隔<5秒，自动回到顶部、切换至全部、聚焦搜索框

---

## 📋 功能清单

### 一、剪贴板设置 (Clipboard Settings)

#### 1. 窗口位置 (window_position)
- **当前状态**: ✅ **已实现**
- **设置类型**: select - remember/center/cursor
- **应实现功能**: 
  - remember: 记住上次窗口位置
  - center: 屏幕居中显示
  - cursor: 跟随鼠标位置显示
- **涉及文件**: 
  - `src-tauri/src/window_manager.rs` - 创建窗口时根据设置定位
  - 可能需要新增命令: `get_window_position`

#### 2. 智能激活 (smart_activate) ⭐ 新增
- **当前状态**: ✅ **已实现**
- **设置类型**: boolean (可选，默认开启)
- **实现功能**: 
  - 当剪贴板窗口被激活(显示)时，检查上次复制时间
  - 如果距离上次复制 < 5秒，则执行：
    1. 列表滚动到顶部
    2. 切换至"全部"标签
    3. 聚焦搜索框
  - 如果 > 5秒，保持当前状态
- **实现文件**: 
  - `src/composables/useClipboard.ts` - 记录 `lastCopyTime`
  - `src/components/ClipboardList.vue` - 监听 `tauri://focus` 事件并执行智能激活逻辑
- **实现日期**: 2026-02-07

#### 3. 复制音效 (copy_sound)
- **当前状态**: ⏳ 待实现（需要准备音效文件资源）
- **设置类型**: boolean + 音效文件路径
- **应实现功能**: 复制内容到剪贴板时播放音效
- **涉及文件**: 
  - `src/composables/useClipboard.ts` - restoreToClipboard 方法
  - 需要准备音效文件资源

#### 4. 搜索框位置 (search_position)
- **当前状态**: ✅ **已实现**
- **设置类型**: select - top/bottom
- **应实现功能**: 搜索框可以显示在列表顶部或底部
- **涉及文件**: `src/components/ClipboardList.vue` - 条件渲染搜索栏位置

#### 5. 默认聚焦 (auto_focus_search)
- **当前状态**: ✅ **已实现**（已合并到智能激活功能）
- **设置类型**: boolean
- **实现说明**: 智能激活功能已包含聚焦搜索框的逻辑，当5秒内复制过内容时自动聚焦
- **涉及文件**: `src/components/ClipboardList.vue`

#### 6. 自动粘贴 (auto_paste)
- **当前状态**: ✅ **已实现**
- **设置类型**: select - off/single/double
- **应实现功能**: 
  - off: 单击只复制
  - single: 单击复制并粘贴
  - double: 双击复制并粘贴（当前行为）
- **涉及文件**: 
  - `src/components/ClipboardList.vue` - handleItemClick/handleItemDoubleClick
  - 需要实现模拟粘贴功能（可能需要Rust后端支持）

#### 7. 图片OCR (image_ocr)
- **当前状态**: ❌ 未实现
- **设置类型**: boolean
- **应实现功能**: 复制图片时自动识别文字（需要集成OCR库）
- **涉及文件**: 
  - `src/composables/useClipboard.ts`
  - Rust后端需要新增OCR功能（如使用tesseract）
- **优先级**: 低（需要额外依赖）

#### 8. 复制为纯文本 (copy_as_plain_text)
- **当前状态**: ✅ **已实现**
- **设置类型**: boolean
- **应实现功能**: HTML/RTF内容复制时，只复制纯文本
- **涉及文件**: `src/composables/useClipboard.ts` - restoreToClipboard

#### 9. 粘贴为纯文本 (paste_as_plain_text)
- **当前状态**: ❌ 未实现
- **设置类型**: boolean
- **应实现功能**: 粘贴到其他应用时，只粘贴纯文本
- **涉及文件**: 模拟粘贴逻辑

#### 10. 删除确认 (confirm_delete)
- **当前状态**: ✅ **已实现**
- **设置类型**: boolean
- **应实现功能**: 删除剪贴板内容时显示确认对话框
- **涉及文件**: 
  - `src/components/ClipboardList.vue` - handleQuickAction/delete
  - `src/components/ClipboardItem.vue` - 删除按钮

#### 11. 自动排序 (auto_sort)
- **当前状态**: ✅ **已实现**
- **设置类型**: boolean
- **应实现功能**: 复制已存在的内容时，将其排列到列表最前面
- **涉及文件**: 
  - 后端 `src-tauri/src/storage.rs` - add_clipboard_item
  - 目前实现是更新时间，但列表顺序没有变化

---

### 二、历史记录设置 (History Settings)

#### 15. 最大历史记录数 (max_history_count)
- **当前状态**: ✅ 后端已实现，前端无感知
- **设置类型**: number (100-10000)
- **当前行为**: 后端自动限制存储数量
- **改进建议**: 前端设置页面显示当前存储数量/最大数量

#### 16. 自动清理 (auto_cleanup_days)
- **当前状态**: ✅ 后端已实现，前端无感知
- **设置类型**: number (0/7/30/90)
- **当前行为**: 后端自动清理过期数据
- **改进建议**: 显示上次清理时间或下次预计清理时间

---

### 三、通用设置 (General Settings)

#### 14. 开机自启 (auto_start)
- **当前状态**: ✅ **已实现**
- **设置类型**: boolean
- **应实现功能**: 系统启动时自动运行应用
- **涉及文件**: 
  - Rust后端需要实现开机自启逻辑
  - Tauri可能有相关API或需要系统级配置
- **优先级**: 中

---

### 四、快捷键设置 (Hotkey Settings)

#### 15. 唤醒快捷键 (hotkey)
- **当前状态**: ✅ 已实现
- **设置类型**: string (如 "Alt+V")
- **当前行为**: 只在应用启动时注册一次
- **改进建议**: 修改后需要提示用户重启应用生效

---

## ❌ 已移除的功能

以下功能已从需求中移除：

| 功能 | 移除原因 |
|------|----------|
| 自动收藏 (auto_favorite) | 不需要 |
| 操作按钮 | 不需要 |
| 自动清除 (clear_search_on_activate) | 不需要 |
| 激活时回到顶部 (scroll_to_top_on_activate) | 合并到智能激活 |
| 激活时切换至全部分组 (switch_to_all_on_activate) | 合并到智能激活 |
| 应用黑名单 (blacklist_apps) | 不需要 |
| 窗口尺寸 (window_width/window_height) | 不需要 |

---

## 📊 统计汇总

| 类别 | 总数 | 已实现 | 未实现 | 已移除 |
|------|------|--------|--------|--------|
| 剪贴板设置 | 11 | 7 | 3 | 4 |
| 历史记录设置 | 2 | 2 | 0 | 0 |
| 通用设置 | 1 | 1 | 0 | 1 |
| 快捷键设置 | 1 | 1 | 0 | 0 |
| **总计** | **15** | **11** | **3** | **7** |

**实现进度**: 73% (11/15)

---

## 🎯 优先级建议

### ✅ 已实现
1. **smart_activate** - 智能激活 (2026-02-07)
2. **auto_paste** - 自动粘贴行为（单击/双击）
3. **window_position** - 窗口位置（remember/center/cursor）
4. **search_position** - 搜索框位置（top/bottom）
5. **confirm_delete** - 删除确认对话框
6. **copy_as_plain_text** - 复制为纯文本
7. **auto_sort** - 自动排序（重复内容置顶）
8. **auto_start** - 开机自启

### ⏳ 待实现

### 🔴 P0 - 核心体验
- 全部完成 ✓

### 🟡 P1 - 增强体验
9. **copy_sound** - 复制音效（需要准备音效文件资源）

### 🟢 P2 - 进阶功能
10. **paste_as_plain_text** - 粘贴为纯文本（需要模拟粘贴时去除富文本格式）
11. **image_ocr** - 图片OCR（需要集成 OCR 库，如 tesseract）

---

## 📝 智能激活功能详细设计

### 需求
当剪贴板窗口被激活（显示）时：
1. 获取当前时间和上次复制时间的时间差
2. 如果 time_diff < 5秒：
   - 滚动列表到顶部
   - 切换 activeTab 到 'all'
   - 聚焦搜索框（如果 search_position 是 top/bottom）
3. 如果 time_diff >= 5秒：
   - 保持当前状态不变

### 实现方案

#### 方案1：前端记录（推荐）
```typescript
// useClipboard.ts
const lastCopyTime = ref<number>(Date.now());

const handleClipboardChange = async () => {
  // ... 现有逻辑
  lastCopyTime.value = Date.now();
  await loadHistory();
};

// ClipboardList.vue
const onWindowActivate = () => {
  const timeDiff = Date.now() - lastCopyTime.value;
  if (timeDiff < 5000) {
    // 5秒内
    scrollToTop();
    activeTab.value = 'all';
    searchInputRef.value?.focus();
  }
};
```

#### 方案2：后端提供时间戳
后端在返回 ClipboardItem 时包含 created_at，前端取第一项的时间。

### 涉及文件
- `src/composables/useClipboard.ts` - 记录 lastCopyTime
- `src/composables/useWindow.ts` - 监听窗口激活事件
- `src/components/ClipboardList.vue` - 执行激活逻辑

---

## 📝 其他实现注意事项

### 模拟粘贴
auto_paste 和 paste_as_plain_text 需要模拟键盘粘贴操作，可能需要：
1. Rust后端实现模拟按键（使用enigo等库）
2. 或使用clipboard库的paste功能
3. 需要处理不同平台的粘贴快捷键差异（Ctrl+V / Cmd+V）

### 音效文件
copy_sound需要准备音效文件并打包到应用中，配置Tauri的资源引用。

---

## 🔗 相关文件

- `src/components/SettingsPanel.vue` - 设置面板UI
- `src/composables/useSettings.ts` - 设置管理
- `src/composables/useClipboard.ts` - 剪贴板逻辑
- `src/composables/useWindow.ts` - 窗口管理
- `src/components/ClipboardList.vue` - 剪贴板列表
- `src-tauri/src/window_manager.rs` - 后端窗口管理
- `src-tauri/src/storage.rs` - 数据存储
- `src/types/index.ts` - TypeScript类型定义
- `src-tauri/src/models.rs` - Rust模型定义
