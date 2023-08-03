//import 'package:flutter/cupertino.dart';

import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:bitsdojo_window/bitsdojo_window.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:for_net_ui/native/ffi.dart';
import 'package:for_net_ui/page/login_page.dart';
import 'package:for_net_ui/page/welcome_page.dart';
import 'package:launch_at_startup/launch_at_startup.dart';
import 'package:system_tray/system_tray.dart';
import 'package:package_info_plus/package_info_plus.dart';

final _navKey = GlobalKey<NavigatorState>();

String getTrayImagePath(String imageName) {
  return Platform.isWindows ? 'assets/$imageName.ico' : 'assets/$imageName.png';
}

class PCApp extends StatefulWidget {
  const PCApp({Key? key}) : super(key: key);

  @override
  State<PCApp> createState() => _PCAppState();
}

bool initSuccess = false;

class _PCAppState extends State<PCApp> {
  final AppWindow _appWindow = AppWindow();
  final SystemTray _systemTray = SystemTray();
  final Menu _menu = Menu();
  Timer? _timer;
  bool? _isConnected; 

  @override
  void initState() {
    super.initState();
    initSystemTray();
    initRunTime();
  }

  Future<void> initRunTime() async {
    if (!initSuccess) {
      await api.initRuntime(configPath: await api.getConfigPath(),
          workThread: 4, logLevel: kReleaseMode ? "info" : "debug");
      _timer = Timer(const Duration(seconds: 5), ()async{
        try {
          String data = await api.listNetwork();
          Map<String, dynamic> jsonResult = jsonDecode(data);
          bool isConnected = jsonResult['data'].length() > 0;
          if(isConnected != _isConnected) {
            _isConnected = isConnected;
            buildMenu(isConnected ? MenuItemLabel(label: 'Running',enabled: false):
            MenuItemLabel(label: 'Join Network',onClicked: (_) async {
              if(appWindow.isVisible) {
                // check if now is LoginPage
                return;
              }
              await _appWindow.show();
              if(_navKey.currentContext != null) {
                _navKey.currentState?.pushReplacementNamed(LoginPage.sName);
              } else {
                debugPrint("do not enter LoginPage");
              }
            },));
          }
        }catch(e) {
          // do nothing..
        }
      });
      initSuccess = true;
    }
  }

  @override
  void dispose() {
    _timer?.cancel();
    _timer = null;
    super.dispose();
  }

  Future<void> buildMenu(MenuItem first) async {
    _menu.buildFrom([
      first,
      MenuSeparator(),
      MenuItemCheckbox(
        label: 'Automatic Startup',
        onClicked: (_) async {
          if (await launchAtStartup.isEnabled()) {
            await launchAtStartup.disable();
          } else {
            await launchAtStartup.enable();
          }
        },
        //checked: await launchAtStartup.isEnabled(),
      ),
      MenuItemLabel(
        label: 'Quit',
        onClicked: (_) async {
          //debugPrint("Quit");
          exit(0);
        },
      )
    ]);
    _systemTray.setContextMenu(_menu);
  }

  Future<void> initSystemTray() async {
    // We first init the systray menu and then add the menu entries
    await _systemTray.initSystemTray(iconPath: getTrayImagePath('app_icon'));

    if (Platform.isWindows) {
      // MACOS would show title on tray.
      _systemTray.setTitle("ForNet");
    }
    _systemTray.setToolTip("ForNet UI Client");

    // handle system tray event
    _systemTray.registerSystemTrayEventHandler((eventName) {
      if (eventName == kSystemTrayEventClick ||
          eventName == kSystemTrayEventRightClick) {
        //Platform.isWindows ? _appWindow.show() : _systemTray.popUpContextMenu();
        _systemTray.popUpContextMenu();
      }
    });

    PackageInfo packageInfo = await PackageInfo.fromPlatform();
    // add start
    launchAtStartup.setup(
      appName: packageInfo.appName,
      appPath: Platform.resolvedExecutable,
    );

    await buildMenu(MenuItemLabel(
      label: 'Init',
      enabled: false,
    ));
  }

  @override
  Widget build(BuildContext context) {
    // would not show window.

    return MaterialApp(
      home: const LoginPage(),
      navigatorKey: _navKey,
      debugShowCheckedModeBanner: false,
      routes: _routes,
    );
  }
}

final Map<String, WidgetBuilder> _routes = {
  WelcomePage.sName: (_) => const WelcomePage(),
  LoginPage.sName: (_) => const LoginPage(),
};
