import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:for_net_ui/ffi.dart';
import 'package:path_provider/path_provider.dart';
import 'package:path/path.dart' as p;

void mobileRun() {
  runApp(MaterialApp(
    title: 'ForNet',
    theme: ThemeData.light(useMaterial3: true),
    home: const MobileApp(),
  ));
}

class MobileApp extends StatelessWidget {
  const MobileApp({
    Key? key,
  }) : super(key: key);

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
        }, child: const Text('Test'))
      ],)
    );
  }
}

