import type { ClipboardItem } from "@/types";
import type { Ref } from "vue";

export interface KeyboardNavigationOptions {
  filteredHistory: Ref<ClipboardItem[]>;
  selectedIndex: Ref<number>;
  searchQuery: Ref<string>;
  settings: Ref<{ pin_shortcut?: string; number_key_shortcut?: string }>;
  smartSearchRef?: Ref<{ focus: () => void } | null>;
  scrollerRef?: Ref<{ scrollToItem: (index: number, position: string) => void } | null>;
  executeClipboardAction: (item: ClipboardItem, action: "copy" | "paste") => Promise<void>;
  onEscape?: () => void | Promise<void>;
  onTogglePinMode?: () => Promise<void>;
  handleSmartSearch?: (query: string, shouldScrollToTop?: boolean) => Promise<void>;
}

export function useKeyboardNavigation(options: KeyboardNavigationOptions) {
  const {
    filteredHistory,
    selectedIndex,
    searchQuery,
    settings,
    smartSearchRef,
    scrollerRef,
    executeClipboardAction,
    onEscape,
    onTogglePinMode,
    handleSmartSearch,
  } = options;

  // 快捷键匹配辅助函数
  const matchesShortcut = (
    e: KeyboardEvent,
    shortcutConfig: string,
  ): boolean => {
    if (shortcutConfig === "none") {
      return !e.ctrlKey && !e.altKey && !e.shiftKey && !e.metaKey;
    }

    const requiredModifiers = shortcutConfig
      .toLowerCase()
      .split("+")
      .map((s) => s.trim());

    const actualModifiers: string[] = [];
    if (e.ctrlKey) actualModifiers.push("ctrl");
    if (e.altKey) actualModifiers.push("alt");
    if (e.shiftKey) actualModifiers.push("shift");
    if (e.metaKey) actualModifiers.push("meta");

    const allRequiredPressed = requiredModifiers.every((mod) =>
      actualModifiers.includes(mod),
    );
    const noExtraModifiers = actualModifiers.every((mod) =>
      requiredModifiers.includes(mod),
    );

    return allRequiredPressed && noExtraModifiers;
  };

  // 滚动选中项到可视区域
  const scrollSelectedItemIntoView = () => {
    if (selectedIndex.value < 0 || !scrollerRef?.value) return;
    scrollerRef.value.scrollToItem(selectedIndex.value, "center");
  };

  // 处理钉住模式快捷键
  const handlePinShortcut = async (e: KeyboardEvent): Promise<boolean> => {
    const pinShortcut = settings.value.pin_shortcut || "Ctrl+Shift+P";
    const shortcutParts = pinShortcut.toLowerCase().split("+");
    const hasCtrl = shortcutParts.includes("ctrl");
    const hasShift = shortcutParts.includes("shift");
    const hasAlt = shortcutParts.includes("alt");
    const hasMeta =
      shortcutParts.includes("meta") || shortcutParts.includes("cmd");
    const keyPart = shortcutParts.find(
      (p) => !["ctrl", "shift", "alt", "meta", "cmd"].includes(p),
    );

    if (
      hasCtrl === e.ctrlKey &&
      hasShift === e.shiftKey &&
      hasAlt === e.altKey &&
      hasMeta === e.metaKey &&
      e.key.toLowerCase() === (keyPart || "p")
    ) {
      e.preventDefault();
      await onTogglePinMode?.();
      return true;
    }
    return false;
  };

  // 主键盘处理器
  const handleKeyDown = async (e: KeyboardEvent): Promise<void> => {
    // Ctrl+F 聚焦搜索框
    if ((e.ctrlKey || e.metaKey) && e.key === "f") {
      e.preventDefault();
      smartSearchRef?.value?.focus();
      return;
    }

    // 钉住模式快捷键
    if (await handlePinShortcut(e)) return;

    // Esc 处理：先检查搜索框状态
    if (e.key === "Escape") {
      const query = searchQuery.value.trim();
      if (query) {
        // 搜索框有内容，先清空搜索框
        e.preventDefault();
        searchQuery.value = "";
        await handleSmartSearch?.("");
      } else {
        // 搜索框已为空，执行用户自定义的 Escape 逻辑
        e.preventDefault();
        await onEscape?.();
      }
      return;
    }

    // 上下导航
    if (e.key === "ArrowUp" || e.key === "ArrowDown") {
      e.preventDefault();
      const direction = e.key === "ArrowUp" ? -1 : 1;
      const newIndex = selectedIndex.value + direction;

      if (newIndex >= 0 && newIndex < filteredHistory.value.length) {
        selectedIndex.value = newIndex;
        scrollSelectedItemIntoView();
      }
      return;
    }

    // Enter 粘贴选中项
    if (e.key === "Enter" && selectedIndex.value >= 0) {
      e.preventDefault();
      const item = filteredHistory.value[selectedIndex.value];
      if (item) {
        await executeClipboardAction(item, "paste");
      }
      return;
    }

    // 数字键 1-9 快速粘贴
    if (e.key >= "1" && e.key <= "9") {
      const shortcut = settings.value.number_key_shortcut || "ctrl";

      if (matchesShortcut(e, shortcut)) {
        const index = parseInt(e.key) - 1;
        if (index < filteredHistory.value.length) {
          e.preventDefault();
          const item = filteredHistory.value[index];
          if (item) {
            await executeClipboardAction(item, "paste");
          }
        }
      }
      return;
    }
  };

  return {
    handleKeyDown,
    matchesShortcut,
    scrollSelectedItemIntoView,
  };
}
