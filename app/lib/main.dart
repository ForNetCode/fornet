import 'dart:io';

import 'package:bitsdojo_window/bitsdojo_window.dart';
import 'package:flutter/material.dart';
import 'package:for_net_ui/mobile_app.dart';
import 'package:for_net_ui/pc_app.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  if(Platform.isAndroid || Platform.isIOS) {
    mobileRun();
  }else {
    pcRun();
  }
}

void pcRun() {
  runApp(const PCApp());
  doWhenWindowReady(() {
    final win = appWindow;
    const initialSize = Size(400, 160);
    win.minSize = initialSize;
    win.size = initialSize;
    win.alignment = Alignment.center;
    //win.show();
    //win.hide();
  });
}

