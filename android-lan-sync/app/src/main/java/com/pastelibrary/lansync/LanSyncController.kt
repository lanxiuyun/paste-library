package com.pastelibrary.lansync

import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import android.os.Build
import android.os.Handler
import android.os.Looper
import android.util.Log
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.cancel
import kotlinx.coroutines.delay
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch
import org.json.JSONObject
import java.io.BufferedInputStream
import java.net.DatagramPacket
import java.net.DatagramSocket
import java.net.InetAddress
import java.net.ServerSocket
import java.net.Socket
import java.net.SocketTimeoutException
import java.security.MessageDigest
import java.time.Instant
import java.util.UUID

private const val PROTOCOL_VERSION = 1
private const val DISCOVERY_INTERVAL_MS = 3000L
private const val DISCOVERY_STALE_MS = 12000L
private const val SOCKET_TIMEOUT_MS = 1000
private const val DEFAULT_TCP_PORT = 48571
private const val DEFAULT_DISCOVERY_PORT = 48572
private const val REMOTE_CLIPBOARD_TTL_MS = 4000L
private const val LOCAL_CLIPBOARD_DEDUPE_WINDOW_MS = 1500L
private const val REMOTE_CLIPBOARD_WRITE_RETRY_COUNT = 5
private const val REMOTE_CLIPBOARD_WRITE_RETRY_DELAY_MS = 120L
private const val MAX_LOG_LINES = 200
private const val LOG_TAG = "LanSync"
private const val KIND_HEARTBEAT = "heartbeat"
private const val KIND_PAIR_REQUEST = "pair_request"
private const val KIND_PAIR_RESPONSE = "pair_response"
private const val KIND_SYNC_TEXT = "sync_text"
const val DIRECTION_RECEIVED = "received"
const val DIRECTION_SENT = "sent"

data class DeviceEntry(
  val deviceId: String,
  var deviceName: String,
  var address: String,
  var tcpPort: Int,
  var trusted: Boolean,
  var lastSeen: Long,
)

data class PairingRequest(
  val deviceId: String,
  val deviceName: String,
  val address: String,
  val tcpPort: Int,
  val requestedAt: Long,
)

data class SyncedTextRecord(
  val hash: String,
  val text: String,
  val sourceDeviceId: String,
  val sourceDeviceName: String,
  val createdAt: String,
  val direction: String,
)

data class LanSyncSnapshot(
  val running: Boolean,
  val discoveryEnabled: Boolean,
  val deviceId: String,
  val deviceName: String,
  val latestPendingReceivedText: String?,
  val discoveredDevices: List<DeviceEntry>,
  val pendingRequests: List<PairingRequest>,
  val trustedDevices: List<DeviceEntry>,
  val syncedRecords: List<SyncedTextRecord>,
  val logs: List<String>,
)

object LanSyncController {
  private val stateLock = Any()
  private val mainHandler = Handler(Looper.getMainLooper())
  private val deviceId = "android-peer-${UUID.randomUUID().toString().take(8)}"
  private val discoveredDevices = mutableMapOf<String, DeviceEntry>()
  private val trustedDevices = mutableMapOf<String, DeviceEntry>()
  private val pendingRequests = mutableMapOf<String, PairingRequest>()
  private val syncedRecords = mutableListOf<SyncedTextRecord>()
  private val logs = mutableListOf<String>()
  private val observers = mutableSetOf<(LanSyncSnapshot) -> Unit>()

  private var appContext: Context? = null
  private var running = false
  private var discoveryEnabled = true
  private var deviceName = defaultDeviceName()
  private var latestPendingReceivedText: String? = null
  private var foregroundUiAttached = false
  private var scope: CoroutineScope? = null
  private var tcpServerJob: Job? = null
  private var udpListenerJob: Job? = null
  private var heartbeatJob: Job? = null
  private var pendingRemoteClipboardText: String? = null
  private var pendingRemoteClipboardExpiresAt = 0L
  private var lastObservedLocalClipboardHash: String? = null
  private var lastObservedLocalClipboardAt = 0L

  fun start(context: Context) {
    synchronized(stateLock) {
      appContext = context.applicationContext
      if (running) {
        notifyObserversLocked()
        return
      }
      running = true
      scope = CoroutineScope(SupervisorJob() + Dispatchers.IO)
      appendLogLocked("Starting LAN sync service")
      startTcpServerLocked()
      restartDiscoveryJobsLocked()
      notifyObserversLocked()
    }
  }

  fun stop() {
    synchronized(stateLock) {
      if (!running) {
        return
      }
      running = false
      tcpServerJob?.cancel()
      udpListenerJob?.cancel()
      heartbeatJob?.cancel()
      tcpServerJob = null
      udpListenerJob = null
      heartbeatJob = null
      scope?.cancel()
      scope = null
      appendLogLocked("LAN sync service stopped")
      notifyObserversLocked()
    }
  }

  fun setDeviceName(value: String) {
    synchronized(stateLock) {
      deviceName = value.ifBlank { defaultDeviceName() }
      notifyObserversLocked()
    }
  }

  fun setForegroundUiAttached(attached: Boolean) {
    synchronized(stateLock) {
      foregroundUiAttached = attached
      appendLogLocked("Foreground UI attached = $attached")
      notifyObserversLocked()
    }
  }

  fun setDiscoveryEnabled(enabled: Boolean) {
    synchronized(stateLock) {
      discoveryEnabled = enabled
      if (running) {
        restartDiscoveryJobsLocked()
      }
      notifyObserversLocked()
    }
  }

  fun snapshot(): LanSyncSnapshot = synchronized(stateLock) { snapshotLocked() }

  fun addObserver(observer: (LanSyncSnapshot) -> Unit) {
    val snapshot = synchronized(stateLock) {
      observers.add(observer)
      snapshotLocked()
    }
    observer(snapshot)
  }

  fun removeObserver(observer: (LanSyncSnapshot) -> Unit) {
    synchronized(stateLock) {
      observers.remove(observer)
    }
  }

  fun consumePendingRemoteClipboardText(text: String): Boolean = synchronized(stateLock) {
    consumePendingRemoteClipboardTextLocked(text)
  }

  fun submitObservedClipboardText(text: String): Boolean {
    val normalizedText = text.trim()
    if (normalizedText.isBlank()) {
      appendLog("Ignored observed clipboard text because it is blank")
      return false
    }

    synchronized(stateLock) {
      if (!running) {
        appendLogLocked("Ignored observed clipboard text because LAN sync is not running")
        return false
      }

      if (consumePendingRemoteClipboardTextLocked(normalizedText)) {
        appendLogLocked("Ignored observed clipboard text because it matches pending remote clipboard content")
        return false
      }

      val hash = computeTextHash(normalizedText)
      val now = System.currentTimeMillis()
      if (lastObservedLocalClipboardHash == hash && now - lastObservedLocalClipboardAt <= LOCAL_CLIPBOARD_DEDUPE_WINDOW_MS) {
        appendLogLocked("Ignored observed clipboard text because it is a local duplicate within debounce window")
        return false
      }

      lastObservedLocalClipboardHash = hash
      lastObservedLocalClipboardAt = now
      appendLogLocked("Accepted observed clipboard text for outbound sync: hash=${hash.take(12)} preview=${previewText(normalizedText)}")
    }

    sendLocalText(normalizedText)
    return true
  }

  fun sendLocalText(text: String) {
    if (text.isBlank()) {
      return
    }

    val payloadCreatedAt = Instant.now().toString()
    val payloadHash = computeTextHash(text)
    val currentName: String
    val launchScope: CoroutineScope

    synchronized(stateLock) {
      if (!running) {
        return
      }
      currentName = deviceName
      launchScope = scope ?: return
      upsertSyncedRecordLocked(
        SyncedTextRecord(
          hash = payloadHash,
          text = text,
          sourceDeviceId = deviceId,
          sourceDeviceName = currentName,
          createdAt = payloadCreatedAt,
          direction = DIRECTION_SENT,
        ),
      )
      appendLogLocked("Queued local text for sync: ${previewText(text)}")
      notifyObserversLocked()
    }

    launchScope.launch {
      sendSyncTextToTrustedDevices(text, payloadHash, payloadCreatedAt)
    }
  }

  fun copyLatestReceivedTextToClipboard(): Boolean {
    val text = synchronized(stateLock) {
      val pending = latestPendingReceivedText ?: return@synchronized null
      latestPendingReceivedText = null
      pendingRemoteClipboardText = pending
      pendingRemoteClipboardExpiresAt = System.currentTimeMillis() + REMOTE_CLIPBOARD_TTL_MS
      appendLogLocked("Copied pending received text to clipboard")
      notifyObserversLocked()
      pending
    } ?: return false

    writeTextToClipboard(text)
    return true
  }

  fun requestPairing(targetDeviceId: String) {
    val target = synchronized(stateLock) { discoveredDevices[targetDeviceId]?.copy() } ?: return
    scope?.launch {
      runCatching {
        sendTcpMessage(target.address, target.tcpPort, buildPairRequestJson())
        appendLog("Sent pairing request to ${target.deviceName}")
      }.onFailure {
        appendLog("Failed to send pairing request: ${it.message}")
      }
    }
  }

  fun approveRequest(targetDeviceId: String) {
    val request = synchronized(stateLock) { pendingRequests.remove(targetDeviceId) } ?: return
    synchronized(stateLock) {
      trustedDevices[request.deviceId] = DeviceEntry(
        request.deviceId,
        request.deviceName,
        request.address,
        request.tcpPort,
        true,
        System.currentTimeMillis(),
      )
      discoveredDevices[request.deviceId]?.trusted = true
      appendLogLocked("Approved pairing request from ${request.deviceName}")
      notifyObserversLocked()
    }
    sendPairResponse(request.address, request.tcpPort, true)
  }

  fun rejectRequest(targetDeviceId: String) {
    val request = synchronized(stateLock) { pendingRequests.remove(targetDeviceId) } ?: return
    synchronized(stateLock) {
      appendLogLocked("Rejected pairing request from ${request.deviceName}")
      notifyObserversLocked()
    }
    sendPairResponse(request.address, request.tcpPort, false)
  }

  fun removeTrustedDevice(targetDeviceId: String) {
    synchronized(stateLock) {
      if (trustedDevices.remove(targetDeviceId) == null) {
        return
      }
      discoveredDevices[targetDeviceId]?.trusted = false
      appendLogLocked("Removed trusted device $targetDeviceId")
      notifyObserversLocked()
    }
  }

  private fun startTcpServerLocked() {
    val currentScope = scope ?: return
    tcpServerJob?.cancel()
    tcpServerJob = currentScope.launch {
      try {
        ServerSocket(DEFAULT_TCP_PORT).use { server ->
          server.soTimeout = SOCKET_TIMEOUT_MS
          appendLog("TCP listening on 0.0.0.0:$DEFAULT_TCP_PORT")
          while (isActive) {
            try {
              handleIncomingClient(server.accept())
            } catch (_: SocketTimeoutException) {
            }
          }
        }
      } catch (error: Exception) {
        appendLog("TCP listener failed: ${error.message}")
      }
    }
  }

  private fun restartDiscoveryJobsLocked() {
    udpListenerJob?.cancel()
    heartbeatJob?.cancel()
    udpListenerJob = null
    heartbeatJob = null

    if (!running || !discoveryEnabled) {
      appendLogLocked("Discovery disabled, TCP sync remains available")
      return
    }

    val currentScope = scope ?: return
    udpListenerJob = currentScope.launch {
      try {
        DatagramSocket(DEFAULT_DISCOVERY_PORT).use { socket ->
          socket.soTimeout = SOCKET_TIMEOUT_MS
          appendLog("UDP discovery listening on 0.0.0.0:$DEFAULT_DISCOVERY_PORT")
          while (isActive) {
            val buffer = ByteArray(4096)
            val packet = DatagramPacket(buffer, buffer.size)
            try {
              socket.receive(packet)
              val payload = String(packet.data, 0, packet.length, Charsets.UTF_8).trim()
              val address = packet.address?.hostAddress ?: continue
              handleIncomingPayload(payload, address)
            } catch (_: SocketTimeoutException) {
              cleanupStaleDevices()
            }
          }
        }
      } catch (error: Exception) {
        appendLog("UDP listener failed: ${error.message}")
      }
    }

    heartbeatJob = currentScope.launch {
      while (isActive) {
        runCatching {
          sendUdpBroadcast(buildHeartbeatJson(), DEFAULT_DISCOVERY_PORT)
          cleanupStaleDevices()
        }.onFailure {
          appendLog("Heartbeat failed: ${it.message}")
        }
        delay(DISCOVERY_INTERVAL_MS)
      }
    }
  }

  private fun handleIncomingClient(client: Socket) {
    client.use { socket ->
      val payload = BufferedInputStream(socket.getInputStream()).readBytes().toString(Charsets.UTF_8)
      val address = socket.inetAddress.hostAddress ?: return
      appendLog("Received TCP message from $address:${socket.port}")
      handleIncomingPayload(payload, address)
    }
  }

  private fun handleIncomingPayload(payload: String, sourceAddress: String) {
    if (payload.isBlank()) {
      return
    }

    val json = runCatching { JSONObject(payload) }.getOrNull() ?: return
    if (json.optInt("protocol_version", -1) != PROTOCOL_VERSION) {
      return
    }

    val kind = json.optString("kind")
    val remoteDeviceId = json.optString("device_id")
    if (remoteDeviceId.isBlank() || remoteDeviceId == deviceId) {
      return
    }

    val remoteDeviceName = json.optString("device_name").ifBlank { "Unknown device" }
    val remoteTcpPort = json.optInt("tcp_port", DEFAULT_TCP_PORT).coerceIn(1, 65535)

    when (kind) {
      KIND_HEARTBEAT -> upsertDiscoveredDevice(remoteDeviceId, remoteDeviceName, sourceAddress, remoteTcpPort)
      KIND_PAIR_REQUEST -> handlePairRequest(remoteDeviceId, remoteDeviceName, sourceAddress, remoteTcpPort)
      KIND_PAIR_RESPONSE -> handlePairResponse(
        remoteDeviceId,
        remoteDeviceName,
        sourceAddress,
        remoteTcpPort,
        json.optBoolean("accepted", false),
      )
      KIND_SYNC_TEXT -> handleSyncText(json, remoteDeviceId, remoteDeviceName, sourceAddress, remoteTcpPort)
    }
  }

  private fun handlePairRequest(
    remoteDeviceId: String,
    remoteDeviceName: String,
    sourceAddress: String,
    remoteTcpPort: Int,
  ) {
    upsertDiscoveredDevice(remoteDeviceId, remoteDeviceName, sourceAddress, remoteTcpPort)
    val autoAccept = synchronized(stateLock) { trustedDevices.containsKey(remoteDeviceId) }
    if (autoAccept) {
      sendPairResponse(sourceAddress, remoteTcpPort, true)
      appendLog("Auto-accepted pairing request from $remoteDeviceName")
      return
    }

    synchronized(stateLock) {
      pendingRequests[remoteDeviceId] = PairingRequest(
        remoteDeviceId,
        remoteDeviceName,
        sourceAddress,
        remoteTcpPort,
        System.currentTimeMillis(),
      )
      appendLogLocked("Received pairing request from $remoteDeviceName ($sourceAddress:$remoteTcpPort)")
      notifyObserversLocked()
    }
  }

  private fun handlePairResponse(
    remoteDeviceId: String,
    remoteDeviceName: String,
    sourceAddress: String,
    remoteTcpPort: Int,
    accepted: Boolean,
  ) {
    synchronized(stateLock) {
      if (accepted) {
        trustedDevices[remoteDeviceId] = DeviceEntry(
          remoteDeviceId,
          remoteDeviceName,
          sourceAddress,
          remoteTcpPort,
          true,
          System.currentTimeMillis(),
        )
        appendLogLocked("Pairing succeeded with $remoteDeviceName")
      } else {
        appendLogLocked("$remoteDeviceName rejected the pairing request")
      }
      notifyObserversLocked()
    }
  }

  private fun handleSyncText(
    json: JSONObject,
    remoteDeviceId: String,
    remoteDeviceName: String,
    sourceAddress: String,
    remoteTcpPort: Int,
  ) {
    upsertDiscoveredDevice(remoteDeviceId, remoteDeviceName, sourceAddress, remoteTcpPort)
    val text = json.optString("text")
    if (text.isBlank()) {
      return
    }

    val launchScope = synchronized(stateLock) {
      if (!trustedDevices.containsKey(remoteDeviceId)) {
        appendLogLocked("Ignored sync_text from untrusted device $remoteDeviceName")
        notifyObserversLocked()
        return
      }

      upsertSyncedRecordLocked(
        SyncedTextRecord(
          hash = json.optString("hash").ifBlank { computeTextHash(text) },
          text = text,
          sourceDeviceId = remoteDeviceId,
          sourceDeviceName = remoteDeviceName,
          createdAt = json.optString("created_at").ifBlank { Instant.now().toString() },
          direction = DIRECTION_RECEIVED,
        ),
      )
      latestPendingReceivedText = null
      appendLogLocked(
        "Accepted sync_text from $remoteDeviceName: hash=${json.optString("hash").ifBlank { computeTextHash(text) }.take(12)} preview=${previewText(text)}",
      )
      appendLogLocked(
        "Received text from $remoteDeviceName, writing to clipboard: ${previewText(text)}",
      )
      pendingRemoteClipboardText = text
      pendingRemoteClipboardExpiresAt = System.currentTimeMillis() + REMOTE_CLIPBOARD_TTL_MS
      notifyObserversLocked()
      scope
    }

    launchScope?.launch {
      val copied = writeRemoteTextToClipboardWithRetry(text)
      synchronized(stateLock) {
        appendLogLocked(
          if (copied) {
            "Remote text copied to clipboard from $remoteDeviceName"
          } else {
            "Failed to copy remote text to clipboard from $remoteDeviceName"
          },
        )
        if (!copied) {
          latestPendingReceivedText = text
        }
        notifyObserversLocked()
      }
    }
  }

  private fun sendSyncTextToTrustedDevices(text: String, hash: String, createdAt: String) {
    val payload = buildSyncTextJson(text, hash, createdAt)
    val targets = synchronized(stateLock) { trustedDevices.values.map { it.copy() } }
    if (targets.isEmpty()) {
      appendLog("Recorded local text, but there are no trusted devices to sync")
      return
    }

    var sent = 0
    targets.forEach { target ->
      runCatching {
        sendTcpMessage(target.address, target.tcpPort, payload)
        sent += 1
      }.onFailure {
        appendLog("Failed to sync text to ${target.deviceName}: ${it.message}")
      }
    }
    appendLog("Broadcast text to $sent trusted device(s): ${previewText(text)}")
  }

  private fun buildHeartbeatJson(): String = baseMessage().apply {
    put("kind", KIND_HEARTBEAT)
  }.toString()

  private fun buildPairRequestJson(): String = baseMessage().apply {
    put("kind", KIND_PAIR_REQUEST)
  }.toString()

  private fun buildPairResponseJson(accepted: Boolean): String = baseMessage().apply {
    put("kind", KIND_PAIR_RESPONSE)
    put("accepted", accepted)
  }.toString()

  private fun buildSyncTextJson(text: String, hash: String, createdAt: String): String = baseMessage().apply {
    put("kind", KIND_SYNC_TEXT)
    put("hash", hash)
    put("created_at", createdAt)
    put("text", text)
  }.toString()

  private fun baseMessage(): JSONObject = JSONObject().apply {
    put("protocol_version", PROTOCOL_VERSION)
    put("device_id", deviceId)
    put("device_name", deviceName)
    put("tcp_port", DEFAULT_TCP_PORT)
  }

  private fun sendUdpBroadcast(payload: String, discoveryPort: Int) {
    val bytes = payload.toByteArray(Charsets.UTF_8)
    val address = InetAddress.getByName("255.255.255.255")
    DatagramSocket().use { socket ->
      socket.broadcast = true
      socket.send(DatagramPacket(bytes, bytes.size, address, discoveryPort))
    }
  }

  private fun sendPairResponse(targetAddress: String, targetPort: Int, accepted: Boolean) {
    scope?.launch {
      runCatching {
        sendTcpMessage(targetAddress, targetPort, buildPairResponseJson(accepted))
      }.onFailure {
        appendLog("Failed to send pairing response: ${it.message}")
      }
    }
  }

  private fun sendTcpMessage(host: String, port: Int, payload: String) {
    Socket(host, port).use { socket ->
      socket.soTimeout = SOCKET_TIMEOUT_MS
      socket.getOutputStream().use {
        it.write(payload.toByteArray(Charsets.UTF_8))
        it.flush()
      }
      socket.shutdownOutput()
    }
  }

  private fun writeTextToClipboard(text: String) {
    val context = appContext ?: return
    val clipboardManager = context.getSystemService(Context.CLIPBOARD_SERVICE) as ClipboardManager
    clipboardManager.setPrimaryClip(ClipData.newPlainText("LAN Sync", text))
  }

  private suspend fun writeRemoteTextToClipboardWithRetry(text: String): Boolean {
    repeat(REMOTE_CLIPBOARD_WRITE_RETRY_COUNT) { attempt ->
      appendLog("Clipboard write attempt ${attempt + 1}/$REMOTE_CLIPBOARD_WRITE_RETRY_COUNT for remote text: ${previewText(text)}")
      val success = runCatching {
        kotlinx.coroutines.withContext(Dispatchers.Main.immediate) {
          writeTextToClipboard(text)
        }
      }.isSuccess
      if (success) {
        appendLog("Clipboard write attempt ${attempt + 1} succeeded")
        return true
      }
      appendLog("Clipboard write attempt ${attempt + 1} failed")
      if (attempt < REMOTE_CLIPBOARD_WRITE_RETRY_COUNT - 1) {
        delay(REMOTE_CLIPBOARD_WRITE_RETRY_DELAY_MS)
      }
    }
    return false
  }

  private fun consumePendingRemoteClipboardTextLocked(text: String): Boolean {
    val pending = pendingRemoteClipboardText ?: return false
    if (pendingRemoteClipboardExpiresAt <= System.currentTimeMillis()) {
      pendingRemoteClipboardText = null
      pendingRemoteClipboardExpiresAt = 0L
      return false
    }
    if (pending != text) {
      return false
    }
    pendingRemoteClipboardText = null
    pendingRemoteClipboardExpiresAt = 0L
    return true
  }

  private fun upsertDiscoveredDevice(deviceId: String, deviceName: String, address: String, tcpPort: Int) {
    synchronized(stateLock) {
      val trusted = trustedDevices.containsKey(deviceId)
      val existing = discoveredDevices[deviceId]
      if (existing == null) {
        discoveredDevices[deviceId] = DeviceEntry(deviceId, deviceName, address, tcpPort, trusted, System.currentTimeMillis())
      } else {
        existing.deviceName = deviceName
        existing.address = address
        existing.tcpPort = tcpPort
        existing.trusted = trusted
        existing.lastSeen = System.currentTimeMillis()
      }

      trustedDevices[deviceId]?.apply {
        this.deviceName = deviceName
        this.address = address
        this.tcpPort = tcpPort
        this.lastSeen = System.currentTimeMillis()
      }
      notifyObserversLocked()
    }
  }

  private fun upsertSyncedRecordLocked(record: SyncedTextRecord) {
    val existingIndex = syncedRecords.indexOfFirst { it.hash == record.hash }
    if (existingIndex >= 0) {
      syncedRecords.removeAt(existingIndex)
    }
    syncedRecords.add(0, record)
  }

  private fun cleanupStaleDevices() {
    synchronized(stateLock) {
      val now = System.currentTimeMillis()
      val iterator = discoveredDevices.iterator()
      var changed = false
      while (iterator.hasNext()) {
        val entry = iterator.next().value
        val keep = now - entry.lastSeen <= DISCOVERY_STALE_MS || entry.trusted
        if (!keep) {
          iterator.remove()
          changed = true
        }
      }
      if (changed) {
        notifyObserversLocked()
      }
    }
  }

  private fun appendLog(message: String) {
    synchronized(stateLock) {
      appendLogLocked(message)
      notifyObserversLocked()
    }
  }

  private fun appendLogLocked(message: String) {
    Log.d(LOG_TAG, message)
    logs.add(message)
    while (logs.size > MAX_LOG_LINES) {
      logs.removeAt(0)
    }
  }

  private fun notifyObserversLocked() {
    val snapshot = snapshotLocked()
    val currentObservers = observers.toList()
    currentObservers.forEach { observer ->
      mainHandler.post {
        observer(snapshot)
      }
    }
  }

  private fun snapshotLocked(): LanSyncSnapshot = LanSyncSnapshot(
    running = running,
    discoveryEnabled = discoveryEnabled,
    deviceId = deviceId,
    deviceName = deviceName,
    latestPendingReceivedText = latestPendingReceivedText,
    discoveredDevices = discoveredDevices.values.sortedByDescending { it.lastSeen }.map { it.copy() },
    pendingRequests = pendingRequests.values.sortedByDescending { it.requestedAt }.map { it.copy() },
    trustedDevices = trustedDevices.values.sortedBy { it.deviceName.lowercase() }.map { it.copy() },
    syncedRecords = syncedRecords.toList(),
    logs = logs.toList(),
  )

  private fun computeTextHash(text: String): String =
    MessageDigest.getInstance("SHA-256").digest(text.toByteArray(Charsets.UTF_8))
      .joinToString("") { "%02x".format(it) }

  private fun previewText(text: String, maxLength: Int = 32): String {
    val normalized = text.replace('\n', ' ').trim()
    return if (normalized.length <= maxLength) normalized else normalized.take(maxLength) + "..."
  }

  private fun defaultDeviceName(): String {
    val modelName = Build.MODEL?.trim().orEmpty()
    return if (modelName.isNotBlank()) modelName else "Android device"
  }
}
