package com.pastelibrary.lansync

import android.os.Bundle
import android.widget.Button
import android.widget.EditText
import android.widget.TextView
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
import java.net.Socket
import java.time.Instant
import java.util.UUID

private const val PROTOCOL_VERSION: Int = 1

class MainActivity : AppCompatActivity() {
  private lateinit var targetHostInput: EditText
  private lateinit var discoveryPortInput: EditText
  private lateinit var targetTcpPortInput: EditText
  private lateinit var listenPortInput: EditText
  private lateinit var deviceNameInput: EditText
  private lateinit var textInput: EditText
  private lateinit var logView: TextView

  private lateinit var startListenButton: Button
  private lateinit var stopListenButton: Button
  private lateinit var heartbeatButton: Button
  private lateinit var pairRequestButton: Button
  private lateinit var syncTextButton: Button
  private lateinit var runAllButton: Button

  private val deviceId: String = "android-peer-${UUID.randomUUID().toString().take(8)}"
  private var listenJob: Job? = null

  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
    setContentView(R.layout.activity_main)

    bindViews()
    bindActions()
    appendLog("设备 ID: $deviceId")
  }

  override fun onDestroy() {
    listenJob?.cancel()
    super.onDestroy()
  }

  private fun bindViews() {
    targetHostInput = findViewById(R.id.targetHostInput)
    discoveryPortInput = findViewById(R.id.discoveryPortInput)
    targetTcpPortInput = findViewById(R.id.targetTcpPortInput)
    listenPortInput = findViewById(R.id.listenPortInput)
    deviceNameInput = findViewById(R.id.deviceNameInput)
    textInput = findViewById(R.id.textInput)
    logView = findViewById(R.id.logView)

    startListenButton = findViewById(R.id.startListenButton)
    stopListenButton = findViewById(R.id.stopListenButton)
    heartbeatButton = findViewById(R.id.heartbeatButton)
    pairRequestButton = findViewById(R.id.pairRequestButton)
    syncTextButton = findViewById(R.id.syncTextButton)
    runAllButton = findViewById(R.id.runAllButton)
  }

  private fun bindActions() {
    startListenButton.setOnClickListener {
      startPairResponseListener()
    }
    stopListenButton.setOnClickListener {
      stopPairResponseListener()
    }
    heartbeatButton.setOnClickListener {
      sendHeartbeat()
    }
    pairRequestButton.setOnClickListener {
      sendPairRequest()
    }
    syncTextButton.setOnClickListener {
      sendSyncText()
    }
    runAllButton.setOnClickListener {
      runAllFlow()
    }
  }

  private fun startPairResponseListener() {
    val listenPort = parsePort(listenPortInput.text.toString(), 48581)
    if (listenJob?.isActive == true) {
      appendLog("监听已在运行，端口: $listenPort")
      return
    }

    listenJob = lifecycleScope.launch(Dispatchers.IO) {
      try {
        ServerSocket(listenPort).use { server ->
          appendLog("开始监听 PairResponse: 0.0.0.0:$listenPort")
          while (isActive) {
            val client = server.accept()
            handleIncomingClient(client)
          }
        }
      } catch (error: Exception) {
        appendLog("监听失败: ${error.message}")
      }
    }
  }

  private fun stopPairResponseListener() {
    listenJob?.cancel()
    listenJob = null
    appendLog("已停止监听 PairResponse")
  }

  private fun handleIncomingClient(client: Socket) {
    client.use { socket ->
      val bytes = BufferedInputStream(socket.getInputStream()).readBytes()
      val payload = bytes.toString(Charsets.UTF_8)
      appendLog("收到消息 ${socket.inetAddress.hostAddress}:${socket.port}")
      appendLog(payload)
    }
  }

  private fun sendHeartbeat() {
    lifecycleScope.launch(Dispatchers.IO) {
      runCatching {
        val payload = buildHeartbeatJson()
        val discoveryPort = parsePort(discoveryPortInput.text.toString(), 48572)
        sendUdpBroadcast(payload, discoveryPort)
        appendLog("Heartbeat 已发送到 255.255.255.255:$discoveryPort")
      }.onFailure {
        appendLog("Heartbeat 发送失败: ${it.message}")
      }
    }
  }

  private fun sendPairRequest() {
    lifecycleScope.launch(Dispatchers.IO) {
      runCatching {
        val payload = buildPairRequestJson()
        sendTcpToTarget(payload)
        appendLog("PairRequest 已发送")
      }.onFailure {
        appendLog("PairRequest 发送失败: ${it.message}")
      }
    }
  }

  private fun sendSyncText() {
    lifecycleScope.launch(Dispatchers.IO) {
      runCatching {
        val payload = buildSyncTextJson()
        sendTcpToTarget(payload)
        appendLog("SyncText 已发送")
      }.onFailure {
        appendLog("SyncText 发送失败: ${it.message}")
      }
    }
  }

  private fun runAllFlow() {
    lifecycleScope.launch(Dispatchers.IO) {
      runCatching {
        sendUdpBroadcast(buildHeartbeatJson(), parsePort(discoveryPortInput.text.toString(), 48572))
        appendLog("All: Heartbeat 已发送")
        delay(500)
        sendTcpToTarget(buildPairRequestJson())
        appendLog("All: PairRequest 已发送")
        delay(500)
        sendTcpToTarget(buildSyncTextJson())
        appendLog("All: SyncText 已发送")
      }.onFailure {
        appendLog("All 流程失败: ${it.message}")
      }
    }
  }

  private fun buildHeartbeatJson(): String {
    val message = JSONObject().apply {
      put("kind", "heartbeat")
      put("protocol_version", PROTOCOL_VERSION)
      put("device_id", deviceId)
      put("device_name", normalizedDeviceName())
      put("tcp_port", parsePort(listenPortInput.text.toString(), 48581))
    }
    return message.toString()
  }

  private fun buildPairRequestJson(): String {
    val message = JSONObject().apply {
      put("kind", "pair_request")
      put("protocol_version", PROTOCOL_VERSION)
      put("device_id", deviceId)
      put("device_name", normalizedDeviceName())
      put("tcp_port", parsePort(listenPortInput.text.toString(), 48581))
    }
    return message.toString()
  }

  private fun buildSyncTextJson(): String {
    val message = JSONObject().apply {
      put("kind", "sync_text")
      put("protocol_version", PROTOCOL_VERSION)
      put("device_id", deviceId)
      put("device_name", normalizedDeviceName())
      put("tcp_port", parsePort(listenPortInput.text.toString(), 48581))
      put("message_id", "$deviceId-${UUID.randomUUID()}")
      put("created_at", Instant.now().toString())
      put("text", textInput.text.toString())
    }
    return message.toString()
  }

  private fun normalizedDeviceName(): String {
    val raw = deviceNameInput.text.toString().trim()
    return raw.ifBlank { "Android LAN Peer" }
  }

  private fun sendUdpBroadcast(payload: String, discoveryPort: Int) {
    val bytes = payload.toByteArray(Charsets.UTF_8)
    val address = InetAddress.getByName("255.255.255.255")
    DatagramSocket().use { socket ->
      socket.broadcast = true
      val packet = DatagramPacket(bytes, bytes.size, address, discoveryPort)
      socket.send(packet)
    }
    appendLog(payload)
  }

  private fun sendTcpToTarget(payload: String) {
    val host = targetHostInput.text.toString().trim().ifBlank { "127.0.0.1" }
    val port = parsePort(targetTcpPortInput.text.toString(), 48571)
    Socket(host, port).use { socket ->
      socket.getOutputStream().use { output ->
        output.write(payload.toByteArray(Charsets.UTF_8))
        output.flush()
      }
    }
    appendLog("发送到 $host:$port")
    appendLog(payload)
  }

  private fun parsePort(raw: String, fallback: Int): Int {
    val parsed = raw.trim().toIntOrNull() ?: fallback
    return parsed.coerceIn(1, 65535)
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
