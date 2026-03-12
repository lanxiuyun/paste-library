<template>
  <div class="settings-view">
    <!-- 左侧导航栏 -->
    <div class="sidebar">
      <nav class="nav-menu">
        <button 
          v-for="item in menuItems" 
          :key="item.key"
          class="nav-item"
          :class="{ active: activeMenu === item.key }"
          @click="activeMenu = item.key"
        >
          <span class="nav-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path v-if="item.key === 'clipboard'" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"/>
              <path v-else-if="item.key === 'history'" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
              <path v-else-if="item.key === 'general'" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
              <path v-else-if="item.key === 'hotkey'" d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
              <path v-else-if="item.key === 'backup'" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4"/>
              <path v-else-if="item.key === 'about'" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
            </svg>
          </span>
          <span class="nav-label">{{ item.label }}</span>
        </button>
      </nav>
    </div>

    <!-- 右侧内容区 -->
    <div class="content">
      <ClipboardSection
        v-if="activeMenu === 'clipboard'"
        :form="form"
      />

      <HistorySection
        v-else-if="activeMenu === 'history'"
        :form="form"
        @clear-all="handleClearAllHistory"
      />

      <GeneralSection
        v-else-if="activeMenu === 'general'"
        :form="form"
      />

      <HotkeySection
        v-else-if="activeMenu === 'hotkey'"
        :form="form"
        :shortcut-error="shortcutError"
        @hotkey-record="validateHotkey"
      />

      <BackupSection
        v-else-if="activeMenu === 'backup'"
        :storage-paths="storagePaths"
        @export="handleExport"
        @import="handleImport"
        @path-copy="handlePathCopy"
        @path-open="handlePathOpen"
      />

      <AboutSection
        v-else-if="activeMenu === 'about'"
        :current-version="currentVersion"
        :update-status="updateStatus"
        :latest-version="latestVersion"
        :update-notes="updateNotes"
        :update-date="updateDate"
        :is-downloading="isDownloading"
        :download-progress="downloadProgress"
        @check="checkForUpdates"
        @download="downloadUpdate"
        @skip="skipUpdate"
        @open-link="openExternalLink"
      />

      <!-- 底部操作栏 -->
      <div v-if="activeMenu !== 'about'" class="settings-footer">
        <button class="btn-secondary" @click="resetSettings">恢复默认设置</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, reactive } from 'vue';
import { useSettings } from '@/composables/useSettings';
import { useClipboard } from '@/composables/useClipboard';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { check, type Update } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import type { AppSettings } from '@/types';

import ClipboardSection from './sections/ClipboardSection.vue';
import HistorySection from './sections/HistorySection.vue';
import GeneralSection from './sections/GeneralSection.vue';
import HotkeySection from './sections/HotkeySection.vue';
import BackupSection from './sections/BackupSection.vue';
import AboutSection from './sections/AboutSection.vue';

const { settings, loadSettings, saveSettings } = useSettings();
const { loadHistory } = useClipboard();

const activeMenu = ref('clipboard');

const menuItems = [
  { key: 'clipboard', label: '剪贴板' },
  { key: 'history', label: '历史记录' },
  { key: 'general', label: '通用设置' },
  { key: 'hotkey', label: '快捷键' },
  { key: 'backup', label: '数据备份' },
  { key: 'about', label: '关于' },
];

const form = reactive<AppSettings>({
  max_history_count: 5000,
  auto_cleanup_days: 30,
  window_position: 'remember',
  smart_activate: true,
  copy_sound: false,
  search_position: 'bottom',
  focus_search_on_activate: false,
  click_action: 'copy',
  double_click_action: 'paste',
  paste_shortcut: 'ctrl_v',
  hide_window_after_copy: false,
  image_ocr: false,
  copy_as_plain_text: false,
  paste_as_plain_text: true,
  confirm_delete: true,
  auto_sort: false,
  hotkey: 'Alt+V',
  auto_start: false,
  number_key_shortcut: 'ctrl',
});

const shortcutError = ref('');
const storagePaths = ref<Record<string, string>>({
  data_dir: '',
  log_dir: '',
});
let unlistenShortcutError: UnlistenFn | null = null;
let isInitializing = true;

// 更新检查相关状态
const currentVersion = ref('0.1.0');
const updateStatus = ref<'idle' | 'checking' | 'available' | 'uptodate' | 'error'>('idle');
const latestVersion = ref('');
const updateNotes = ref('');
const updateDate = ref('');
const isDownloading = ref(false);
const skippedVersion = ref<string | null>(null);
const downloadProgress = ref(0);
let updateInstance: Update | null = null;

onMounted(async () => {
  await loadSettings();
  syncFromSettings();

  // 加载存储路径
  try {
    const paths = await invoke<Record<string, string>>('get_storage_paths');
    storagePaths.value = paths;
  } catch (error) {
    console.error('Failed to load storage paths:', error);
  }

  // 加载版本号并自动检查更新
  await loadAppVersion();
  setTimeout(() => {
    checkForUpdates();
  }, 1000);

  // 监听快捷键注册失败事件
  unlistenShortcutError = await listen<string>('shortcut-registration-failed', (event) => {
    shortcutError.value = `快捷键 "${event.payload}" 已被其他程序占用，请使用备用快捷键 Ctrl+Shift+V，或修改快捷键后重启应用`;
  });

  isInitializing = false;
});

onUnmounted(() => {
  if (unlistenShortcutError) {
    unlistenShortcutError();
  }
});

const syncFromSettings = () => {
  Object.assign(form, settings.value);
};

// 自动保存
watch(form, async (newValue) => {
  if (isInitializing) return;
  
  try {
    await saveSettings({ ...newValue });
  } catch (error) {
    console.error('自动保存设置失败:', error);
  }
}, { deep: true });

const resetSettings = async () => {
  if (confirm('确定要恢复默认设置吗？')) {
    isInitializing = true;

    Object.assign(form, {
      max_history_count: 5000,
      auto_cleanup_days: 30,
      window_position: 'remember',
      smart_activate: true,
      copy_sound: false,
      search_position: 'top',
      focus_search_on_activate: false,
      click_action: 'copy',
      double_click_action: 'paste',
      paste_shortcut: 'ctrl_v',
      hide_window_after_copy: false,
      image_ocr: false,
      copy_as_plain_text: false,
      paste_as_plain_text: true,
      confirm_delete: true,
      auto_sort: false,
      auto_start: false,
      number_key_shortcut: 'ctrl',
    });

    try {
      await saveSettings({ ...form });
    } catch (error) {
      console.error('保存默认设置失败:', error);
    }

    isInitializing = false;
  }
};

const validateHotkey = async () => {
  try {
    await invoke('validate_shortcut', { hotkey: form.hotkey });
    shortcutError.value = '';
  } catch (error) {
    shortcutError.value = '快捷键格式无效';
    form.hotkey = 'Alt+V';
  }
};

const handleClearAllHistory = async () => {
  const confirmed = confirm('确定要删除所有历史记录吗？此操作不可撤销！');
  if (!confirmed) return;

  try {
    await invoke('clear_clipboard_history', { 
      request: { keep_count: null, keep_days: null }
    });
    alert('历史记录已清空');
    await loadHistory();
  } catch (error) {
    console.error('清空历史记录失败:', error);
    alert('清空失败，请重试');
  }
};

const handleExport = async () => {
  try {
    const jsonData = await invoke<string>('export_clipboard_data');
    
    const blob = new Blob([jsonData], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `clipboard-backup-${new Date().toISOString().split('T')[0]}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
    
    alert('导出成功！');
  } catch (error) {
    console.error('导出失败:', error);
    alert('导出失败，请重试');
  }
};

const handleImport = async () => {
  try {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.json';
    input.onchange = async (e: Event) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (!file) return;
      
      const reader = new FileReader();
      reader.onload = async (event) => {
        try {
          const jsonData = event.target?.result as string;
          const count = await invoke<number>('import_clipboard_data', { jsonData });
          alert(`导入成功！共导入 ${count} 条记录`);
          await loadHistory();
        } catch (error) {
          console.error('导入失败:', error);
          alert('导入失败，请检查文件格式');
        }
      };
      reader.readAsText(file);
    };
    input.click();
  } catch (error) {
    console.error('导入失败:', error);
    alert('导入失败，请重试');
  }
};

const handlePathCopy = () => {
  alert('路径已复制到剪贴板');
};

const handlePathOpen = () => {
  // PathDisplay组件内部处理打开逻辑，这里只需要响应事件
};

// 更新检查
const loadAppVersion = async () => {
  try {
    const version = await invoke<string>('get_app_version');
    currentVersion.value = version;
  } catch (error) {
    console.error('获取版本号失败:', error);
  }
};

const checkForUpdates = async () => {
  updateStatus.value = 'checking';
  try {
    const update = await check({
      headers: {
        "X-AccessKey": "9SzxzOb3pQgkOB-LU-QU1Q",
      },
      timeout: 5000,
    });

    if (update) {
      if (skippedVersion.value === update.version) {
        updateStatus.value = 'uptodate';
      } else {
        updateInstance = update;
        latestVersion.value = update.version;
        updateDate.value = update.date || '';
        updateNotes.value = update.body || '';
        updateStatus.value = 'available';
      }
    } else {
      updateStatus.value = 'uptodate';
    }
  } catch (error) {
    console.error('检查更新失败:', error);
    updateStatus.value = 'error';
  }
};

const downloadUpdate = async () => {
  if (!updateInstance) return;

  isDownloading.value = true;
  downloadProgress.value = 0;

  try {
    await updateInstance.downloadAndInstall((event) => {
      switch (event.event) {
        case 'Started':
          break;
        case 'Progress':
          // 简化进度显示
          break;
        case 'Finished':
          break;
      }
    });

    await relaunch();
  } catch (error) {
    console.error('安装更新失败:', error);
    alert('安装更新失败，请稍后重试');
    isDownloading.value = false;
  }
};

const skipUpdate = () => {
  skippedVersion.value = latestVersion.value;
  updateStatus.value = 'uptodate';
};

const openExternalLink = async (url: string) => {
  try {
    await invoke('open_external_link', { url });
  } catch (error) {
    console.error('打开链接失败:', error);
    window.open(url, '_blank');
  }
};
</script>

<style scoped>
.settings-view {
  height: 100vh;
  display: flex;
  background-color: #f5f5f5;
}

/* 左侧导航栏 */
.sidebar {
  width: 220px;
  background: #fff;
  border-right: 1px solid #e8e8e8;
  display: flex;
  flex-direction: column;
}

.nav-menu {
  flex: 1;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px 16px;
  border: none;
  background: transparent;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  text-align: left;
}

.nav-item:hover {
  background: #f5f5f5;
}

.nav-item.active {
  background: #262626;
  color: #fff;
}

.nav-icon {
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.nav-icon svg {
  width: 100%;
  height: 100%;
}

.nav-label {
  font-size: 14px;
}

/* 右侧内容区 */
.content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* 底部操作栏 */
.settings-footer {
  display: flex;
  align-items: flex-end;
  justify-content: flex-end;
  background: #fff;
  padding: 12px;
  border-top: 1px solid #e8e8e8;
}

.btn-secondary {
  padding: 8px 16px;
  background: #fff;
  border: 1px solid #d9d9d9;
  border-radius: 6px;
  font-size: 13px;
  color: #595959;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary:hover {
  border-color: #262626;
  color: #262626;
}

/* Scrollbar */
::-webkit-scrollbar {
  width: 6px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: #c0c0c0;
  border-radius: 3px;
}

::-webkit-scrollbar-thumb:hover {
  background: #a0a0a0;
}
</style>
