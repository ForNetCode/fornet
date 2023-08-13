import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_smart_dialog/flutter_smart_dialog.dart';
import 'package:for_net_ui/native/extra_ffi.dart';
import 'package:for_net_ui/native/ffi.dart';
import 'package:for_net_ui/page/mobile/main_page.dart';
import 'package:for_net_ui/state/fornet_lib_manager.dart';
import 'package:path_provider/path_provider.dart';
import 'package:path/path.dart' as p;
import 'package:permission_handler/permission_handler.dart';
import 'package:provider/provider.dart';

void mobileRun() {

  final forNetLibManager = ForNetLibManager();
  runApp(MultiProvider(
    providers: [
      Provider.value(value: forNetLibManager),
      StreamProvider<RuntimeStatus>(create: (_) => forNetLibManager.statusStream, initialData: forNetLibManager.status)
    ],
    child: MaterialApp(
        title: 'ForNet',
        theme: ThemeData.light(useMaterial3: true),
        navigatorObservers: [FlutterSmartDialog.observer],
        builder: FlutterSmartDialog.init(),
        //home: const MobileApp(),
        home: MainPage()
    )
  ));
}

class MobileApp extends StatelessWidget {
  const MobileApp({
    Key? key,
  }) : super(key: key);


  startService(BuildContext context) async {
    if(await Permission.notification.request().isGranted) {
      await AndroidFFI.instance.invokeMethod('init_vpn_service');
    }else {
      ScaffoldMessenger.of(context).showSnackBar(const SnackBar(
        content: Text("Service must be run with notification permission"),
      ));
    }
  }
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('ForNet'),
      ),
      body:Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
        const Text('Hello World'),
        ElevatedButton(onPressed: () async {
          final path = p.join((await getExternalStorageDirectory())!.path, 'config');
          print('path:$path');
          await api.initRuntime(configPath: path,
          workThread: 4, logLevel: kReleaseMode ? 'info' : 'debug');
          print('finish init....');
        }, child: const Text('Test')),
        ElevatedButton(onPressed: ()async {
          await startService(context);
        }, child: const Text('Start Service')),
        ElevatedButton(onPressed: ()async {
          await AndroidFFI.instance.invokeMethod('stop_vpn_service');
        }, child: const Text('Stop Service')),
      ],)
    );
  }
}

