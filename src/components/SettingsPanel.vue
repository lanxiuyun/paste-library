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
      <!-- 剪贴板设置 -->
      <div v-if="activeMenu === 'clipboard'" class="settings-section">
        <h2 class="section-title">窗口设置</h2>
        
        <div class="setting-group">
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">窗口位置</div>
            </div>
            <div class="setting-control">
              <select v-model="form.window_position" class="select-input">
                <option value="remember">记住位置</option>
                <option value="center">居中显示</option>
                <option value="cursor">跟随鼠标</option>
              </select>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">激活时回到顶部</div>
              <div class="setting-desc">激活窗口时，滚动至顶部并选中首条</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.scroll_to_top_on_activate" />
                <span class="slider"></span>
              </label>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">激活时切换至全部分组</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.switch_to_all_on_activate" />
                <span class="slider"></span>
              </label>
            </div>
          </div>
        </div>

        <h2 class="section-title">音效设置</h2>
        
        <div class="setting-group">
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">复制音效</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.copy_sound" />
                <span class="slider"></span>
              </label>
              <button class="icon-btn" title="预览音效">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/>
                  <path d="M19.07 4.93a10 10 0 010 14.14M15.54 8.46a5 5 0 010 7.07"/>
                </svg>
              </button>
            </div>
          </div>
        </div>

        <h2 class="section-title">搜索设置</h2>
        
        <div class="setting-group">
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">搜索框位置</div>
            </div>
            <div class="setting-control">
              <select v-model="form.search_position" class="select-input">
                <option value="top">顶部</option>
                <option value="bottom">底部</option>
              </select>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">默认聚焦</div>
              <div class="setting-desc">激活窗口时，默认聚焦搜索框</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.auto_focus_search" />
                <span class="slider"></span>
              </label>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">自动清除</div>
              <div class="setting-desc">激活窗口时，清除搜索框内容</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.clear_search_on_activate" />
                <span class="slider"></span>
              </label>
            </div>
          </div>
        </div>

        <h2 class="section-title">内容设置</h2>
        
        <div class="setting-group">
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">自动粘贴</div>
              <div class="setting-desc">鼠标左键操作时，快速粘贴内容至输入位置</div>
            </div>
            <div class="setting-control">
              <select v-model="form.auto_paste" class="select-input">
                <option value="off">关闭</option>
                <option value="single">单击</option>
                <option value="double">双击</option>
              </select>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">图片OCR</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.image_ocr" />
                <span class="slider"></span>
              </label>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">复制为纯文本</div>
              <div class="setting-desc">富文本和HTML格式在复制时仅保留纯文本内容</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.copy_as_plain_text" />
                <span class="slider"></span>
              </label>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">粘贴为纯文本</div>
              <div class="setting-desc">富文本和HTML格式在粘贴时仅保留纯文本内容</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.paste_as_plain_text" />
                <span class="slider"></span>
              </label>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">操作按钮</div>
              <div class="setting-desc">自定义操作剪贴板内容的图标按钮</div>
            </div>
            <div class="setting-control">
              <button class="btn-secondary">自定义</button>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">自动收藏</div>
              <div class="setting-desc">新增或编辑备注后自动收藏</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.auto_favorite" />
                <span class="slider"></span>
              </label>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">删除确认</div>
              <div class="setting-desc">删除剪贴板内容时弹出确认对话框</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.confirm_delete" />
                <span class="slider"></span>
              </label>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">自动排序</div>
              <div class="setting-desc">复制已存在的内容时排列到最前面</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.auto_sort" />
                <span class="slider"></span>
              </label>
            </div>
          </div>
        </div>
      </div>

      <!-- 历史记录设置 -->
      <div v-if="activeMenu === 'history'" class="settings-section">
        <h2 class="section-title">历史记录设置</h2>
        
        <div class="setting-group">
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">最大历史记录数</div>
              <div class="setting-desc">达到上限后自动删除最早的数据</div>
            </div>
            <div class="setting-control">
              <input 
                type="number" 
                v-model.number="form.max_history_count"
                min="100"
                max="10000"
                step="100"
                class="number-input"
              />
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">自动清理</div>
              <div class="setting-desc">自动删除超过指定天数的历史记录</div>
            </div>
            <div class="setting-control">
              <select v-model.number="form.auto_cleanup_days" class="select-input">
                <option :value="0">不自动清理</option>
                <option :value="7">7天</option>
                <option :value="30">30天</option>
                <option :value="90">90天</option>
              </select>
            </div>
          </div>
        </div>
      </div>

      <!-- 通用设置 -->
      <div v-if="activeMenu === 'general'" class="settings-section">
        <h2 class="section-title">通用设置</h2>
        
        <div class="setting-group">
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">开机自启</div>
              <div class="setting-desc">系统启动时自动运行 Paste Library</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.auto_start" />
                <span class="slider"></span>
              </label>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">应用黑名单</div>
              <div class="setting-desc">来自这些应用的剪贴板内容将被忽略（每行一个应用名）</div>
            </div>
          </div>
          <div class="setting-item full-width">
            <textarea 
              v-model="blacklistText"
              class="textarea-input"
              rows="4"
              placeholder="例如：&#10;Password Manager&#10;1Password&#10;KeePass"
            />
          </div>
        </div>
      </div>

      <!-- 快捷键设置 -->
      <div v-if="activeMenu === 'hotkey'" class="settings-section">
        <h2 class="section-title">快捷键设置</h2>
        
        <div class="setting-group">
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">唤醒快捷键</div>
              <div class="setting-desc">按下此快捷键可快速打开或关闭剪贴板窗口</div>
              <div class="setting-note">点击下方按钮开始录制，然后按下想要的快捷键组合</div>
              <div v-if="shortcutError" class="error-message">{{ shortcutError }}</div>
            </div>
            <div class="setting-control">
              <button
                class="hotkey-record-btn"
                :class="{ 'recording': isRecordingHotkey, 'has-value': form.hotkey && !isRecordingHotkey }"
                @click="toggleHotkeyRecording"
              >
                {{ isRecordingHotkey ? '请按下快捷键...' : (form.hotkey || '点击录制') }}
              </button>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">窗口尺寸</div>
              <div class="setting-desc">剪贴板窗口的默认大小</div>
            </div>
            <div class="setting-control size-control">
              <input 
                type="number" 
                v-model.number="form.window_width"
                min="400"
                max="1200"
                class="number-input small"
              />
              <span class="size-separator">×</span>
              <input 
                type="number" 
                v-model.number="form.window_height"
                min="300"
                max="900"
                class="number-input small"
              />
            </div>
          </div>
        </div>
      </div>

      <!-- 数据备份 -->
      <div v-if="activeMenu === 'backup'" class="settings-section">
        <h2 class="section-title">数据备份</h2>
        
        <div class="setting-group">
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">导出数据</div>
              <div class="setting-desc">将剪贴板历史导出为 JSON 文件</div>
            </div>
            <div class="setting-control">
              <button class="btn-secondary" @click="exportData">
                导出
              </button>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">导入数据</div>
              <div class="setting-desc">从 JSON 文件导入剪贴板历史</div>
            </div>
            <div class="setting-control">
              <button class="btn-secondary" @click="importData">
                导入
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 关于 -->
      <div v-if="activeMenu === 'about'" class="settings-section">
        <h2 class="section-title">关于</h2>
        
        <div class="about-content">
          <div class="app-info">
            <h3>Paste Library</h3>
            <p>版本 0.1.0</p>
            <p class="app-desc">现代化的剪贴板管理工具</p>
          </div>
          
          <div class="about-actions">
            <button class="open-clipboard-btn" @click="openClipboardWindow">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
              </svg>
              打开剪贴板
            </button>
          </div>
        </div>
      </div>

      <!-- 底部操作栏 -->
      <div v-if="activeMenu !== 'about'" class="settings-footer">
        <button class="btn-secondary" @click="resetSettings">恢复默认</button>
        <button class="btn-primary" @click="saveAllSettings" :disabled="!hasChanges">
          {{ hasChanges ? '保存更改' : '已保存' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, reactive, onUnmounted } from 'vue';
import { useSettings } from '@/composables/useSettings';
import { useWindow } from '@/composables/useWindow';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { AppSettings } from '@/types';

const { settings, loadSettings, saveSettings } = useSettings();
const { openClipboardWindow } = useWindow();

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
  window_width: 800,
  window_height: 600,
  scroll_to_top_on_activate: false,
  switch_to_all_on_activate: true,
  copy_sound: false,
  search_position: 'bottom',
  auto_focus_search: true,
  clear_search_on_activate: false,
  auto_paste: 'double',
  image_ocr: false,
  copy_as_plain_text: false,
  paste_as_plain_text: true,
  auto_favorite: false,
  confirm_delete: true,
  auto_sort: false,
  left_click_action: 'copy',
  hotkey: 'Alt+V',
  auto_start: false,
  blacklist_apps: [],
});

const blacklistText = ref('');
const originalSettings = ref<AppSettings | null>(null);
const shortcutError = ref('');
const isRecordingHotkey = ref(false);
let unlistenShortcutError: UnlistenFn | null = null;

// 录制快捷键
const toggleHotkeyRecording = () => {
  if (isRecordingHotkey.value) {
    // 停止录制
    isRecordingHotkey.value = false;
    window.removeEventListener('keydown', handleHotkeyRecord);
  } else {
    // 开始录制
    isRecordingHotkey.value = true;
    window.addEventListener('keydown', handleHotkeyRecord, { capture: true });
  }
};

const handleHotkeyRecord = (e: KeyboardEvent) => {
  e.preventDefault();
  e.stopPropagation();

  const modifiers: string[] = [];
  if (e.ctrlKey) modifiers.push('Ctrl');
  if (e.altKey) modifiers.push('Alt');
  if (e.shiftKey) modifiers.push('Shift');
  if (e.metaKey) modifiers.push('Win');

  // 获取按键名称
  let key = e.key;

  // 忽略单独的修饰键
  if (key === 'Control' || key === 'Alt' || key === 'Shift' || key === 'Meta') {
    return;
  }

  // 标准化按键名称
  if (key === ' ') key = 'Space';
  if (key.length === 1) key = key.toUpperCase();

  // 组合快捷键
  const hotkeyParts = [...modifiers, key];
  const hotkeyString = hotkeyParts.join('+');

  // 验证并设置
  form.hotkey = hotkeyString;
  validateHotkey();

  // 停止录制
  isRecordingHotkey.value = false;
  window.removeEventListener('keydown', handleHotkeyRecord, { capture: true });
};

onMounted(async () => {
  await loadSettings();
  syncFromSettings();
  originalSettings.value = { ...settings.value };

  // 监听快捷键注册失败事件
  unlistenShortcutError = await listen<string>('shortcut-registration-failed', (event) => {
    shortcutError.value = `快捷键 "${event.payload}" 已被其他程序占用，请使用备用快捷键 Ctrl+Shift+V，或修改快捷键后重启应用`;
  });
});

onUnmounted(() => {
  if (unlistenShortcutError) {
    unlistenShortcutError();
  }
  // 清理录制监听器
  if (isRecordingHotkey.value) {
    window.removeEventListener('keydown', handleHotkeyRecord, { capture: true });
  }
});

const syncFromSettings = () => {
  Object.assign(form, settings.value);
  blacklistText.value = settings.value.blacklist_apps.join('\n');
};

const currentSettings = computed((): AppSettings => ({
  ...form,
  blacklist_apps: blacklistText.value.split('\n').filter(s => s.trim()),
}));

const hasChanges = computed(() => {
  if (!originalSettings.value) return false;
  const current = currentSettings.value;
  const original = originalSettings.value;
  
  return (
    current.max_history_count !== original.max_history_count ||
    current.auto_cleanup_days !== original.auto_cleanup_days ||
    current.window_position !== original.window_position ||
    current.window_width !== original.window_width ||
    current.window_height !== original.window_height ||
    current.scroll_to_top_on_activate !== original.scroll_to_top_on_activate ||
    current.switch_to_all_on_activate !== original.switch_to_all_on_activate ||
    current.copy_sound !== original.copy_sound ||
    current.search_position !== original.search_position ||
    current.auto_focus_search !== original.auto_focus_search ||
    current.clear_search_on_activate !== original.clear_search_on_activate ||
    current.auto_paste !== original.auto_paste ||
    current.image_ocr !== original.image_ocr ||
    current.copy_as_plain_text !== original.copy_as_plain_text ||
    current.paste_as_plain_text !== original.paste_as_plain_text ||
    current.auto_favorite !== original.auto_favorite ||
    current.confirm_delete !== original.confirm_delete ||
    current.auto_sort !== original.auto_sort ||
    current.hotkey !== original.hotkey ||
    current.left_click_action !== original.left_click_action ||
    current.auto_start !== original.auto_start ||
    JSON.stringify(current.blacklist_apps) !== JSON.stringify(original.blacklist_apps)
  );
});

const saveAllSettings = async () => {
  await saveSettings(currentSettings.value);
  originalSettings.value = { ...currentSettings.value };
};

const resetSettings = () => {
  if (confirm('确定要恢复默认设置吗？')) {
    form.max_history_count = 5000;
    form.auto_cleanup_days = 30;
    form.window_position = 'remember';
    form.window_width = 800;
    form.window_height = 600;
    form.scroll_to_top_on_activate = false;
    form.switch_to_all_on_activate = true;
    form.copy_sound = false;
    form.search_position = 'bottom';
    form.auto_focus_search = true;
    form.clear_search_on_activate = false;
    form.auto_paste = 'double';
    form.image_ocr = false;
    form.copy_as_plain_text = false;
    form.paste_as_plain_text = true;
    form.auto_favorite = false;
    form.confirm_delete = true;
    form.auto_sort = false;
    form.auto_start = false;
    blacklistText.value = '';
  }
};

const exportData = () => {
  console.log('Export data');
};

const importData = () => {
  console.log('Import data');
};

const hotkeyError = ref('');

const validateHotkey = async () => {
  try {
    await invoke('validate_shortcut', { hotkey: form.hotkey });
    hotkeyError.value = '';
  } catch (error) {
    hotkeyError.value = '快捷键格式无效';
    // 恢复到默认值
    form.hotkey = 'Alt+V';
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

.settings-section {
  flex: 1;
  padding: 24px 32px;
  overflow-y: auto;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: #262626;
  margin: 0 0 12px 0;
}

.section-title:not(:first-child) {
  margin-top: 12px;
}

.setting-group {
  background: #fff;
  border-radius: 8px;
  padding: 0 20px;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 0;
  border-bottom: 1px solid #f0f0f0;
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-item.full-width {
  flex-direction: column;
  align-items: flex-start;
  gap: 12px;
}

.setting-item.full-width .setting-control {
  width: 100%;
}

.setting-info {
  flex: 1;
}

.setting-title {
  font-size: 14px;
  font-weight: 500;
  color: #262626;
  margin-bottom: 4px;
}

.setting-desc {
  font-size: 12px;
  color: #8c8c8c;
}

.setting-control {
  display: flex;
  align-items: center;
  gap: 12px;
}

.hotkey-record-btn {
  min-width: 120px;
  padding: 8px 16px;
  border: 2px dashed #d9d9d9;
  border-radius: 4px;
  background: #fafafa;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
  font-size: 13px;
  color: #8c8c8c;
  cursor: pointer;
  transition: all 0.2s;
  text-align: center;
}

.hotkey-record-btn:hover {
  border-color: #262626;
  color: #262626;
}

.hotkey-record-btn.recording {
  border-color: #fa8c16;
  border-style: solid;
  background: #fff7e6;
  color: #fa8c16;
  animation: pulse 1s infinite;
}

.hotkey-record-btn.has-value {
  border-style: solid;
  border-color: #52c41a;
  background: #f6ffed;
  color: #52c41a;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.6; }
}

.hotkey-input {
  width: 120px;
  padding: 6px 10px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
  font-size: 13px;
  color: #262626;
  text-align: center;
}

.hotkey-input:focus {
  border-color: #262626;
  outline: none;
}

.setting-note {
  font-size: 11px;
  color: #faad14;
  margin-top: 4px;
}

.error-message {
  font-size: 11px;
  color: #ff4d4f;
  margin-top: 4px;
  padding: 4px 8px;
  background: #fff2f0;
  border: 1px solid #ffccc7;
  border-radius: 4px;
}

.hotkey-input.has-error {
  border-color: #ff4d4f;
  background: #fff2f0;
}

.number-input {
  width: 80px;
  padding: 6px 10px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-size: 13px;
  text-align: center;
  outline: none;
}

.number-input.small {
  width: 60px;
}

.number-input:focus {
  border-color: #262626;
}

.select-input {
  padding: 6px 28px 6px 10px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-size: 13px;
  background: #fff;
  cursor: pointer;
  outline: none;
  min-width: 120px;
}

.select-input:focus {
  border-color: #262626;
}

.size-control {
  display: flex;
  align-items: center;
  gap: 8px;
}

.size-separator {
  color: #8c8c8c;
  font-size: 13px;
}

.textarea-input {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-size: 13px;
  resize: vertical;
  outline: none;
  font-family: inherit;
}

.textarea-input:focus {
  border-color: #262626;
}

.textarea-input::placeholder {
  color: #bfbfbf;
}

/* Switch toggle */
.switch {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 24px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #d9d9d9;
  transition: 0.2s;
  border-radius: 24px;
}

.slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: 0.2s;
  border-radius: 50%;
}

input:checked + .slider {
  background-color: #262626;
}

input:checked + .slider:before {
  transform: translateX(20px);
}

/* Icon button */
.icon-btn {
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #8c8c8c;
  transition: all 0.2s;
}

.icon-btn:hover {
  background: #f5f5f5;
  color: #262626;
}

.icon-btn svg {
  width: 16px;
  height: 16px;
}

/* 关于页面 */
.about-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  text-align: center;
}

.app-info h3 {
  font-size: 24px;
  font-weight: 600;
  color: #262626;
  margin: 0 0 8px 0;
}

.app-info p {
  font-size: 14px;
  color: #8c8c8c;
  margin: 0;
}

.app-desc {
  margin-top: 8px;
  font-size: 13px;
}

.about-actions {
  margin-top: 32px;
}

.open-clipboard-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  background: #262626;
  color: #fff;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  cursor: pointer;
  transition: background 0.2s;
}

.open-clipboard-btn:hover {
  background: #404040;
}

.open-clipboard-btn svg {
  width: 16px;
  height: 16px;
}

/* 底部操作栏 */
.settings-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 32px;
  background: #fff;
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

.btn-primary {
  padding: 8px 16px;
  background: #262626;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  color: #fff;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-primary:hover:not(:disabled) {
  background: #404040;
}

.btn-primary:disabled {
  background: #d9d9d9;
  cursor: not-allowed;
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
