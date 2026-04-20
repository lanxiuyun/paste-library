package com.pastelibrary.lansync

import android.accessibilityservice.AccessibilityService
import android.accessibilityservice.AccessibilityServiceInfo
import android.content.ClipData
import android.content.ClipboardManager
import android.content.ComponentName
import android.content.Context
import android.graphics.PixelFormat
import android.os.Bundle
import android.util.Log
import android.util.TypedValue
import android.view.Gravity
import android.view.View
import android.view.WindowManager
import android.view.accessibility.AccessibilityEvent
import android.view.accessibility.AccessibilityManager
import android.view.accessibility.AccessibilityNodeInfo
import android.widget.Button
import android.widget.LinearLayout
import android.widget.ScrollView
import android.widget.TextView

class ClipboardMonitorAccessibilityService : AccessibilityService() {
  private lateinit var clipboardManager: ClipboardManager
  private lateinit var windowManager: WindowManager

  private var overlayButton: View? = null
  private var overlayPanel: View? = null
  private var currentFocusedNode: AccessibilityNodeInfo? = null
  private var currentFocusedPackage: String? = null

  private val clipboardListener = ClipboardManager.OnPrimaryClipChangedListener {
    val clip = clipboardManager.primaryClip ?: return@OnPrimaryClipChangedListener
    if (clip.itemCount <= 0) {
      return@OnPrimaryClipChangedListener
    }

    val text = clip.getItemAt(0).coerceToText(this)?.toString().orEmpty()
    Log.d("LanSync", "Accessibility clipboard listener fired: preview=${text.trim().take(32)}")
    LanSyncController.submitObservedClipboardText(text)
  }

  override fun onServiceConnected() {
    super.onServiceConnected()
    Log.d("LanSync", "Accessibility service connected")
    clipboardManager = getSystemService(CLIPBOARD_SERVICE) as ClipboardManager
    windowManager = getSystemService(WINDOW_SERVICE) as WindowManager
    clipboardManager.addPrimaryClipChangedListener(clipboardListener)
  }

  override fun onDestroy() {
    currentFocusedNode?.recycle()
    currentFocusedNode = null
    removeOverlayButton()
    removeOverlayPanel()
    if (::clipboardManager.isInitialized) {
      clipboardManager.removePrimaryClipChangedListener(clipboardListener)
    }
    Log.d("LanSync", "Accessibility service destroyed")
    super.onDestroy()
  }

  override fun onAccessibilityEvent(event: AccessibilityEvent?) {
    if (event == null) {
      return
    }

    when (event.eventType) {
      AccessibilityEvent.TYPE_VIEW_FOCUSED,
      AccessibilityEvent.TYPE_VIEW_CLICKED,
      AccessibilityEvent.TYPE_VIEW_TEXT_SELECTION_CHANGED,
      AccessibilityEvent.TYPE_WINDOW_STATE_CHANGED,
      AccessibilityEvent.TYPE_WINDOWS_CHANGED -> {
        updateFocusedInputTarget(event)
      }
    }
  }

  override fun onInterrupt() = Unit

  private fun updateFocusedInputTarget(event: AccessibilityEvent) {
    val eventPackageName = event.packageName?.toString()
    val node = findEditableFocusedNode(event)
    val shouldShowOverlay = node != null && eventPackageName != packageName

    currentFocusedNode?.recycle()
    currentFocusedNode = node
    currentFocusedPackage = eventPackageName

    if (shouldShowOverlay) {
      showOverlayButton()
      Log.d("LanSync", "Editable node focused in package=$eventPackageName")
    } else {
      removeOverlayPanel()
      removeOverlayButton()
    }
  }

  private fun findEditableFocusedNode(event: AccessibilityEvent): AccessibilityNodeInfo? {
    val sourceNode = event.source
    if (sourceNode != null && sourceNode.isEditable) {
      return AccessibilityNodeInfo.obtain(sourceNode)
    }

    val focusedNode = rootInActiveWindow?.findFocus(AccessibilityNodeInfo.FOCUS_INPUT)
    if (focusedNode != null && focusedNode.isEditable) {
      return AccessibilityNodeInfo.obtain(focusedNode)
    }

    return null
  }

  private fun showOverlayButton() {
    if (overlayButton != null) {
      return
    }

    val button = Button(this).apply {
      text = "Paste"
      setOnClickListener { toggleOverlayPanel() }
      setBackgroundColor(0xFFF6FFED.toInt())
      setTextColor(0xFF135200.toInt())
      elevation = dp(4).toFloat()
    }

    windowManager.addView(
      button,
      WindowManager.LayoutParams(
        WindowManager.LayoutParams.WRAP_CONTENT,
        WindowManager.LayoutParams.WRAP_CONTENT,
        WindowManager.LayoutParams.TYPE_ACCESSIBILITY_OVERLAY,
        WindowManager.LayoutParams.FLAG_NOT_FOCUSABLE or WindowManager.LayoutParams.FLAG_LAYOUT_IN_SCREEN,
        PixelFormat.TRANSLUCENT,
      ).apply {
        gravity = Gravity.END or Gravity.CENTER_VERTICAL
        x = dp(16)
      },
    )

    overlayButton = button
  }

  private fun removeOverlayButton() {
    overlayButton?.let { view ->
      windowManager.removeView(view)
    }
    overlayButton = null
  }

  private fun toggleOverlayPanel() {
    if (overlayPanel != null) {
      removeOverlayPanel()
      return
    }
    showOverlayPanel()
  }

  private fun showOverlayPanel() {
    if (overlayPanel != null) {
      return
    }

    val records = LanSyncController.snapshot().syncedRecords.take(12)
    val container = LinearLayout(this).apply {
      orientation = LinearLayout.VERTICAL
      setBackgroundColor(0xFDFDFBF7.toInt())
      setPadding(dp(12), dp(12), dp(12), dp(12))
    }

    container.addView(TextView(this).apply {
      text = "Paste Library"
      setTextColor(0xFF262626.toInt())
      setTextSize(TypedValue.COMPLEX_UNIT_SP, 16f)
    })

    if (records.isEmpty()) {
      container.addView(TextView(this).apply {
        text = "No synced records"
        setTextColor(0xFF8C8C8C.toInt())
        setPadding(0, dp(12), 0, 0)
      })
    } else {
      records.forEach { record ->
        container.addView(createRecordButton(record))
      }
    }

    container.addView(Button(this).apply {
      text = "Close"
      setOnClickListener { removeOverlayPanel() }
    })

    val scrollView = ScrollView(this).apply {
      addView(container)
    }

    windowManager.addView(
      scrollView,
      WindowManager.LayoutParams(
        dp(320),
        WindowManager.LayoutParams.WRAP_CONTENT,
        WindowManager.LayoutParams.TYPE_ACCESSIBILITY_OVERLAY,
        WindowManager.LayoutParams.FLAG_LAYOUT_IN_SCREEN,
        PixelFormat.TRANSLUCENT,
      ).apply {
        gravity = Gravity.END or Gravity.CENTER_VERTICAL
        x = dp(12)
      },
    )

    overlayPanel = scrollView
  }

  private fun removeOverlayPanel() {
    overlayPanel?.let { view ->
      windowManager.removeView(view)
    }
    overlayPanel = null
  }

  private fun createRecordButton(record: SyncedTextRecord): View = Button(this).apply {
    text = buildString {
      append(record.sourceDeviceName)
      append(" | ")
      append(record.text.replace('\n', ' ').take(36))
    }
    gravity = Gravity.START or Gravity.CENTER_VERTICAL
    isAllCaps = false
    setOnClickListener {
      val clipboardWritten = writeTextToClipboard(record.text)
      val inserted = insertTextIntoFocusedNode(record.text)
      Log.d(
        "LanSync",
        "Overlay record click hash=${record.hash.take(12)} inserted=$inserted clipboardWritten=$clipboardWritten package=$currentFocusedPackage",
      )
      if (inserted) {
        removeOverlayPanel()
      }
    }
  }

  private fun insertTextIntoFocusedNode(text: String): Boolean {
    val target = resolveFocusedNode() ?: return false

    val setTextArguments = Bundle().apply {
      putCharSequence(AccessibilityNodeInfo.ACTION_ARGUMENT_SET_TEXT_CHARSEQUENCE, text)
    }
    if (target.performAction(AccessibilityNodeInfo.ACTION_SET_TEXT, setTextArguments)) {
      return true
    }

    val contextText = target.text?.toString().orEmpty()
    val clipboardResult = writeTextToClipboard(text)
    Log.d("LanSync", "Overlay ACTION_SET_TEXT failed, clipboard fallback success=$clipboardResult currentText=${contextText.take(24)}")

    return clipboardResult && target.performAction(AccessibilityNodeInfo.ACTION_PASTE)
  }

  private fun writeTextToClipboard(text: String): Boolean {
    return runCatching {
      clipboardManager.setPrimaryClip(ClipData.newPlainText("LAN Sync Overlay", text))
      true
    }.getOrElse { error ->
      Log.d("LanSync", "Overlay clipboard write failed: ${error.message}")
      false
    }
  }

  private fun resolveFocusedNode(): AccessibilityNodeInfo? {
    currentFocusedNode?.refresh()
    currentFocusedNode?.takeIf { it.isEditable }?.let { return it }

    val focusedNode = rootInActiveWindow?.findFocus(AccessibilityNodeInfo.FOCUS_INPUT)
    if (focusedNode != null && focusedNode.isEditable) {
      currentFocusedNode?.recycle()
      currentFocusedNode = AccessibilityNodeInfo.obtain(focusedNode)
      return currentFocusedNode
    }

    return null
  }

  private fun dp(value: Int): Int =
    TypedValue.applyDimension(TypedValue.COMPLEX_UNIT_DIP, value.toFloat(), resources.displayMetrics).toInt()

  companion object {
    fun isEnabled(context: Context): Boolean {
      val manager = context.getSystemService(Context.ACCESSIBILITY_SERVICE) as AccessibilityManager
      val componentName = ComponentName(context, ClipboardMonitorAccessibilityService::class.java)
      return manager.getEnabledAccessibilityServiceList(AccessibilityServiceInfo.FEEDBACK_ALL_MASK)
        .any { serviceInfo ->
          val service = serviceInfo.resolveInfo.serviceInfo
          service.packageName == componentName.packageName && service.name == componentName.className
        }
    }
  }
}
