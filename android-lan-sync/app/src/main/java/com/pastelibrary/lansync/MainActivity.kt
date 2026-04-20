package com.pastelibrary.lansync

import android.Manifest
import android.content.ClipboardManager
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import android.provider.Settings
import android.util.Log
import android.view.ViewGroup
import android.widget.Button
import android.widget.CompoundButton
import android.widget.EditText
import android.widget.LinearLayout
import android.widget.Switch
import android.widget.TextView
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat

private const val NOTIFICATION_PERMISSION_REQUEST_CODE = 1001

class MainActivity : AppCompatActivity() {
  private lateinit var clipboardManager: ClipboardManager
  private lateinit var lanSyncSwitch: Switch
  private lateinit var discoverySwitch: Switch
  private lateinit var deviceNameInput: EditText
  private lateinit var statusView: TextView
  private lateinit var accessibilityStatusView: TextView
  private lateinit var openAccessibilitySettingsButton: Button
  private lateinit var syncedRecordsContainer: LinearLayout
  private lateinit var discoveredDevicesContainer: LinearLayout
  private lateinit var pendingRequestsContainer: LinearLayout
  private lateinit var trustedDevicesContainer: LinearLayout
  private lateinit var logView: TextView

  private var suppressClipboardListener = false

  private val clipboardListener = ClipboardManager.OnPrimaryClipChangedListener {
    if (suppressClipboardListener) {
      return@OnPrimaryClipChangedListener
    }
    handlePrimaryClipChanged()
  }

  private val snapshotObserver: (LanSyncSnapshot) -> Unit = { snapshot ->
    runOnUiThread {
      suppressClipboardListener = true
      lanSyncSwitch.isChecked = snapshot.running
      discoverySwitch.isChecked = snapshot.discoveryEnabled
      if (deviceNameInput.text.toString() != snapshot.deviceName) {
        deviceNameInput.setText(snapshot.deviceName)
        deviceNameInput.setSelection(deviceNameInput.text.length)
      }
      suppressClipboardListener = false
      renderSnapshot(snapshot)
    }
  }

  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
    setContentView(R.layout.activity_main)
    clipboardManager = getSystemService(CLIPBOARD_SERVICE) as ClipboardManager
    bindViews()
    bindActions()

    val initialSnapshot = LanSyncController.snapshot()
    if (!initialSnapshot.running && deviceNameInput.text.isNullOrBlank()) {
      deviceNameInput.setText(defaultDeviceName())
    }
    renderSnapshot(initialSnapshot)
  }

  override fun onStart() {
    super.onStart()
    Log.d("LanSync", "MainActivity onStart")
    LanSyncController.setForegroundUiAttached(true)
    LanSyncController.addObserver(snapshotObserver)
    clipboardManager.addPrimaryClipChangedListener(clipboardListener)
    refreshAccessibilityUi()
  }

  override fun onStop() {
    Log.d("LanSync", "MainActivity onStop")
    clipboardManager.removePrimaryClipChangedListener(clipboardListener)
    LanSyncController.removeObserver(snapshotObserver)
    LanSyncController.setForegroundUiAttached(false)
    super.onStop()
  }

  private fun bindViews() {
    lanSyncSwitch = findViewById(R.id.lanSyncSwitch)
    discoverySwitch = findViewById(R.id.discoverySwitch)
    deviceNameInput = findViewById(R.id.deviceNameInput)
    statusView = findViewById(R.id.statusView)
    accessibilityStatusView = findViewById(R.id.accessibilityStatusView)
    openAccessibilitySettingsButton = findViewById(R.id.openAccessibilitySettingsButton)
    syncedRecordsContainer = findViewById(R.id.syncedRecordsContainer)
    discoveredDevicesContainer = findViewById(R.id.discoveredDevicesContainer)
    pendingRequestsContainer = findViewById(R.id.pendingRequestsContainer)
    trustedDevicesContainer = findViewById(R.id.trustedDevicesContainer)
    logView = findViewById(R.id.logView)
  }

  private fun bindActions() {
    lanSyncSwitch.setOnCheckedChangeListener { _: CompoundButton, checked: Boolean ->
      if (suppressClipboardListener) {
        return@setOnCheckedChangeListener
      }
      if (checked) {
        ensureNotificationPermission()
        LanSyncForegroundService.startOrUpdate(
          this,
          normalizedDeviceName(),
          discoverySwitch.isChecked,
        )
      } else {
        LanSyncForegroundService.stop(this)
      }
    }

    discoverySwitch.setOnCheckedChangeListener { _: CompoundButton, _: Boolean ->
      if (suppressClipboardListener) {
        return@setOnCheckedChangeListener
      }
      if (lanSyncSwitch.isChecked) {
        ensureNotificationPermission()
        LanSyncForegroundService.startOrUpdate(
          this,
          normalizedDeviceName(),
          discoverySwitch.isChecked,
        )
      }
    }

    deviceNameInput.setOnFocusChangeListener { _, hasFocus ->
      if (!hasFocus && lanSyncSwitch.isChecked) {
        ensureNotificationPermission()
        LanSyncForegroundService.startOrUpdate(
          this,
          normalizedDeviceName(),
          discoverySwitch.isChecked,
        )
      }
    }

    openAccessibilitySettingsButton.setOnClickListener {
      startActivity(Intent(Settings.ACTION_ACCESSIBILITY_SETTINGS))
    }
  }

  private fun handlePrimaryClipChanged() {
    if (!lanSyncSwitch.isChecked) {
      Log.d("LanSync", "MainActivity clipboard listener ignored change because LAN sync is disabled")
      return
    }

    val clip = clipboardManager.primaryClip ?: return
    if (clip.itemCount <= 0) {
      return
    }

    val text = clip.getItemAt(0).coerceToText(this)?.toString()?.trim().orEmpty()
    if (text.isBlank()) {
      Log.d("LanSync", "MainActivity clipboard listener ignored blank clipboard text")
      return
    }

    Log.d("LanSync", "MainActivity clipboard listener accepted text: preview=${text.take(32)}")
    LanSyncController.setDeviceName(normalizedDeviceName())
    LanSyncController.submitObservedClipboardText(text)
  }

  private fun renderSnapshot(snapshot: LanSyncSnapshot) {
    val accessibilityEnabled = ClipboardMonitorAccessibilityService.isEnabled(this)
    statusView.text = buildString {
      append("Service: ")
      append(if (snapshot.running) "running" else "stopped")
      append('\n')
      append("Discovery: ")
      append(if (snapshot.discoveryEnabled) "enabled" else "disabled")
      append("\nTCP port: 48571\n")
      append("Discovery port: 48572\n")
      append("Trusted devices: ${snapshot.trustedDevices.size}\n")
      append("Synced records: ${snapshot.syncedRecords.size}\n")
      append("Input overlay paste: ")
      append(if (accessibilityEnabled) "enabled" else "requires accessibility service")
      append('\n')
      append("Device ID: ${snapshot.deviceId}")
    }

    refreshAccessibilityUi()
    renderSyncedRecords(snapshot.syncedRecords)
    renderDiscoveredDevices(snapshot.discoveredDevices)
    renderPendingRequests(snapshot.pendingRequests)
    renderTrustedDevices(snapshot.trustedDevices)
    logView.text = snapshot.logs.joinToString(separator = "\n")
  }

  private fun renderSyncedRecords(records: List<SyncedTextRecord>) {
    syncedRecordsContainer.removeAllViews()
    if (records.isEmpty()) {
      syncedRecordsContainer.addView(createEmptyView("No synced text yet"))
      return
    }

    records.forEach { record ->
      val row = createRowContainer()
      row.addView(createRecordInfoColumn(record))
      row.addView(TextView(this).apply {
        text = if (record.direction == DIRECTION_RECEIVED) "RECV" else "SENT"
        textSize = 11f
        setTextColor(if (record.direction == DIRECTION_RECEIVED) 0xFF1677FF.toInt() else 0xFF389E0D.toInt())
      })
      syncedRecordsContainer.addView(row)
    }
  }

  private fun renderDiscoveredDevices(devices: List<DeviceEntry>) {
    discoveredDevicesContainer.removeAllViews()
    if (devices.isEmpty()) {
      discoveredDevicesContainer.addView(createEmptyView("No devices discovered"))
      return
    }

    devices.forEach { device ->
      val row = createRowContainer()
      row.addView(createInfoColumn(device.deviceName, "${device.address}:${device.tcpPort}"))
      if (device.trusted) {
        row.addView(TextView(this).apply {
          text = "Trusted"
          textSize = 12f
          setTextColor(0xFF389E0D.toInt())
        })
      } else {
        row.addView(Button(this).apply {
          text = "Pair"
          setOnClickListener { LanSyncController.requestPairing(device.deviceId) }
        })
      }
      discoveredDevicesContainer.addView(row)
    }
  }

  private fun renderPendingRequests(requests: List<PairingRequest>) {
    pendingRequestsContainer.removeAllViews()
    if (requests.isEmpty()) {
      pendingRequestsContainer.addView(createEmptyView("No pending requests"))
      return
    }

    requests.forEach { request ->
      val row = createRowContainer()
      row.addView(createInfoColumn(request.deviceName, "${request.address}:${request.tcpPort}"))
      row.addView(LinearLayout(this).apply {
        orientation = LinearLayout.HORIZONTAL
        addView(Button(this@MainActivity).apply {
          text = "Reject"
          setOnClickListener { LanSyncController.rejectRequest(request.deviceId) }
        })
        addView(Button(this@MainActivity).apply {
          text = "Allow"
          setOnClickListener { showPairingConfirm(request) }
        })
      })
      pendingRequestsContainer.addView(row)
    }
  }

  private fun renderTrustedDevices(devices: List<DeviceEntry>) {
    trustedDevicesContainer.removeAllViews()
    if (devices.isEmpty()) {
      trustedDevicesContainer.addView(createEmptyView("No trusted devices"))
      return
    }

    devices.forEach { device ->
      val row = createRowContainer()
      row.addView(createInfoColumn(device.deviceName, "${device.address}:${device.tcpPort}"))
      row.addView(Button(this).apply {
        text = "Remove"
        setOnClickListener { LanSyncController.removeTrustedDevice(device.deviceId) }
      })
      trustedDevicesContainer.addView(row)
    }
  }

  private fun showPairingConfirm(request: PairingRequest) {
    AlertDialog.Builder(this)
      .setTitle("Pairing request")
      .setMessage("${request.deviceName} wants to pair with this device\n${request.address}:${request.tcpPort}")
      .setNegativeButton("Cancel", null)
      .setPositiveButton("Allow") { _, _ ->
        LanSyncController.approveRequest(request.deviceId)
      }
      .show()
  }

  private fun createRowContainer(): LinearLayout = LinearLayout(this).apply {
    orientation = LinearLayout.HORIZONTAL
    setPadding(4, 10, 4, 10)
    layoutParams = LinearLayout.LayoutParams(
      ViewGroup.LayoutParams.MATCH_PARENT,
      ViewGroup.LayoutParams.WRAP_CONTENT,
    )
  }

  private fun createInfoColumn(title: String, subtitle: String): LinearLayout = LinearLayout(this).apply {
    orientation = LinearLayout.VERTICAL
    layoutParams = LinearLayout.LayoutParams(
      0,
      ViewGroup.LayoutParams.WRAP_CONTENT,
      1f,
    )
    addView(TextView(this@MainActivity).apply {
      text = title
      textSize = 14f
      setTextColor(0xFF262626.toInt())
    })
    addView(TextView(this@MainActivity).apply {
      text = subtitle
      textSize = 12f
      setTextColor(0xFF8C8C8C.toInt())
    })
  }

  private fun createRecordInfoColumn(record: SyncedTextRecord): LinearLayout = LinearLayout(this).apply {
    orientation = LinearLayout.VERTICAL
    layoutParams = LinearLayout.LayoutParams(
      0,
      ViewGroup.LayoutParams.WRAP_CONTENT,
      1f,
    )
    addView(TextView(this@MainActivity).apply {
      text = record.sourceDeviceName
      textSize = 14f
      setTextColor(0xFF262626.toInt())
    })
    addView(TextView(this@MainActivity).apply {
      text = "${record.createdAt} · ${record.hash.take(12)}"
      textSize = 11f
      setTextColor(0xFF8C8C8C.toInt())
    })
    addView(TextView(this@MainActivity).apply {
      text = record.text
      textSize = 13f
      setTextColor(0xFF434343.toInt())
    })
  }

  private fun createEmptyView(content: String): TextView = TextView(this).apply {
    text = content
    textSize = 13f
    setTextColor(0xFF8C8C8C.toInt())
    setPadding(4, 10, 4, 10)
  }

  private fun normalizedDeviceName(): String {
    val raw = deviceNameInput.text.toString().trim()
    return raw.ifBlank { defaultDeviceName() }
  }

  private fun defaultDeviceName(): String {
    val modelName = Build.MODEL?.trim().orEmpty()
    return if (modelName.isNotBlank()) modelName else "Android device"
  }

  private fun refreshAccessibilityUi() {
    val enabled = ClipboardMonitorAccessibilityService.isEnabled(this)
    accessibilityStatusView.text =
      if (enabled) {
        "Floating paste entry will appear when an input box gains focus."
      } else {
        "Enable accessibility service to show the floating paste entry on focused input boxes."
      }
    openAccessibilitySettingsButton.text =
      if (enabled) "Open accessibility settings" else "Enable floating paste entry"
  }

  private fun ensureNotificationPermission() {
    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) {
      return
    }

    if (ContextCompat.checkSelfPermission(this, Manifest.permission.POST_NOTIFICATIONS) == PackageManager.PERMISSION_GRANTED) {
      return
    }

    ActivityCompat.requestPermissions(
      this,
      arrayOf(Manifest.permission.POST_NOTIFICATIONS),
      NOTIFICATION_PERMISSION_REQUEST_CODE,
    )
  }
}
