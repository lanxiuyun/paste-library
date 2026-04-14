<template>
  <div class="settings-section">
    <h2 class="section-title">局域网同步</h2>

    <div class="setting-group">
      <SettingItem
        title="启用局域网同步"
        description="在同一局域网内同步文本剪贴板内容"
      >
        <label class="switch">
          <input type="checkbox" v-model="form.lan_sync_enabled" />
          <span class="slider"></span>
        </label>
      </SettingItem>

      <SettingItem
        title="设备名称"
        description="其他设备看到的名称，用于配对识别"
      >
        <input
          v-model.trim="form.lan_sync_device_name"
          class="text-input"
          maxlength="32"
          placeholder="我的设备"
        />
      </SettingItem>

      <SettingItem
        title="自动发现设备"
        description="通过局域网广播自动发现可配对设备"
      >
        <label class="switch">
          <input type="checkbox" v-model="form.lan_sync_discovery_enabled" />
          <span class="slider"></span>
        </label>
      </SettingItem>

      <SettingItem title="同步状态" full-width>
        <div class="status-card">
          <div class="status-row">
            <span class="status-label">服务状态</span>
            <span class="status-value" :class="{ active: status.running }">
              {{ status.running ? '运行中' : '已停止' }}
            </span>
          </div>
          <div class="status-row">
            <span class="status-label">本机端口</span>
            <span class="status-value">{{ status.tcp_port }}</span>
          </div>
          <div class="status-row">
            <span class="status-label">发现端口</span>
            <span class="status-value">{{ status.discovery_port }}</span>
          </div>
          <div class="status-row">
            <span class="status-label">设备 ID</span>
            <span class="status-value status-code">{{ status.device_id || '--' }}</span>
          </div>
          <div v-if="status.last_error" class="status-error">
            {{ status.last_error }}
          </div>
        </div>
      </SettingItem>
    </div>

    <h2 class="section-title">已发现设备</h2>
    <div class="setting-group">
      <div v-if="status.discovered_devices.length === 0" class="empty-state">
        暂未发现其他设备
      </div>
      <div
        v-for="device in status.discovered_devices"
        :key="device.device_id"
        class="device-row"
      >
        <div class="device-main">
          <div class="device-name">
            {{ device.device_name }}
            <span v-if="device.trusted" class="device-badge">已信任</span>
          </div>
          <div class="device-meta">{{ device.address }}:{{ device.tcp_port }}</div>
        </div>
        <button
          v-if="!device.trusted"
          class="btn-primary"
          @click="handleRequestPairing(device.device_id)"
        >
          请求配对
        </button>
      </div>
    </div>

    <h2 class="section-title">待确认请求</h2>
    <div class="setting-group">
      <div v-if="status.pending_requests.length === 0" class="empty-state">
        暂无待处理请求
      </div>
      <div
        v-for="request in status.pending_requests"
        :key="request.device_id"
        class="device-row"
      >
        <div class="device-main">
          <div class="device-name">{{ request.device_name }}</div>
          <div class="device-meta">{{ request.address }}:{{ request.tcp_port }}</div>
        </div>
        <div class="device-actions">
          <button class="btn-secondary" @click="handleReject(request.device_id)">
            拒绝
          </button>
          <button class="btn-primary" @click="handleApprove(request.device_id)">
            允许
          </button>
        </div>
      </div>
    </div>

    <h2 class="section-title">已信任设备</h2>
    <div class="setting-group">
      <div v-if="status.trusted_devices.length === 0" class="empty-state">
        暂无已信任设备
      </div>
      <div
        v-for="device in status.trusted_devices"
        :key="device.device_id"
        class="device-row"
      >
        <div class="device-main">
          <div class="device-name">{{ device.device_name }}</div>
          <div class="device-meta">{{ device.address }}:{{ device.tcp_port }}</div>
        </div>
        <button
          class="btn-secondary btn-danger"
          @click="handleRemoveTrusted(device.device_id)"
        >
          移除
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import SettingItem from '../components/SettingItem.vue'
import { useLanSync } from '@/composables/useLanSync'
import type { AppSettings } from '@/types'

interface Props {
  form: AppSettings;
}

defineProps<Props>()

const {
  status,
  requestPairing,
  approveDevice,
  rejectDevice,
  removeTrustedDevice,
} = useLanSync()

const handleRequestPairing = async (deviceId: string) => {
  try {
    await requestPairing(deviceId)
  } catch (error) {
    console.error('Request pairing failed:', error)
  }
}

const handleApprove = async (deviceId: string) => {
  try {
    await approveDevice(deviceId)
  } catch (error) {
    console.error('Approve pairing failed:', error)
  }
}

const handleReject = async (deviceId: string) => {
  try {
    await rejectDevice(deviceId)
  } catch (error) {
    console.error('Reject pairing failed:', error)
  }
}

const handleRemoveTrusted = async (deviceId: string) => {
  try {
    await removeTrustedDevice(deviceId)
  } catch (error) {
    console.error('Remove trusted device failed:', error)
  }
}
</script>

<style scoped>
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
  background-color: #fff;
  transition: 0.2s;
  border-radius: 50%;
}

input:checked + .slider {
  background-color: #262626;
}

input:checked + .slider:before {
  transform: translateX(20px);
}

.text-input {
  min-width: 180px;
  padding: 6px 10px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-size: 13px;
  outline: none;
}

.text-input:focus {
  border-color: #262626;
}

.status-card {
  display: grid;
  gap: 10px;
}

.status-row {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  font-size: 13px;
}

.status-label {
  color: #8c8c8c;
}

.status-value {
  color: #262626;
}

.status-value.active {
  color: #389e0d;
  font-weight: 600;
}

.status-code {
  max-width: 420px;
  text-align: right;
  word-break: break-all;
}

.status-error {
  font-size: 12px;
  color: #ff4d4f;
  background: #fff2f0;
  border: 1px solid #ffccc7;
  border-radius: 6px;
  padding: 10px 12px;
}

.empty-state {
  padding: 16px 0;
  color: #8c8c8c;
  font-size: 13px;
}

.device-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 16px 0;
  border-bottom: 1px solid #f0f0f0;
}

.device-row:last-child {
  border-bottom: none;
}

.device-main {
  min-width: 0;
  flex: 1;
}

.device-name {
  display: flex;
  align-items: center;
  gap: 8px;
  color: #262626;
  font-size: 14px;
  font-weight: 500;
}

.device-meta {
  margin-top: 4px;
  color: #8c8c8c;
  font-size: 12px;
  word-break: break-all;
}

.device-badge {
  display: inline-flex;
  align-items: center;
  padding: 2px 8px;
  border-radius: 999px;
  background: #f6ffed;
  color: #389e0d;
  font-size: 11px;
  font-weight: 500;
}

.device-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.btn-primary,
.btn-secondary {
  padding: 8px 14px;
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary {
  border: none;
  background: #262626;
  color: #fff;
}

.btn-primary:hover {
  background: #434343;
}

.btn-secondary {
  border: 1px solid #d9d9d9;
  background: #fff;
  color: #595959;
}

.btn-secondary:hover {
  border-color: #262626;
  color: #262626;
}

.btn-danger:hover {
  border-color: #ff4d4f;
  color: #ff4d4f;
}
</style>
