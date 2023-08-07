package com.fornet.ui
import android.net.VpnService
import android.os.Binder
import android.os.ParcelFileDescriptor
import android.util.Log

class ForNetVPNService: VpnService() {
    companion object {
        val TAG = "com.fornet.ui.VPNService"
    }
    //lateinit var fileDescriptor: ParcelFileDescriptor

    private val binder = LocalBinder()

    inner class LocalBinder : Binder() {
        init {
            Log.d(ForNetVPNService.TAG, "LocalBinder init")
        }

        fun getService(): ForNetVPNService = this@ForNetVPNService
    }
    override fun onCreate() {
        super.onCreate()

    }

    fun setupVPN() {
        val builder = Builder()
        //fileDescriptor = builder.establish()
    }

    fun destroy() {
        Log.d(ForNetVPNService.TAG, "destroy service")

    }


}