import 'package:bitsdojo_window/bitsdojo_window.dart';
import 'package:flutter/material.dart';
import 'package:for_net_ui/component/window_button.dart';
import 'package:for_net_ui/native/ffi.dart';

class LoginPage extends StatefulWidget {
  static const sName = "login";

  const LoginPage({super.key});

  @override
  State createState() => _LoginPage();
}

class _LoginPage extends State<StatefulWidget> {
  String? errorText;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.white,
      body: WindowBorder(
        width: 1,
        color: const Color(0xFF805306),
        child: Column(
          //mainAxisAlignment: MainAxisAlignment.center,
          //crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            WindowTitleBarBox(
                child: Row(
              children: [
                Expanded(child: MoveWindow()),
                const WindowButtons(),
              ],
            )),
            Expanded(
                child: Container(
                    padding: const EdgeInsets.symmetric(horizontal: 30),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      //mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        const SizedBox(
                          height: 10,
                        ),
                        const Text("Invite Code:"),
                        TextFormField(
                          decoration: const InputDecoration(
                            hintText: 'fornet-cli join xxx',
                          ),
                          onFieldSubmitted: (String? inviteCode) async {
                            if (errorText != null) {
                              setState(() {
                                errorText = null;
                              });
                            }
                            if (inviteCode != null && inviteCode.isNotEmpty) {
                              try {
                                await api.joinNetwork(
                                    inviteCode: inviteCode.split('').last);
                              } catch (e) {
                                debugPrint("join network error: $e");
                                setState(() {
                                  errorText =
                                      "join network error:${e.toString()}";
                                });
                              }
                            }
                          },
                        ),
                      ],
                    ))),
          ],
        ),
      ),
    );
  }
}
