import 'package:flutter/material.dart';
import 'package:flutter_smart_dialog/flutter_smart_dialog.dart';
import 'package:for_net_ui/native/ffi.dart';
import 'package:for_net_ui/page/mobile/join_network_page.dart';
import 'package:for_net_ui/state/fornet_lib_manager.dart';
import 'package:logger/logger.dart';
import 'package:provider/provider.dart';

class MainPage extends StatelessWidget {
  final log = Logger();
  MainPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Consumer<RuntimeStatus>(
      builder:(_,status,__) {
        log.i('runtime status: $status');
        return Scaffold(
            appBar: AppBar(
              title: const Text('ForNet'),
              leading: IconButton(
                icon: const Icon(Icons.add),
                onPressed: () async {
                  await Navigator.push(context, MaterialPageRoute(builder: (_) => const JoinNetworkPage()));
                },
              ),
            ),
            body: Column(
              children: [
                ElevatedButton(
                    style: const ButtonStyle(),
                    onPressed: ()=> {}, child: const Text('Hello World!')),
                const SizedBox(height: 30,),
                OutlinedButton.icon(onPressed: () async{
                  var version = await api.version();
                  SmartDialog.showToast('ForNetLib Version: $version');
                }, icon: const Icon(Icons.info), label: const Text('Info'))
              ],
            )
        );
      }
    );
  }
}

