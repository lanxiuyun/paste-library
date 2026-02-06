# Paste Library - 实施计划

## 开发任务清单

### Phase 1: 类型定义更新
**文件**: `src/types/index.ts`, `src-tauri/src/models.rs`

- [ ] 更新 `AppSettings` 接口
  - [ ] 添加新字段到 TypeScript 类型
  - [ ] 更新 Rust Settings 结构体
  - [ ] 添加新的类型别名

### Phase 2: 图片支持
**文件**: `src-tauri/src/image_handler.rs`, `src/components/ImageItem.vue`

- [ ] Rust 后端
  - [ ] 集成 image crate
  - [ ] 实现剪贴板图片监听
  - [ ] 实现缩略图生成
  - [ ] 添加 get_image 命令
  
- [ ] 前端组件
  - [ ] 创建 ImageItem 组件
  - [ ] 实现缩略图加载
  - [ ] 添加图片预览功能

### Phase 3: 文件/文件夹支持
**文件**: `src-tauri/src/file_handler.rs`, `src/components/FileItem.vue`

- [ ] Rust 后端
  - [ ] 实现文件路径监听
  - [ ] 添加 get_file_icon 命令
  - [ ] 实现文件类型判断
  
- [ ] 前端组件
  - [ ] 创建 FileItem 组件
  - [ ] 创建 FolderItem 组件
  - [ ] 创建 FilesItem 组件

### Phase 4: 数据库迁移
**文件**: `src-tauri/src/storage.rs`

- [ ] 添加 metadata 字段
- [ ] 添加 file_paths 字段
- [ ] 添加 thumbnail_path 字段
- [ ] 实现数据迁移

### Phase 5: 交互增强
**文件**: `src/components/ClipboardItem.vue`, `src/components/ContextMenu.vue`

- [ ] 左键/双击事件处理
  - [ ] 区分单击和双击
  - [ ] 实现单击/双击逻辑
  
- [ ] 右键上下文菜单
  - [ ] 创建 ContextMenu 组件
  - [ ] 实现菜单项动态显示
  - [ ] 实现菜单功能

### Phase 6: 设置面板更新
**文件**: `src/components/SettingsPanel.vue`

- [ ] 历史记录页面
  - [ ] 改为带单位输入框
  - [ ] 添加删除按钮
  
- [ ] 通用设置页面
  - [ ] 添加外观设置分组
  - [ ] 添加应用更新分组
  
- [ ] 快捷键页面
  - [ ] 添加快捷键配置
  - [ ] 添加快速粘贴设置
  
- [ ] 数据备份页面
  - [ ] 添加存储路径显示
  - [ ] 重构导入导出

### Phase 7: Rust 后端命令
**文件**: `src-tauri/src/lib.rs`

- [ ] 添加 storage 相关命令
- [ ] 添加 backup 相关命令
- [ ] 添加 clipboard 操作命令

### Phase 8: 功能实现
**文件**: 各 composables

- [ ] 历史记录管理
- [ ] 存储路径功能
- [ ] 数据导入导出
- [ ] 左键行为配置

---

## 优先级排序

### 🔴 P0 - 核心功能
1. 图片类型支持
2. 文件/文件夹类型支持
3. 其他原生剪贴板类型支持
3. 左键/双击/右键交互
4. 上下文菜单

### 🟡 P1 - 增强体验
5. 设置面板完善
6. 数据备份功能
7. 存储路径显示

### 🟢 P2 - 优化完善
8. 多语言/主题
9. 性能优化
10. 系统托盘

---

## 依赖库

### Rust
- `image` - 图片处理
- `clipboard-win` / `arboard` - 跨平台剪贴板
- `directories` - 获取系统路径

### TypeScript/Vue
- 无新增主要依赖

---

## 测试清单

### 功能测试
- [ ] 各类型内容正确显示
- [ ] 左键/双击/右键交互正常
- [ ] 上下文菜单功能完整
- [ ] 设置正确持久化

### 性能测试
- [ ] 大量历史记录滚动流畅
- [ ] 图片缩略图加载快速
- [ ] 大文件处理不卡顿

### 兼容性测试
- [ ] Windows 正常
- [ ] macOS 正常
- [ ] Linux 正常
