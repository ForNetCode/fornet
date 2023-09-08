import 'dart:io';

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


