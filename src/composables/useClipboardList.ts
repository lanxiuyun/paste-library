import type { ClipboardItem } from "@/types";
import { decodeHtmlEntities } from "@/utils/htmlUtils";
import { invoke } from "@tauri-apps/api/core";
import { writeText } from "tauri-plugin-clipboard-x-api";
import { ref, type Ref } from "vue";

export type ClipboardAction = "copy" | "paste";

export interface UIState {
  // Context Menu
  contextMenuVisible: Ref<boolean>;
  contextMenuPosition: Ref<{ x: number; y: number }>;
  contextMenuItem: Ref<ClipboardItem | null>;

  // Drawer
  drawerVisible: Ref<boolean>;
  drawerItem: Ref<ClipboardItem | null>;

  // Tag Manager
  tagManagerVisible: Ref<boolean>;
  tagManagerItem: Ref<ClipboardItem | null>;

  // Delete Confirm
  deleteConfirmVisible: Ref<boolean>;
  itemToDelete: Ref<ClipboardItem | null>;

  // Selection
  selectedIndex: Ref<number>;
}

export interface UseClipboardListOptions {
  settings: Ref<{
    copy_as_plain_text?: boolean;
    hide_window_after_copy?: boolean;
    paste_shortcut?: string;
    confirm_delete?: boolean;
    click_action?: string;
    double_click_action?: string;
  }>;
  isPinned: Ref<boolean>;
  history: Ref<ClipboardItem[]>;
  deleteItem: (id: number) => Promise<void>;
  loadHistory: () => Promise<void>;
  restoreToClipboard: (
    item: ClipboardItem,
    options?: { copyAsPlainText?: boolean },
  ) => Promise<void>;
  resetPanelState?: () => void;
}

export function useClipboardList(options: UseClipboardListOptions) {
  const {
    settings,
    isPinned,
    history,
    deleteItem,
    loadHistory,
    restoreToClipboard,
    resetPanelState,
  } = options;

  // UI 状态
  const contextMenuVisible = ref(false);
  const contextMenuPosition = ref({ x: 0, y: 0 });
  const contextMenuItem = ref<ClipboardItem | null>(null);

  const drawerVisible = ref(false);
  const drawerItem = ref<ClipboardItem | null>(null);

  const tagManagerVisible = ref(false);
  const tagManagerItem = ref<ClipboardItem | null>(null);

  const deleteConfirmVisible = ref(false);
  const itemToDelete = ref<ClipboardItem | null>(null);

  const selectedIndex = ref(-1);

  // UI 状态导出
  const uiState: UIState = {
    contextMenuVisible,
    contextMenuPosition,
    contextMenuItem,
    drawerVisible,
    drawerItem,
    tagManagerVisible,
    tagManagerItem,
    deleteConfirmVisible,
    itemToDelete,
    selectedIndex,
  };

  // 模拟粘贴
  const simulatePaste = async (): Promise<void> => {
    try {
      await invoke("simulate_paste", {
        pasteShortcut: settings.value.paste_shortcut,
      });
    } catch (error) {
      console.log("Paste simulation not available yet");
    }
  };

  // 统一的剪贴板操作入口
  const executeClipboardAction = async (
    item: ClipboardItem,
    action: ClipboardAction,
    opts: {
      copyAsPlainText?: boolean;
      hideWindow?: boolean;
    } = {},
  ) => {
    const {
      copyAsPlainText = settings.value.copy_as_plain_text,
      hideWindow = settings.value.hide_window_after_copy,
    } = opts;

    // 1. 恢复到剪贴板
    await restoreToClipboard(item, { copyAsPlainText });

    // 2. 默认模式下，复制/粘贴后按现有语义关闭窗口并重置状态
    if (!isPinned.value && (action === "paste" || hideWindow)) {
      await invoke("hide_clipboard_window");
      resetPanelState?.();
    }

    // 3. 粘贴必须最后执行，确保焦点已回到目标输入框
    if (action === "paste") {
      await simulatePaste();
    }
  };

  // 项目点击/双击通用处理
  const handleItemAction = async (
    item: ClipboardItem,
    index: number,
    actionType: "click" | "doubleClick",
  ) => {
    selectedIndex.value = index;

    const action =
      actionType === "click"
        ? settings.value.click_action
        : settings.value.double_click_action;

    if (action === "none") {
      return;
    }

    await executeClipboardAction(item, action as ClipboardAction);
  };

  const handleItemClick = (item: ClipboardItem, index: number) => {
    handleItemAction(item, index, "click");
  };

  const handleItemDoubleClick = (item: ClipboardItem, index: number) => {
    handleItemAction(item, index, "doubleClick");
  };

  // 右键菜单处理
  const handleItemContextMenu = (
    event: MouseEvent,
    item: ClipboardItem,
    index: number,
  ) => {
    contextMenuPosition.value = { x: event.clientX, y: event.clientY };
    contextMenuItem.value = item;
    contextMenuVisible.value = true;
    selectedIndex.value = index;
  };

  // 快捷操作
  const handleQuickAction = async (action: string, item: ClipboardItem) => {
    switch (action) {
      case "detail":
        drawerItem.value = item;
        drawerVisible.value = true;
        break;
      case "delete":
        await handleDelete(item);
        break;
      case "tag":
        tagManagerItem.value = item;
        tagManagerVisible.value = true;
        break;
    }
  };

  // 删除处理
  const handleDelete = async (item: ClipboardItem) => {
    if (settings.value.confirm_delete) {
      itemToDelete.value = item;
      deleteConfirmVisible.value = true;
    } else {
      await deleteItem(item.id);
    }
  };

  const confirmDelete = async () => {
    if (itemToDelete.value) {
      await deleteItem(itemToDelete.value.id);
      itemToDelete.value = null;
    }
    deleteConfirmVisible.value = false;
  };

  const cancelDelete = () => {
    itemToDelete.value = null;
    deleteConfirmVisible.value = false;
  };

  // Drawer 处理
  const handleDrawerCopy = async (item: ClipboardItem) => {
    await executeClipboardAction(item, "copy");
  };

  const handleDrawerPaste = async (item: ClipboardItem) => {
    await executeClipboardAction(item, "paste");
  };

  const handleSaveAsNew = async (content: string, type: string) => {
    try {
      if (type === "html") {
        await invoke("add_clipboard_item", {
          text: content.replace(/<[^>]*>/g, ""),
          html: content,
        });
      } else {
        await invoke("add_clipboard_item", { text: content, html: null });
      }
      await loadHistory();
    } catch (error) {
      console.error("Failed to save as new:", error);
    }
  };

  // 标签管理器处理
  const handleTagManagerSave = async (itemId: number, tags: string[]) => {
    const item = history.value.find((h) => h.id === itemId);
    if (item) {
      item.tags = tags.length > 0 ? tags : undefined;
    }
  };

  const handleRemoveTag = async (item: ClipboardItem, tag: string) => {
    try {
      const newTags = (item.tags || []).filter((t) => t !== tag);
      await invoke("update_tags", {
        id: item.id,
        tags: newTags.length > 0 ? newTags : null,
      });
      item.tags = newTags.length > 0 ? newTags : undefined;
    } catch (error) {
      console.error("Failed to remove tag:", error);
    }
  };

  // 复制文件路径
  const copyFilePath = async (item: ClipboardItem): Promise<void> => {
    try {
      let pathToCopy = "";

      if (item.file_paths && item.file_paths.length > 0) {
        pathToCopy = item.file_paths.join("\r\n") + "\r\n";
      } else if (item.content) {
        pathToCopy = item.content + "\r\n";
      }

      if (pathToCopy) {
        await writeText(pathToCopy);
      }
    } catch (error) {
      console.error("Failed to copy file path:", error);
    }
  };

  // 上下文菜单动作
  const handleContextMenuAction = async (action: string, item: ClipboardItem) => {
    switch (action) {
      case "copy":
        await executeClipboardAction(item, "copy");
        break;
      case "paste":
        await executeClipboardAction(item, "paste");
        break;
      case "tag":
        tagManagerItem.value = item;
        tagManagerVisible.value = true;
        break;
      case "copyPlain": {
        const plainContent =
          item.text_content ||
          decodeHtmlEntities(item.content.replace(/<[^>]*>/g, ""));
        const plainItem: ClipboardItem = {
          ...item,
          content_type: "text",
          content: plainContent,
        };
        await executeClipboardAction(plainItem, "copy", {
          copyAsPlainText: true,
        });
        break;
      }
      case "pastePlain": {
        const pastePlainContent =
          item.text_content ||
          decodeHtmlEntities(item.content.replace(/<[^>]*>/g, ""));
        const plainItem: ClipboardItem = {
          ...item,
          content_type: "text",
          content: pastePlainContent,
        };
        await executeClipboardAction(plainItem, "paste", {
          copyAsPlainText: true,
        });
        break;
      }
      case "delete":
        await handleDelete(item);
        break;
      case "openFile":
        if (item.file_paths && item.file_paths.length > 0) {
          await invoke("open_file", { path: item.file_paths[0] });
        } else if (item.thumbnail_path) {
          await invoke("open_file", { path: item.thumbnail_path });
        }
        break;
      case "showInFolder":
        if (item.file_paths && item.file_paths.length > 0) {
          await invoke("show_in_folder", { path: item.file_paths[0] });
        } else if (item.thumbnail_path) {
          await invoke("show_in_folder", { path: item.thumbnail_path });
        }
        break;
      case "copyFilePath":
        await copyFilePath(item);
        break;
    }
  };

  return {
    // UI State
    uiState,

    // Actions
    executeClipboardAction,
    simulatePaste,

    // Item handlers
    handleItemClick,
    handleItemDoubleClick,
    handleItemContextMenu,
    handleQuickAction,

    // Drawer
    handleDrawerCopy,
    handleDrawerPaste,
    handleSaveAsNew,

    // Delete
    handleDelete,
    confirmDelete,
    cancelDelete,

    // Tags
    handleTagManagerSave,
    handleRemoveTag,

    // Context menu
    handleContextMenuAction,
    copyFilePath,
  };
}
