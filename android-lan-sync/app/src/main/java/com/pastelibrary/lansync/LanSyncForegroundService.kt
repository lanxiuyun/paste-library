package com.pastelibrary.lansync

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service
import android.content.Context
import android.content.Intent
import android.os.Build
import android.os.IBinder
import androidx.core.app.NotificationCompat
import androidx.core.content.ContextCompat

private const val CHANNEL_ID = "lan_sync_service"
private const val CHANNEL_NAME = "LAN Sync"
private const val NOTIFICATION_ID = 1001

class LanSyncForegroundService : Service() {
  private val snapshotObserver: (LanSyncSnapshot) -> Unit = {
    updateNotification(it)
  }

  override fun onBind(intent: Intent?): IBinder? = null

  override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
    when (intent?.action) {
      ACTION_STOP -> {
        LanSyncController.stop()
        stopForeground(STOP_FOREGROUND_REMOVE)
        stopSelf()
      }

      ACTION_COPY_LATEST -> {
        LanSyncController.copyLatestReceivedTextToClipboard()
      }

      else -> {
        val deviceName = intent?.getStringExtra(EXTRA_DEVICE_NAME).orEmpty()
        val discoveryEnabled = intent?.getBooleanExtra(EXTRA_DISCOVERY_ENABLED, true) ?: true
        LanSyncController.setDeviceName(deviceName)
        LanSyncController.setDiscoveryEnabled(discoveryEnabled)
        startForeground(NOTIFICATION_ID, buildNotification(LanSyncController.snapshot()))
        LanSyncController.removeObserver(snapshotObserver)
        LanSyncController.addObserver(snapshotObserver)
        LanSyncController.start(applicationContext)
      }
    }

    return START_STICKY
  }

  override fun onDestroy() {
    LanSyncController.removeObserver(snapshotObserver)
    LanSyncController.stop()
    super.onDestroy()
  }

  private fun buildNotification(snapshot: LanSyncSnapshot): Notification {
    ensureNotificationChannel()

    val contentIntent = PendingIntent.getActivity(
      this,
      0,
      Intent(this, MainActivity::class.java).apply {
        flags = Intent.FLAG_ACTIVITY_SINGLE_TOP or Intent.FLAG_ACTIVITY_CLEAR_TOP
      },
      PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE,
    )

    val builder = NotificationCompat.Builder(this, CHANNEL_ID)
      .setSmallIcon(android.R.drawable.ic_menu_share)
      .setContentTitle("LAN Sync is running")
      .setContentText(
        snapshot.latestPendingReceivedText?.let { "New text received. Tap Copy Now." }
          ?: "Background receiving is active",
      )
      .setStyle(
        NotificationCompat.BigTextStyle().bigText(
          snapshot.latestPendingReceivedText?.let { "Received: $it" }
            ?: "Background receiving is active",
        ),
      )
      .setOngoing(true)
      .setContentIntent(contentIntent)

    if (!snapshot.latestPendingReceivedText.isNullOrBlank()) {
      val copyIntent = PendingIntent.getService(
        this,
        1,
        Intent(this, LanSyncForegroundService::class.java).apply {
          action = ACTION_COPY_LATEST
        },
        PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE,
      )
      builder.addAction(
        android.R.drawable.ic_menu_save,
        "Copy Now",
        copyIntent,
      )
      builder.setAutoCancel(false)
    }

    return builder.build()
  }

  private fun updateNotification(snapshot: LanSyncSnapshot) {
    val manager = getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
    manager.notify(NOTIFICATION_ID, buildNotification(snapshot))
  }

  private fun ensureNotificationChannel() {
    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.O) {
      return
    }

    val manager = getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
    if (manager.getNotificationChannel(CHANNEL_ID) != null) {
      return
    }

    manager.createNotificationChannel(
      NotificationChannel(
        CHANNEL_ID,
        CHANNEL_NAME,
        NotificationManager.IMPORTANCE_LOW,
      ),
    )
  }

  companion object {
    private const val ACTION_START_OR_UPDATE = "com.pastelibrary.lansync.action.START_OR_UPDATE"
    private const val ACTION_STOP = "com.pastelibrary.lansync.action.STOP"
    private const val ACTION_COPY_LATEST = "com.pastelibrary.lansync.action.COPY_LATEST"
    private const val EXTRA_DEVICE_NAME = "device_name"
    private const val EXTRA_DISCOVERY_ENABLED = "discovery_enabled"

    fun startOrUpdate(context: Context, deviceName: String, discoveryEnabled: Boolean) {
      val intent = Intent(context, LanSyncForegroundService::class.java).apply {
        action = ACTION_START_OR_UPDATE
        putExtra(EXTRA_DEVICE_NAME, deviceName)
        putExtra(EXTRA_DISCOVERY_ENABLED, discoveryEnabled)
      }
      ContextCompat.startForegroundService(context, intent)
    }

    fun stop(context: Context) {
      val intent = Intent(context, LanSyncForegroundService::class.java).apply {
        action = ACTION_STOP
      }
      context.startService(intent)
    }
  }
}
