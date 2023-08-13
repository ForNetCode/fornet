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
        style:  ElevatedButton.styleFrom(
            minimumSize: const Size.fromHeight(76),
            backgroundColor: Colors.deepPurple,
            padding: const EdgeInsets.symmetric(vertical: 20,),
            shape:
            const RoundedRectangleBorder(
              borderRadius: BorderRadius.all(Radius.circular(10)),
            )
        ),
          onPressed: ()=> {}, icon: const Icon(Icons.check_circle_outline, size: 30, color: Colors.white,),
        label: const Text(style: TextStyle(fontSize: 30, color: Colors.white),'Running'),
      ):
      ElevatedButton.icon(
        style:  ElevatedButton.styleFrom(
          minimumSize: const Size.fromHeight(76),
            shape:
                const RoundedRectangleBorder(
                    borderRadius: BorderRadius.all(Radius.circular(10)),
                )
        ),
        onPressed: ()=> {}, icon: const Icon(Icons.block, size: 26,),
        label: const Text('Click To Start', style: TextStyle(fontSize: 20),),
      );
    return
      Padding(padding: const EdgeInsets.symmetric(horizontal: 30), child:
      Column(
      crossAxisAlignment: CrossAxisAlignment.start,

      children: [
        const SizedBox(height: 20,),
        button,
        const SizedBox(height: 60,),
        TextButton.icon(onPressed: () async{
          var version = await api.version();
          SmartDialog.showToast('ForNetLib Version: $version');
        }, icon: const Icon(Icons.info), label: const Text('Info', style: TextStyle(fontSize: 20),))
      ],
    ));
  }
  @override
  Widget build(BuildContext context) {
    return Consumer<RuntimeStatus>(
      builder:(_,status,__) {
        log.i('runtime status: $status');
        return Scaffold(
            appBar: AppBar(
              title: const Text('ForNet'),
              actions: [IconButton(
                icon: const Icon(Icons.add, size: 30,weight:10,),
                onPressed: () async {
                await Navigator.push(context, MaterialPageRoute(builder: (_) => const JoinNetworkPage()));
              },
            ),],
            ),
            body:  status == RuntimeStatus.Unit? renderInit(): renderBody(context, status)
        );
      }
    );
  }
}

