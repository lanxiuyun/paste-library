import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { onMounted, onUnmounted, ref } from 'vue'
import type { LanSyncStatus } from '@/types'

const defaultStatus = (): LanSyncStatus => ({
  running: false,
  device_id: '',
  device_name: '',
  discovery_enabled: true,
  tcp_port: 48571,
  discovery_port: 48572,
  discovered_devices: [],
  trusted_devices: [],
  pending_requests: [],
})

const status = ref<LanSyncStatus>(defaultStatus())
let isListening = false
let cleanupListener: (() => void) | null = null

export function useLanSync() {
  const loadLanSyncStatus = async (): Promise<void> => {
    try {
      status.value = await invoke<LanSyncStatus>('get_lan_sync_status')
    } catch (error) {
      console.error('Failed to load LAN sync status:', error)
    }
  }

  const requestPairing = async (deviceId: string): Promise<void> => {
    await invoke('request_lan_pairing', { deviceId })
    await loadLanSyncStatus()
  }

  const approveDevice = async (deviceId: string): Promise<void> => {
    await invoke('approve_lan_device', { deviceId })
    await loadLanSyncStatus()
  }

  const rejectDevice = async (deviceId: string): Promise<void> => {
    await invoke('reject_lan_device', { deviceId })
    await loadLanSyncStatus()
  }

  const removeTrustedDevice = async (deviceId: string): Promise<void> => {
    await invoke('remove_trusted_lan_device', { deviceId })
    await loadLanSyncStatus()
  }

  onMounted(() => {
    loadLanSyncStatus()

    if (!isListening) {
      isListening = true
      listen<LanSyncStatus>('lan-sync-status-changed', (event) => {
        status.value = event.payload
      }).then((unlisten) => {
        cleanupListener = unlisten
      })
    }
  })

  onUnmounted(() => {
    cleanupListener?.()
    cleanupListener = null
    isListening = false
  })

  return {
    status,
    loadLanSyncStatus,
    requestPairing,
    approveDevice,
    rejectDevice,
    removeTrustedDevice,
  }
}
