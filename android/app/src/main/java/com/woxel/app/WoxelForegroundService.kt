package com.woxel.app

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.Service
import android.content.Intent
import android.os.Build
import android.os.IBinder

class WoxelForegroundService : Service() {
    override fun onBind(intent: Intent?): IBinder? = null

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        val manager = getSystemService(NotificationManager::class.java)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            manager.createNotificationChannel(NotificationChannel("woxel_tasks", "Woxel Tasks", NotificationManager.IMPORTANCE_LOW))
        }
        val notification = Notification.Builder(this, "woxel_tasks")
            .setContentTitle("Woxel")
            .setContentText("File operation in progress")
            .setSmallIcon(android.R.drawable.stat_sys_download)
            .build()
        startForeground(1001, notification)
        return START_STICKY
    }
}
