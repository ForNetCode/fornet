import 'package:flutter/material.dart';

void mobileRun() {
  runApp(const MaterialApp(
    title: 'ForNet',
    home: MobileApp(),
  ));
}

class MobileApp extends StatelessWidget {
  const MobileApp({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return const Center(child: Text('Hello World'),);
  }
}

