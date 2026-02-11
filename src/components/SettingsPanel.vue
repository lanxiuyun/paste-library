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
              <div class="setting-title">智能激活</div>
              <div class="setting-desc">5秒内复制过内容时，激活窗口自动回到顶部、切换至全部、聚焦搜索框</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.smart_activate" />
                <span class="slider"></span>
              </label>
            </div>
          </div>
        </div>

        <!-- 音效设置 - 已隐藏
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
        -->

        <!-- 搜索设置 - 搜索框位置已隐藏，默认顶部 -->
        <h2 class="section-title">搜索设置</h2>
        
        <div class="setting-group">
          <!-- 搜索框位置 - 已隐藏，默认顶部
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
          -->

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

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">删除历史记录</div>
              <div class="setting-desc">永久删除所有剪贴板历史记录（此操作不可撤销）</div>
            </div>
            <div class="setting-control">
              <button class="btn-danger" @click="clearAllHistory">
                删除全部
              </button>
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
              <div class="setting-warning">修改快捷键后需要重启应用才能生效</div>
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

        <h2 class="section-title">存储路径</h2>
        
        <div class="setting-group">
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">数据存储路径</div>
              <div class="setting-desc">剪贴板历史和设置数据存储位置</div>
            </div>
          </div>
          <div class="setting-item full-width">
            <div class="path-display">
              <span class="path-text">{{ storagePaths.data_dir }}</span>
              <button class="icon-btn" title="复制路径" @click="copyToClipboard(storagePaths.data_dir)">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                  <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
                </svg>
              </button>
              <button class="icon-btn" title="打开文件夹" @click="openFolder(storagePaths.data_dir)">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
                </svg>
              </button>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">日志存储路径</div>
              <div class="setting-desc">应用日志文件存储位置</div>
            </div>
          </div>
          <div class="setting-item full-width">
            <div class="path-display">
              <span class="path-text">{{ storagePaths.log_dir }}</span>
              <button class="icon-btn" title="复制路径" @click="copyToClipboard(storagePaths.log_dir)">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                  <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
                </svg>
              </button>
              <button class="icon-btn" title="打开文件夹" @click="openFolder(storagePaths.log_dir)">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
                </svg>
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
        <button class="btn-secondary" @click="resetSettings">恢复默认设置</button>
        <span class="save-hint">设置会自动保存</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, reactive, onUnmounted, watch } from 'vue';
import { useSettings } from '@/composables/useSettings';
import { useWindow } from '@/composables/useWindow';
import { useClipboard } from '@/composables/useClipboard';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { AppSettings } from '@/types';

const { settings, loadSettings, saveSettings } = useSettings();
const { openClipboardWindow } = useWindow();
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
  auto_focus_search: true,
  auto_paste: 'double',
  image_ocr: false,
  copy_as_plain_text: false,
  paste_as_plain_text: true,
  confirm_delete: true,
  auto_sort: false,
  left_click_action: 'copy',
  hotkey: 'Alt+V',
  auto_start: false,
});

const shortcutError = ref('');
const isRecordingHotkey = ref(false);
const storagePaths = ref<Record<string, string>>({
  data_dir: '',
  log_dir: '',
});
let unlistenShortcutError: UnlistenFn | null = null;
let isInitializing = true;

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

  // 加载存储路径
  try {
    const paths = await invoke<Record<string, string>>('get_storage_paths');
    storagePaths.value = paths;
  } catch (error) {
    console.error('Failed to load storage paths:', error);
  }

  // 监听快捷键注册失败事件
  unlistenShortcutError = await listen<string>('shortcut-registration-failed', (event) => {
    shortcutError.value = `快捷键 "${event.payload}" 已被其他程序占用，请使用备用快捷键 Ctrl+Shift+V，或修改快捷键后重启应用`;
  });

  // 初始化完成后，开始监听变化
  isInitializing = false;
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
};

// 自动保存：监听 form 变化，变化时自动保存
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
    // 先标记为初始化中，避免重复保存
    isInitializing = true;
    
    form.max_history_count = 5000;
    form.auto_cleanup_days = 30;
    form.window_position = 'remember';
    form.smart_activate = true;
    form.copy_sound = false;
    form.search_position = 'top';
    form.auto_focus_search = true;
    form.auto_paste = 'double';
    form.image_ocr = false;
    form.copy_as_plain_text = false;
    form.paste_as_plain_text = true;
    form.confirm_delete = true;
    form.auto_sort = false;
    form.auto_start = false;
    
    // 立即保存
    try {
      await saveSettings({ ...form });
    } catch (error) {
      console.error('保存默认设置失败:', error);
    }
    
    // 恢复监听
    isInitializing = false;
  }
};

const exportData = async () => {
  try {
    const jsonData = await invoke<string>('export_clipboard_data');
    
    // 创建 Blob 并下载
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

const importData = async () => {
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

const copyToClipboard = async (path: string) => {
  try {
    await navigator.clipboard.writeText(path);
    alert('路径已复制到剪贴板');
  } catch (error) {
    console.error('复制失败:', error);
    alert('复制失败，请重试');
  }
};

const openFolder = async (path: string) => {
  try {
    await invoke('show_in_folder', { path });
  } catch (error) {
    console.error('打开文件夹失败:', error);
    alert('打开文件夹失败，请重试');
  }
};

const clearAllHistory = async () => {
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

.setting-warning {
  font-size: 11px;
  color: #ff4d4f;
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

/* 路径显示 */
.path-display {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 10px 12px;
  background: #fafafa;
  border: 1px solid #e8e8e8;
  border-radius: 4px;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
  font-size: 12px;
}

.path-text {
  flex: 1;
  color: #595959;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
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
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  padding: 16px 32px;
  background: #fff;
  border-top: 1px solid #e8e8e8;
}

.save-hint {
  font-size: 12px;
  color: #8c8c8c;
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

.btn-danger {
  padding: 8px 16px;
  background: #ff4d4f;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  color: #fff;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-danger:hover {
  background: #ff7875;
}

.btn-danger:active {
  background: #d9363e;
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
