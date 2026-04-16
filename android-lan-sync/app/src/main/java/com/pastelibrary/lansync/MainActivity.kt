package com.pastelibrary.lansync

import android.os.Build
import android.os.Bundle
import android.view.ViewGroup
import android.widget.CompoundButton
import android.widget.Button
import android.widget.EditText
import android.widget.LinearLayout
import android.widget.Switch
import android.widget.TextView
import android.widget.Toast
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.lifecycleScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch
import org.json.JSONObject
import java.io.BufferedInputStream
import java.net.DatagramPacket
import java.net.DatagramSocket
import java.net.InetAddress
import java.net.ServerSocket
import java.net.SocketTimeoutException
import java.net.Socket
import java.util.UUID

private const val PROTOCOL_VERSION: Int = 1
private const val DISCOVERY_INTERVAL_MS: Long = 3000
private const val DISCOVERY_STALE_MS: Long = 12000
private const val SOCKET_TIMEOUT_MS: Int = 1000
private const val DEFAULT_TCP_PORT: Int = 48571
private const val DEFAULT_DISCOVERY_PORT: Int = 48572

private data class DeviceEntry(
  val deviceId: String,
  var deviceName: String,
  var address: String,
  var tcpPort: Int,
  var trusted: Boolean,
  var lastSeen: Long
)

private data class PairingRequest(
  val deviceId: String,
  val deviceName: String,
  val address: String,
  val tcpPort: Int,
  val requestedAt: Long
)

class MainActivity : AppCompatActivity() {
  private lateinit var lanSyncSwitch: Switch
  private lateinit var discoverySwitch: Switch
  private lateinit var deviceNameInput: EditText
  private lateinit var statusView: TextView
  private lateinit var discoveredDevicesContainer: LinearLayout
  private lateinit var pendingRequestsContainer: LinearLayout
  private lateinit var trustedDevicesContainer: LinearLayout
  private lateinit var logView: TextView

  private val deviceId: String = "android-peer-${UUID.randomUUID().toString().take(8)}"
  private val stateLock = Any()
  private val discoveredDevices = mutableMapOf<String, DeviceEntry>()
  private val trustedDevices = mutableMapOf<String, DeviceEntry>()
  private val pendingRequests = mutableMapOf<String, PairingRequest>()
  private val activeDialogDeviceIds = mutableSetOf<String>()

  private var tcpServerJob: Job? = null
  private var udpListenerJob: Job? = null
  private var heartbeatJob: Job? = null

  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
    setContentView(R.layout.activity_main)

    bindViews()
    bindActions()
    refreshStatus()
    refreshDeviceLists()
    if (lanSyncSwitch.isChecked) {
      startService()
    }
    appendLog("设备 ID: $deviceId")
  }

  override fun onDestroy() {
    stopService()
    super.onDestroy()
  }

  private fun bindViews() {
    lanSyncSwitch = findViewById(R.id.lanSyncSwitch)
    discoverySwitch = findViewById(R.id.discoverySwitch)
    deviceNameInput = findViewById(R.id.deviceNameInput)
    statusView = findViewById(R.id.statusView)
    discoveredDevicesContainer = findViewById(R.id.discoveredDevicesContainer)
    pendingRequestsContainer = findViewById(R.id.pendingRequestsContainer)
    trustedDevicesContainer = findViewById(R.id.trustedDevicesContainer)
    logView = findViewById(R.id.logView)
  }

  private fun bindActions() {
    lanSyncSwitch.setOnCheckedChangeListener { _: CompoundButton, isChecked: Boolean ->
      if (isChecked) {
        startService()
      } else {
        stopService()
      }
      refreshStatus()
    }

    discoverySwitch.setOnCheckedChangeListener { _: CompoundButton, _: Boolean ->
      if (lanSyncSwitch.isChecked) {
        restartService()
      }
      refreshStatus()
    }

    deviceNameInput.setOnFocusChangeListener { _, hasFocus ->
      if (!hasFocus) {
        refreshStatus()
      }
    }
  }

  private fun startService() {
    if (tcpServerJob?.isActive == true || udpListenerJob?.isActive == true || heartbeatJob?.isActive == true) {
      appendLog("局域网同步已在运行")
      return
    }

    appendLog("启动局域网同步服务")
    startTcpServer()
    if (discoverySwitch.isChecked) {
      startUdpDiscoveryListener()
      startHeartbeatBroadcaster()
    } else {
      appendLog("自动发现已关闭，仅监听 TCP 配对请求")
    }
    refreshStatus()
  }

  private fun stopService() {
    tcpServerJob?.cancel()
    udpListenerJob?.cancel()
    heartbeatJob?.cancel()
    tcpServerJob = null
    udpListenerJob = null
    heartbeatJob = null
    appendLog("局域网同步已停止")
    refreshStatus()
  }

  private fun restartService() {
    stopService()
    startService()
  }

  private fun startTcpServer() {
    tcpServerJob = lifecycleScope.launch(Dispatchers.IO) {
      try {
        ServerSocket(DEFAULT_TCP_PORT).use { server ->
          server.soTimeout = SOCKET_TIMEOUT_MS
          appendLog("TCP 监听: 0.0.0.0:$DEFAULT_TCP_PORT")
          while (isActive) {
            try {
              val client = server.accept()
              handleIncomingClient(client)
            } catch (_: SocketTimeoutException) {
              // Continue polling.
            }
          }
        }
      } catch (error: Exception) {
        appendLog("TCP 监听失败: ${error.message}")
      }
    }
  }

  private fun startUdpDiscoveryListener() {
    udpListenerJob = lifecycleScope.launch(Dispatchers.IO) {
      try {
        DatagramSocket(DEFAULT_DISCOVERY_PORT).use { socket ->
          socket.soTimeout = SOCKET_TIMEOUT_MS
          appendLog("UDP 发现监听: 0.0.0.0:$DEFAULT_DISCOVERY_PORT")
          while (isActive) {
            val packetBuffer = ByteArray(4096)
            val packet = DatagramPacket(packetBuffer, packetBuffer.size)
            try {
              socket.receive(packet)
              val payload = String(packet.data, 0, packet.length, Charsets.UTF_8).trim()
              val sourceAddress = packet.address?.hostAddress ?: return@launch
              handleIncomingPayload(payload, sourceAddress)
            } catch (_: SocketTimeoutException) {
              cleanupStaleDevices()
            }
          }
        }
      } catch (error: Exception) {
        appendLog("UDP 监听失败: ${error.message}")
      }
    }
  }

  private fun startHeartbeatBroadcaster() {
    heartbeatJob = lifecycleScope.launch(Dispatchers.IO) {
      while (isActive) {
        runCatching {
          val heartbeat = buildHeartbeatJson()
          sendUdpBroadcast(heartbeat, DEFAULT_DISCOVERY_PORT)
          cleanupStaleDevices()
        }.onFailure {
          appendLog("发送 Heartbeat 失败: ${it.message}")
        }
        delay(DISCOVERY_INTERVAL_MS)
      }
    }
  }

  private fun handleIncomingClient(client: Socket) {
    client.use { socket ->
      val bytes = BufferedInputStream(socket.getInputStream()).readBytes()
      val payload = bytes.toString(Charsets.UTF_8)
      val sourceAddress = socket.inetAddress.hostAddress ?: return
      appendLog("收到 TCP 消息: $sourceAddress:${socket.port}")
      handleIncomingPayload(payload, sourceAddress)
    }
  }

  private fun handleIncomingPayload(payload: String, sourceAddress: String) {
    if (payload.isBlank()) return
    val json = runCatching { JSONObject(payload) }.getOrNull() ?: return
    val protocolVersion = json.optInt("protocol_version", -1)
    if (protocolVersion != PROTOCOL_VERSION) return
    val kind = json.optString("kind")
    val remoteDeviceId = json.optString("device_id")
    if (remoteDeviceId.isBlank() || remoteDeviceId == deviceId) return
    val remoteDeviceName = json.optString("device_name").ifBlank { "未知设备" }
    val remoteTcpPort = json.optInt("tcp_port", DEFAULT_TCP_PORT).coerceIn(1, 65535)

    when (kind) {
      "heartbeat" -> {
        upsertDiscoveredDevice(
          deviceId = remoteDeviceId,
          deviceName = remoteDeviceName,
          address = sourceAddress,
          tcpPort = remoteTcpPort
        )
      }

      "pair_request" -> {
        upsertDiscoveredDevice(
          deviceId = remoteDeviceId,
          deviceName = remoteDeviceName,
          address = sourceAddress,
          tcpPort = remoteTcpPort
        )

        val shouldAutoAccept = synchronized(stateLock) {
          trustedDevices.containsKey(remoteDeviceId)
        }
        if (shouldAutoAccept) {
          sendPairResponse(
            targetAddress = sourceAddress,
            targetPort = remoteTcpPort,
            accepted = true
          )
          appendLog("已自动接受来自 $remoteDeviceName 的配对请求")
          return
        }

        val request = PairingRequest(
          deviceId = remoteDeviceId,
          deviceName = remoteDeviceName,
          address = sourceAddress,
          tcpPort = remoteTcpPort,
          requestedAt = System.currentTimeMillis()
        )
        synchronized(stateLock) {
          pendingRequests[remoteDeviceId] = request
        }
        appendLog("收到配对请求: $remoteDeviceName ($sourceAddress:$remoteTcpPort)")
        refreshDeviceLists()
        showPairingDialog(request)
      }

      "pair_response" -> {
        val accepted = json.optBoolean("accepted", false)
        if (accepted) {
          synchronized(stateLock) {
            trustedDevices[remoteDeviceId] = DeviceEntry(
              deviceId = remoteDeviceId,
              deviceName = remoteDeviceName,
              address = sourceAddress,
              tcpPort = remoteTcpPort,
              trusted = true,
              lastSeen = System.currentTimeMillis()
            )
          }
          appendLog("配对成功: $remoteDeviceName")
          runOnUiThread {
            Toast.makeText(this, "已连接到 $remoteDeviceName", Toast.LENGTH_SHORT).show()
          }
        } else {
          appendLog("$remoteDeviceName 拒绝了配对请求")
          runOnUiThread {
            Toast.makeText(this, "$remoteDeviceName 已拒绝", Toast.LENGTH_SHORT).show()
          }
        }
        refreshDeviceLists()
      }
    }
  }

  private fun sendPairRequest(deviceId: String) {
    lifecycleScope.launch(Dispatchers.IO) {
      val target = synchronized(stateLock) { discoveredDevices[deviceId] } ?: return@launch
      runCatching {
        val payload = buildPairRequestJson()
        sendTcpMessage(target.address, target.tcpPort, payload)
        appendLog("已向 ${target.deviceName} 发送配对请求")
      }.onFailure {
        appendLog("发送配对请求失败: ${it.message}")
      }
    }
  }

  private fun approveRequest(deviceId: String) {
    val request = synchronized(stateLock) { pendingRequests.remove(deviceId) } ?: return
    synchronized(stateLock) {
      trustedDevices[request.deviceId] = DeviceEntry(
        deviceId = request.deviceId,
        deviceName = request.deviceName,
        address = request.address,
        tcpPort = request.tcpPort,
        trusted = true,
        lastSeen = System.currentTimeMillis()
      )
      discoveredDevices[request.deviceId]?.trusted = true
    }
    refreshDeviceLists()
    appendLog("已允许 ${request.deviceName} 的连接请求")
    sendPairResponse(request.address, request.tcpPort, true)
  }

  private fun rejectRequest(deviceId: String) {
    val request = synchronized(stateLock) { pendingRequests.remove(deviceId) } ?: return
    refreshDeviceLists()
    appendLog("已拒绝 ${request.deviceName} 的连接请求")
    sendPairResponse(request.address, request.tcpPort, false)
  }

  private fun removeTrustedDevice(deviceId: String) {
    val removed = synchronized(stateLock) {
      val didRemove = trustedDevices.remove(deviceId) != null
      discoveredDevices[deviceId]?.trusted = false
      didRemove
    }
    if (removed) {
      appendLog("已移除信任设备: $deviceId")
      refreshDeviceLists()
    }
  }

  private fun buildHeartbeatJson(): String {
    val message = JSONObject().apply {
      put("kind", "heartbeat")
      put("protocol_version", PROTOCOL_VERSION)
      put("device_id", deviceId)
      put("device_name", normalizedDeviceName())
      put("tcp_port", DEFAULT_TCP_PORT)
    }
    return message.toString()
  }

  private fun buildPairRequestJson(): String {
    val message = JSONObject().apply {
      put("kind", "pair_request")
      put("protocol_version", PROTOCOL_VERSION)
      put("device_id", deviceId)
      put("device_name", normalizedDeviceName())
      put("tcp_port", DEFAULT_TCP_PORT)
    }
    return message.toString()
  }

  private fun buildPairResponseJson(accepted: Boolean): String {
    val message = JSONObject().apply {
      put("kind", "pair_response")
      put("protocol_version", PROTOCOL_VERSION)
      put("device_id", deviceId)
      put("device_name", normalizedDeviceName())
      put("tcp_port", DEFAULT_TCP_PORT)
      put("accepted", accepted)
    }
    return message.toString()
  }

  private fun normalizedDeviceName(): String {
    val raw = deviceNameInput.text.toString().trim()
    val modelName = Build.MODEL?.trim().orEmpty()
    return raw.ifBlank { if (modelName.isNotBlank()) modelName else "Android 设备" }
  }

  private fun sendUdpBroadcast(payload: String, discoveryPort: Int) {
    val bytes = payload.toByteArray(Charsets.UTF_8)
    val address = InetAddress.getByName("255.255.255.255")
    DatagramSocket().use { socket ->
      socket.broadcast = true
      val packet = DatagramPacket(bytes, bytes.size, address, discoveryPort)
      socket.send(packet)
    }
  }

  private fun sendPairResponse(targetAddress: String, targetPort: Int, accepted: Boolean) {
    lifecycleScope.launch(Dispatchers.IO) {
      runCatching {
        val payload = buildPairResponseJson(accepted)
        sendTcpMessage(targetAddress, targetPort, payload)
      }.onFailure {
        appendLog("发送配对响应失败: ${it.message}")
      }
    }
  }

  private fun sendTcpMessage(host: String, port: Int, payload: String) {
    Socket(host, port).use { socket ->
      socket.soTimeout = SOCKET_TIMEOUT_MS
      socket.getOutputStream().use { output ->
        output.write(payload.toByteArray(Charsets.UTF_8))
        output.flush()
      }
    }
  }

  private fun upsertDiscoveredDevice(
    deviceId: String,
    deviceName: String,
    address: String,
    tcpPort: Int
  ) {
    synchronized(stateLock) {
      val trusted = trustedDevices.containsKey(deviceId)
      val existing = discoveredDevices[deviceId]
      if (existing == null) {
        discoveredDevices[deviceId] = DeviceEntry(
          deviceId = deviceId,
          deviceName = deviceName,
          address = address,
          tcpPort = tcpPort,
          trusted = trusted,
          lastSeen = System.currentTimeMillis()
        )
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
    }
    refreshDeviceLists()
  }

  private fun cleanupStaleDevices() {
    val now = System.currentTimeMillis()
    var changed = false
    synchronized(stateLock) {
      val iterator = discoveredDevices.iterator()
      while (iterator.hasNext()) {
        val entry = iterator.next().value
        val keep = now - entry.lastSeen <= DISCOVERY_STALE_MS || entry.trusted
        if (!keep) {
          iterator.remove()
          changed = true
        }
      }
    }
    if (changed) {
      refreshDeviceLists()
    }
  }

  private fun showPairingDialog(request: PairingRequest) {
    runOnUiThread {
      if (isFinishing || isDestroyed) return@runOnUiThread
      val shouldShow = synchronized(stateLock) {
        if (activeDialogDeviceIds.contains(request.deviceId)) {
          false
        } else {
          activeDialogDeviceIds.add(request.deviceId)
          true
        }
      }
      if (!shouldShow) return@runOnUiThread

      AlertDialog.Builder(this)
        .setTitle("连接请求")
        .setMessage("${request.deviceName} 请求与你建立连接\n${request.address}:${request.tcpPort}")
        .setNegativeButton("拒绝") { _, _ ->
          rejectRequest(request.deviceId)
        }
        .setPositiveButton("允许") { _, _ ->
          approveRequest(request.deviceId)
        }
        .setOnDismissListener {
          synchronized(stateLock) {
            activeDialogDeviceIds.remove(request.deviceId)
          }
        }
        .show()
    }
  }

  private fun refreshStatus() {
    runOnUiThread {
      val running = lanSyncSwitch.isChecked
      val discovery = discoverySwitch.isChecked
      val statusText = buildString {
        append("服务状态: ")
        append(if (running) "运行中" else "已停止")
        append("\n")
        append("发现状态: ")
        append(if (discovery) "开启" else "关闭")
        append("\n")
        append("TCP 端口: $DEFAULT_TCP_PORT\n")
        append("发现端口: $DEFAULT_DISCOVERY_PORT\n")
        append("设备 ID: $deviceId")
      }
      statusView.text = statusText
    }
  }

  private fun refreshDeviceLists() {
    runOnUiThread {
      val discoveredSnapshot = synchronized(stateLock) {
        discoveredDevices.values
          .sortedByDescending { it.lastSeen }
          .toList()
      }
      val pendingSnapshot = synchronized(stateLock) {
        pendingRequests.values
          .sortedByDescending { it.requestedAt }
          .toList()
      }
      val trustedSnapshot = synchronized(stateLock) {
        trustedDevices.values
          .sortedBy { it.deviceName.lowercase() }
          .toList()
      }

      renderDiscoveredDevices(discoveredSnapshot)
      renderPendingRequests(pendingSnapshot)
      renderTrustedDevices(trustedSnapshot)
    }
  }

  private fun renderDiscoveredDevices(devices: List<DeviceEntry>) {
    discoveredDevicesContainer.removeAllViews()
    if (devices.isEmpty()) {
      discoveredDevicesContainer.addView(createEmptyView("暂未发现其他设备"))
      return
    }

    devices.forEach { device ->
      val row = createRowContainer()
      val info = createInfoColumn(device.deviceName, "${device.address}:${device.tcpPort}")
      row.addView(info)

      if (device.trusted) {
        val badge = TextView(this).apply {
          text = "已信任"
          setTextColor(0xFF389E0D.toInt())
          textSize = 12f
        }
        row.addView(badge)
      } else {
        val requestButton = Button(this).apply {
          text = "请求连接"
          setOnClickListener {
            sendPairRequest(device.deviceId)
          }
        }
        row.addView(requestButton)
      }
      discoveredDevicesContainer.addView(row)
    }
  }

  private fun renderPendingRequests(requests: List<PairingRequest>) {
    pendingRequestsContainer.removeAllViews()
    if (requests.isEmpty()) {
      pendingRequestsContainer.addView(createEmptyView("暂无待处理请求"))
      return
    }

    requests.forEach { request ->
      val row = createRowContainer()
      val info = createInfoColumn(request.deviceName, "${request.address}:${request.tcpPort}")
      row.addView(info)

      val actions = LinearLayout(this).apply {
        orientation = LinearLayout.HORIZONTAL
      }
      val rejectButton = Button(this).apply {
        text = "拒绝"
        setOnClickListener { rejectRequest(request.deviceId) }
      }
      val approveButton = Button(this).apply {
        text = "允许"
        setOnClickListener { approveRequest(request.deviceId) }
      }
      actions.addView(rejectButton)
      actions.addView(approveButton)
      row.addView(actions)
      pendingRequestsContainer.addView(row)
    }
  }

  private fun renderTrustedDevices(devices: List<DeviceEntry>) {
    trustedDevicesContainer.removeAllViews()
    if (devices.isEmpty()) {
      trustedDevicesContainer.addView(createEmptyView("暂无已信任设备"))
      return
    }

    devices.forEach { device ->
      val row = createRowContainer()
      val info = createInfoColumn(device.deviceName, "${device.address}:${device.tcpPort}")
      row.addView(info)

      val removeButton = Button(this).apply {
        text = "移除"
        setOnClickListener { removeTrustedDevice(device.deviceId) }
      }
      row.addView(removeButton)
      trustedDevicesContainer.addView(row)
    }
  }

  private fun createRowContainer(): LinearLayout {
    return LinearLayout(this).apply {
      orientation = LinearLayout.HORIZONTAL
      setPadding(4, 10, 4, 10)
      val params = LinearLayout.LayoutParams(
        ViewGroup.LayoutParams.MATCH_PARENT,
        ViewGroup.LayoutParams.WRAP_CONTENT
      )
      layoutParams = params
    }
  }

  private fun createInfoColumn(title: String, subtitle: String): LinearLayout {
    return LinearLayout(this).apply {
      orientation = LinearLayout.VERTICAL
      layoutParams = LinearLayout.LayoutParams(
        0,
        ViewGroup.LayoutParams.WRAP_CONTENT,
        1f
      )
      val titleView = TextView(this@MainActivity).apply {
        text = title
        textSize = 14f
        setTextColor(0xFF262626.toInt())
      }
      val subtitleView = TextView(this@MainActivity).apply {
        text = subtitle
        textSize = 12f
        setTextColor(0xFF8C8C8C.toInt())
      }
      addView(titleView)
      addView(subtitleView)
    }
  }

  private fun createEmptyView(content: String): TextView {
    return TextView(this).apply {
      text = content
      textSize = 13f
      setTextColor(0xFF8C8C8C.toInt())
      setPadding(4, 10, 4, 10)
    }
  }

  private fun appendLog(content: String) {
    runOnUiThread {
      val next = buildString {
        append(logView.text)
        if (isNotEmpty()) {
          append('\n')
        }
        append(content)
      }
      logView.text = next
    }
  }
}
