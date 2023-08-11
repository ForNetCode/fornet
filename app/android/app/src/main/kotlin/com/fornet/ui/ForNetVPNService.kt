package com.fornet.ui
import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Intent
import android.net.VpnService
import android.os.Binder
import android.os.Build
import android.os.IBinder
import android.os.ParcelFileDescriptor
import android.util.Log
import androidx.core.app.NotificationCompat

class ForNetVPNService: VpnService() {
    companion object {
        const val TAG = "VPNService"
        const val NotificationId = 1
    }
    var fileDescriptor: ParcelFileDescriptor? = null
    lateinit var notificationManager: NotificationManager
    lateinit var notificationBuilder: NotificationCompat.Builder
    private val binder = LocalBinder()

    inner class LocalBinder : Binder() {
        init {
            Log.d(TAG, "LocalBinder init")
        }
        fun getService(): ForNetVPNService = this@ForNetVPNService
    }

    override fun onBind(intent: Intent): IBinder {
        Log.d(TAG, "service onBind")
        return binder
    }
    override fun onCreate() {
        super.onCreate()
        notificationManager = getSystemService(NOTIFICATION_SERVICE) as NotificationManager
        val channelId = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channelId = "ForNet"
            val channel = NotificationChannel(channelId, "ForNet Service", NotificationManager.IMPORTANCE_DEFAULT)
            channel.description = "ForNet Service Channel"
            notificationManager.createNotificationChannel(channel)
            channelId
        } else {
            ""
        }
        notificationBuilder = NotificationCompat.Builder(this, channelId)
        createNotification()
    }

    private fun createNotification() {
        val notification = notificationBuilder
            .setContentText("ForNet Service Running")
            .setContentTitle("ForNet")
            .setSmallIcon(R.mipmap.ic_launcher)
            .setPriority(NotificationCompat.PRIORITY_DEFAULT)
            .setOngoing(true)
            .build()
        Log.d(TAG, "start notification")
        startForeground(NotificationId, notification)
    }
    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        // TODO: check when this be called, we have disabled Always-on Feature
        Log.d(TAG, "this service: ${Thread.currentThread()}, flags:${flags}, startId:${startId}")
        super.onStartCommand(intent, flags, startId)
        return START_NOT_STICKY // don't use sticky (auto restart), the new service (from auto restart) will lose control
    }

    fun setupVPN(ip4FD:Int, ip6FD:Int):Int? {
        if(fileDescriptor!=null) {
            return fileDescriptor?.fd
        }
        val builder = Builder()
        fileDescriptor = builder.establish()
        if(fileDescriptor!=null) {
            protect(ip4FD)
            protect(ip6FD)
        }
        return fileDescriptor?.fd
    }

    override fun onRevoke() {
        super.onRevoke()
        Log.d(TAG, "onRevoke")
    }

    fun destroy() {
        stopSelf()
        Log.d(TAG, "destroy service")
        stopForeground(true)
    }


}