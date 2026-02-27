<template>
  <div class="settings-section">
    <h2 class="section-title">同步设置</h2>
    
    <!-- 同步开关 -->
    <div class="setting-item">
      <div class="setting-info">
        <span class="setting-label">启用同步</span>
        <span class="setting-desc">在同一局域网内同步剪贴板内容</span>
      </div>
      <label class="toggle-switch">
        <input 
          type="checkbox" 
          v-model="syncEnabled"
          @change="handleSyncToggle"
        />
        <span class="toggle-slider"></span>
      </label>
    </div>
    
    <!-- 自动同步开关 -->
    <div class="setting-item" v-if="syncEnabled">
      <div class="setting-info">
        <span class="setting-label">自动同步</span>
        <span class="setting-desc">剪贴板内容变化时自动同步到已配对设备</span>
      </div>
      <label class="toggle-switch">
        <input 
          type="checkbox" 
          v-model="autoSync"
          @change="handleAutoSyncToggle"
        />
        <span class="toggle-slider"></span>
      </label>
    </div>
    
    <!-- 设备发现 -->
    <div class="setting-item" v-if="syncEnabled">
      <div class="setting-info">
        <span class="setting-label">设备发现</span>
        <span class="setting-desc">搜索局域网内的 Paste Library 设备</span>
      </div>
      <button 
        class="btn-secondary"
        :disabled="isDiscovering"
        @click="toggleDiscovery"
      >
        {{ isDiscovering ? '搜索中...' : '搜索设备' }}
      </button>
    </div>
    
    <!-- 发现设备列表 -->
    <div class="device-list" v-if="syncEnabled && discoveredDevices.length > 0">
      <h3 class="subsection-title">发现设备</h3>
      <div 
        v-for="device in discoveredDevices" 
        :key="device.id"
        class="device-item"
      >
        <div class="device-info">
          <span class="device-name">{{ device.name }}</span>
          <span class="device-platform">{{ getPlatformLabel(device.platform) }}</span>
        </div>
        <div class="device-status">
          <span v-if="device.is_paired" class="status-paired">已配对</span>
          <button 
            v-else 
            class="btn-primary btn-sm"
            @click="handlePair(device)"
          >
            配对
          </button>
        </div>
      </div>
    </div>
    
    <!-- 已配对设备列表 -->
    <div class="device-list" v-if="syncEnabled && pairedDevices.length > 0">
      <h3 class="subsection-title">已配对设备</h3>
      <div 
        v-for="device in pairedDevices" 
        :key="device.id"
        class="device-item"
      >
        <div class="device-info">
          <span class="device-name">{{ device.name }}</span>
          <span class="device-platform">{{ getPlatformLabel(device.platform) }}</span>
          <span :class="['device-online', device.is_online ? 'online' : 'offline']">
            {{ device.is_online ? '在线' : '离线' }}
          </span>
        </div>
        <div class="device-actions">
          <button 
            class="btn-danger btn-sm"
            @click="handleUnpair(device)"
          >
            取消配对
          </button>
        </div>
      </div>
    </div>
    
    <!-- 空状态 -->
    <div class="empty-state" v-if="syncEnabled && discoveredDevices.length === 0 && pairedDevices.length === 0 && !isDiscovering">
      <p>未发现其他设备</p>
      <p class="hint">确保其他设备也开启了同步功能</p>
    </div>
    
    <!-- 同步说明 -->
    <div class="sync-note" v-if="syncEnabled">
      <h3 class="subsection-title">同步说明</h3>
      <ul>
        <li>文本内容会实时同步到已配对设备</li>
        <li>图片和文件仅同步引用（显示来源设备）</li>
        <li>点击图片/文件可按需获取内容</li>
        <li>删除操作会同步到所有设备</li>
        <li>标签和收藏为设备本地保存，不会同步</li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { SyncDevice, Platform } from '@/types';

// 状态
const syncEnabled = ref(false);
const autoSync = ref(true);
const isDiscovering = ref(false);
const discoveredDevices = ref<SyncDevice[]>([]);
const pairedDevices = ref<SyncDevice[]>([]);

// 平台标签
function getPlatformLabel(platform: Platform): string {
  const labels: Record<Platform, string> = {
    windows: 'Windows',
    macos: 'macOS',
    linux: 'Linux',
    android: 'Android'
  };
  return labels[platform] || platform;
}

// 切换同步开关
async function handleSyncToggle() {
  try {
    await invoke('set_sync_enabled', { enabled: syncEnabled.value });
    if (syncEnabled.value) {
      await startDiscovery();
    } else {
      await stopDiscovery();
    }
  } catch (err) {
    console.error('Failed to toggle sync:', err);
    syncEnabled.value = !syncEnabled.value;
  }
}

// 切换自动同步
async function handleAutoSyncToggle() {
  try {
    await invoke('set_auto_sync', { enabled: autoSync.value });
  } catch (err) {
    console.error('Failed to toggle auto sync:', err);
    autoSync.value = !autoSync.value;
  }
}

// 切换设备发现
async function toggleDiscovery() {
  if (isDiscovering.value) {
    await stopDiscovery();
  } else {
    await startDiscovery();
  }
}

// 开始发现设备
async function startDiscovery() {
  try {
    await invoke('start_device_discovery');
    isDiscovering.value = true;
    // 定时刷新设备列表
    refreshDevices();
  } catch (err) {
    console.error('Failed to start discovery:', err);
  }
}

// 停止发现设备
async function stopDiscovery() {
  try {
    await invoke('stop_device_discovery');
    isDiscovering.value = false;
  } catch (err) {
    console.error('Failed to stop discovery:', err);
  }
}

// 刷新设备列表
async function refreshDevices() {
  try {
    discoveredDevices.value = await invoke('get_discovered_devices');
    pairedDevices.value = await invoke('get_paired_devices');
  } catch (err) {
    console.error('Failed to refresh devices:', err);
  }
}

// 配对设备
async function handlePair(device: SyncDevice) {
  try {
    // 生成配对 PIN
    const pin = await invoke('request_pair', { deviceId: device.id });
    console.log('Pairing PIN:', pin);
    // TODO: 显示配对确认对话框
    // 确认后调用 confirm_pair
    await invoke('confirm_pair', { deviceId: device.id, pin });
    await refreshDevices();
  } catch (err) {
    console.error('Failed to pair device:', err);
  }
}

// 取消配对
async function handleUnpair(device: SyncDevice) {
  try {
    await invoke('unpair_device', { deviceId: device.id });
    await refreshDevices();
  } catch (err) {
    console.error('Failed to unpair device:', err);
  }
}

// 定时刷新
let refreshInterval: number | null = null;

onMounted(async () => {
  // 加载已配对设备
  try {
    pairedDevices.value = await invoke('get_paired_devices');
  } catch (err) {
    console.error('Failed to load paired devices:', err);
  }
  
  // 定时刷新
  refreshInterval = window.setInterval(refreshDevices, 5000);
});

onUnmounted(() => {
  if (refreshInterval) {
    clearInterval(refreshInterval);
  }
});
</script>

<style scoped>
.settings-section {
  padding: 20px;
}

.section-title {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 24px;
  color: #333;
}

.subsection-title {
  font-size: 14px;
  font-weight: 600;
  margin: 20px 0 12px;
  color: #666;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 0;
  border-bottom: 1px solid #f0f0f0;
}

.setting-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.setting-label {
  font-size: 14px;
  color: #333;
}

.setting-desc {
  font-size: 12px;
  color: #999;
}

/* Toggle Switch */
.toggle-switch {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 24px;
}

.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  transition: 0.3s;
  border-radius: 24px;
}

.toggle-slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: 0.3s;
  border-radius: 50%;
}

.toggle-switch input:checked + .toggle-slider {
  background-color: #4CAF50;
}

.toggle-switch input:checked + .toggle-slider:before {
  transform: translateX(20px);
}

/* Device List */
.device-list {
  margin-top: 16px;
}

.device-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background: #f9f9f9;
  border-radius: 8px;
  margin-bottom: 8px;
}

.device-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.device-name {
  font-size: 14px;
  font-weight: 500;
  color: #333;
}

.device-platform {
  font-size: 12px;
  color: #999;
  background: #eee;
  padding: 2px 8px;
  border-radius: 4px;
}

.device-online {
  font-size: 12px;
  padding: 2px 8px;
  border-radius: 4px;
}

.device-online.online {
  color: #4CAF50;
  background: #e8f5e9;
}

.device-online.offline {
  color: #999;
  background: #f5f5f5;
}

.status-paired {
  font-size: 12px;
  color: #4CAF50;
  background: #e8f5e9;
  padding: 2px 8px;
  border-radius: 4px;
}

/* Buttons */
.btn-secondary {
  padding: 8px 16px;
  font-size: 13px;
  border: 1px solid #ddd;
  border-radius: 6px;
  background: #fff;
  color: #333;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary:hover:not(:disabled) {
  background: #f5f5f5;
}

.btn-secondary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-primary {
  padding: 8px 16px;
  font-size: 13px;
  border: none;
  border-radius: 6px;
  background: #4CAF50;
  color: #fff;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary:hover {
  background: #45a049;
}

.btn-danger {
  padding: 8px 16px;
  font-size: 13px;
  border: none;
  border-radius: 6px;
  background: #f44336;
  color: #fff;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-danger:hover {
  background: #da190b;
}

.btn-sm {
  padding: 6px 12px;
  font-size: 12px;
}

/* Empty State */
.empty-state {
  text-align: center;
  padding: 40px 20px;
  color: #999;
}

.empty-state .hint {
  font-size: 12px;
  margin-top: 8px;
}

/* Sync Note */
.sync-note {
  margin-top: 24px;
  padding: 16px;
  background: #f9f9f9;
  border-radius: 8px;
}

.sync-note ul {
  margin: 0;
  padding-left: 20px;
}

.sync-note li {
  font-size: 13px;
  color: #666;
  margin-bottom: 8px;
}

.sync-note li:last-child {
  margin-bottom: 0;
}
</style>
