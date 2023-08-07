package com.fornet.ui
import android.app.NotificationManager
import android.content.Intent
import android.net.VpnService
import android.os.Binder
import android.os.IBinder
import android.os.ParcelFileDescriptor
import android.util.Log

class ForNetVPNService: VpnService() {
    companion object {
        val TAG = "VPNService"
    }
    //lateinit var fileDescriptor: ParcelFileDescriptor
    lateinit var notificationManager: NotificationManager
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
        initNotification()
        stopForeground(true)
    }
    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        Log.d(TAG, "this service: ${Thread.currentThread()}")
        super.onStartCommand(intent, flags, startId)
        return START_NOT_STICKY // don't use sticky (auto restart), the new service (from auto restart) will lose control
    }
    private fun initNotification() {
        //TODO: add it
    }

    fun setupVPN() {
        //val builder = Builder()
        //fileDescriptor = builder.establish()
    }

    fun destroy() {
        stopSelf()
        Log.d(TAG, "destroy service")
    }


}