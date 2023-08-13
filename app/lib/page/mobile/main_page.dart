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


  renderInit() {
    return const Center(child: Text('Init ForNet Service'),);
  }
  renderBody(BuildContext context,RuntimeStatus status) {
    var button = status == RuntimeStatus.Connected ?
      ElevatedButton.icon(
          onPressed: ()=> {}, icon: const Icon(Icons.check_circle_outline, color: Colors.white,),
        label: const Text('Running'),
      ):
      ElevatedButton.icon(
        style:  ElevatedButton.styleFrom(backgroundColor: Colors.black12),
        onPressed: ()=> {}, icon: const Icon(Icons.block_flipped, color: Colors.white,),
        label: const Text('Click To Start'),
      );
    return Column(
      children: [
        const SizedBox(height: 20,),
        button,
        const SizedBox(height: 30,),
        OutlinedButton.icon(onPressed: () async{
          var version = await api.version();
          SmartDialog.showToast('ForNetLib Version: $version');
        }, icon: const Icon(Icons.info), label: const Text('Info'))
      ],
    );
  }
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
            body:  status == RuntimeStatus.Unit? renderInit():renderBody(context, status)
        );
      }
    );
  }
}

