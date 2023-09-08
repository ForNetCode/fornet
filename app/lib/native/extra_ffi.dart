import 'package:flutter/services.dart';

// This is for Android/iOS where Rust can not reach

class AndroidFFI {
  AndroidFFI._();

  static final AndroidFFI instance = AndroidFFI._();
  final _channel = const MethodChannel('AChannel');

  invokeMethod(String method, [dynamic arguments]) async {
    // if (!Platform.isAndroid) return Future<bool>(() => false);
    return await _channel.invokeMethod(method, arguments);
  }
}