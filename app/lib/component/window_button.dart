import 'package:bitsdojo_window/bitsdojo_window.dart';
import 'package:flutter/material.dart';

final closeButtonColors = WindowButtonColors(
    mouseOver: const Color(0xFFD32F2F),
    mouseDown: const Color(0xFFB71C1C),
    iconNormal: Colors.black,
    iconMouseOver: Colors.white);

class WindowButtons extends StatelessWidget {
  const WindowButtons({super.key});

  @override
  Widget build(BuildContext context) {
    return
      Row(
      children: [
        CloseWindowButton(
          //colors: closeButtonColors,
          onPressed: () {
            debugPrint("close click");
            appWindow.hide();
          },
        ),
      ],
    );
  }
}
