# Paste Library - 技术设计方案

## 数据模型设计

### AppSettings 类型定义

```typescript
export interface AppSettings {
  // 历史记录设置
  max_history_count: number;
  auto_cleanup_days: number;

  // 窗口设置
  window_position: 'remember' | 'center' | 'cursor';
  window_width: number;
  window_height: number;
  scroll_to_top_on_activate: boolean;
  switch_to_all_on_activate: boolean;

  // 音效设置
  copy_sound: boolean;

  // 搜索设置
  search_position: 'top' | 'bottom';
  auto_focus_search: boolean;
  clear_search_on_activate: boolean;

  // 内容设置
  auto_paste: 'off' | 'single' | 'double';
  image_ocr: boolean;
  copy_as_plain_text: boolean;
  paste_as_plain_text: boolean;
  auto_favorite: boolean;
  confirm_delete: boolean;
  auto_sort: boolean;
  left_click_action: 'copy' | 'paste';  // 新增

  // 应用设置
  auto_start: boolean;
  silent_start: boolean;
  show_menu_bar_icon: boolean;
  show_taskbar_icon: boolean;

  // 外观设置
  language: 'zh-CN' | 'en-US';
  theme: 'system' | 'light' | 'dark';

  // 应用更新
  auto_check_update: boolean;
  beta_channel: boolean;

  // 快捷键设置
  hotkey_show_clipboard: string;
  hotkey_show_settings: string;
  quick_paste_enabled: boolean;
  quick_paste_modifier: string;
  hotkey_paste_as_plain: string;

  // 存储路径
  data_storage_path: string;
  log_storage_path: string;
}
```

### ClipboardItem 类型定义

```typescript
export type ClipboardContentType = 
  | 'text' 
  | 'html' 
  | 'rtf' 
  | 'image' 
  | 'file' 
  | 'folder' 
  | 'files';

export interface ClipboardItem {
  id: number;
  content_type: ClipboardContentType;
  content: string;
  content_hash: string;
  created_at: string;
  metadata?: ClipboardMetadata;
  file_paths?: string[];
  thumbnail_path?: string;
  size?: number;
}

export interface ClipboardMetadata {
  // 图片
  width?: number;
  height?: number;
  format?: string;
  
  // 文件
  file_name?: string;
  file_size?: number;
  mime_type?: string;
  
  // 文件夹
  folder_name?: string;
  item_count?: number;
}
```

---

## 图片处理方案

```rust
// src-tauri/src/image_handler.rs

pub struct ImageHandler;

impl ImageHandler {
    pub fn get_image_from_clipboard() -> Option<ImageData>;
    pub fn save_thumbnail(image_data: &[u8], item_id: i64) -> Result<String, Error>;
    pub fn load_thumbnail(path: &str) -> Result<Vec<u8>, Error>;
}
```

**缩略图规格**
- 尺寸: 200x200px
- 格式: PNG
- 存储路径: app_data/thumbnails/{item_id}.png
- 质量: 80%

---

## 文件处理方案

```rust
// src-tauri/src/file_handler.rs

pub struct FileHandler;

impl FileHandler {
    pub fn get_files_from_clipboard() -> Option<Vec<PathBuf>>;
    pub fn classify_path(path: &Path) -> ClipboardContentType;
    pub fn get_file_metadata(path: &Path) -> FileMetadata;
}
```

---

## 交互实现方案

### 左键/双击事件处理

```vue
<!-- 区分单击和双击 -->
<script setup>
const clickTimer = ref(null);
const clickCount = ref(0);

const handleClick = () => {
  clickCount.value++;
  
  if (clickCount.value === 1) {
    clickTimer.value = setTimeout(() => {
      // 单击逻辑
      handleSingleClick();
      clickCount.value = 0;
    }, 250);
  } else if (clickCount.value === 2) {
    clearTimeout(clickTimer.value);
    // 双击逻辑
    handleDoubleClick();
    clickCount.value = 0;
  }
};
</script>
```

### 上下文菜单实现

```vue
<ContextMenu
  v-model:visible="menuVisible"
  :position="menuPosition"
  :item="selectedItem"
  @action="handleMenuAction"
/>
```

---

## 数据流设计

```
剪贴板变化
    │
    ├─→ 文本/HTML → 直接存储到数据库
    │
    ├─→ 图片
    │     ├─→ 获取图片数据
    │     ├─→ 生成缩略图 → 保存到文件系统
    │     └─→ 存储缩略图路径到数据库
    │
    └─→ 文件/文件夹
          ├─→ 获取路径列表
          ├─→ 判断类型
          └─→ 存储路径+元数据到数据库
```

---

## Tauri 命令列表

### 剪贴板操作
- `add_clipboard_item()` - 添加记录
- `get_clipboard_history()` - 获取历史
- `search_clipboard_history()` - 搜索
- `delete_clipboard_item()` - 删除单条
- `clear_clipboard_history()` - 清空历史
- `copy_to_clipboard(item_id)` - 复制到剪贴板
- `simulate_paste()` - 模拟粘贴

### 设置操作
- `get_settings()` - 获取设置
- `save_settings()` - 保存设置
- `get_storage_paths()` - 获取存储路径

### 数据备份
- `export_data(path)` - 导出数据
- `import_data(path)` - 导入数据
- `delete_all_history()` - 删除所有历史

### 图片操作
- `get_image(item_id)` - 获取图片数据
- `get_thumbnail(path)` - 获取缩略图

### 文件操作
- `open_file(path)` - 打开文件
- `show_in_folder(path)` - 在文件夹中显示
