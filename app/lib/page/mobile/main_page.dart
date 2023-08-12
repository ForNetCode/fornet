import 'package:flutter/material.dart';
import 'package:for_net_ui/page/mobile/join_network_page.dart';

class MainPage extends StatelessWidget {
  const MainPage({super.key});


  @override
  Widget build(BuildContext context) {
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
          ElevatedButton(onPressed: ()=> {}, child: const Text('Hello World!'))
        ],
      )
    );
  }
}

