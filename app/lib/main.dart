import 'package:bitsdojo_window/bitsdojo_window.dart';
import 'package:flutter/material.dart';
import 'package:for_net_ui/pc_app.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  pcRun();
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

void mobileRun() {

}