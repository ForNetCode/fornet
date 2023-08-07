package com.fornet.ui

import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.content.ServiceConnection
import android.os.IBinder
import android.util.Log
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodChannel

class MainActivity: FlutterActivity() {
    companion object {
        var flutterMethodChannel: MethodChannel? = null
        val TAG:String = "com.fornet.ui.Main"
    }
    private val channelTag = "AChannel"
    private var vpnService: ForNetVPNService? = null

    override fun configureFlutterEngine(flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)
        flutterMethodChannel = MethodChannel(
            flutterEngine.dartExecutor.binaryMessenger,
            channelTag
        )
        initFlutterChannel(flutterMethodChannel!!)
    }
    private val serviceConnection = object : ServiceConnection {
        override fun onServiceConnected(name: ComponentName?, service: IBinder?) {
            Log.d(TAG, "onServiceConnected")
            val binder = service as ForNetVPNService.LocalBinder
            //vpnService = binder.getService()
        }

        override fun onServiceDisconnected(name: ComponentName?) {
            Log.d(TAG, "onServiceDisconnected")
            vpnService = null
        }
    }

    private fun initFlutterChannel(flutterMethodChannel: MethodChannel) {
        flutterMethodChannel.setMethodCallHandler{call, result ->

            when(call.method) {
                "init_vpn_service" -> {
                    Intent(activity, ForNetVPNService::class.java).also {
                        bindService(it, serviceConnection , Context.BIND_AUTO_CREATE)
                    }
                    result.success(true)
                    return@setMethodCallHandler

                }
                "stop_vpn_service" -> {
                    Log.d(TAG, "stop vpn service")
                    vpnService?.let {
                        it.destroy()
                        result.success(true)
                    }?: {
                        result.success(false)
                    }
                }
            }
        }
        //TODO:
    }
}
