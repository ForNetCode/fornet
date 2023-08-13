
import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:for_net_ui/native/ffi.dart';
import 'package:path_provider/path_provider.dart';
import 'package:path/path.dart' as p;

enum RuntimeStatus {
  Unit, Idle, Connected
}

class ForNetLibManager {

  RuntimeStatus _status = RuntimeStatus.Unit;
  RuntimeStatus get status => _status;
  final StreamController<RuntimeStatus> _statusController= StreamController();
  Stream<RuntimeStatus> get statusStream => _statusController.stream;

  static final ForNetLibManager _sharedInstance = ForNetLibManager._();
  factory ForNetLibManager() => _sharedInstance;

  ForNetLibManager._() {
    _initRuntime();
  }

  _initRuntime() async {
    final path = p.join((await getExternalStorageDirectory())!.path, 'config');
    statusStream.listen((status) { _status = status;});
    await api.initRuntime(configPath: path,
        workThread: 4, logLevel: kReleaseMode ? 'info' : 'debug');
    _statusController.add(RuntimeStatus.Idle);
  }

}