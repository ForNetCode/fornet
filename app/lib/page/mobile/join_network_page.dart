
import 'package:flutter/material.dart';

class JoinNetworkPage extends StatelessWidget {
  const JoinNetworkPage({super.key});


  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Join Network'),
      ),
      body: Column(children: [
        ElevatedButton(onPressed: () => {}, child: const Text('Scan QRCode')),
        ElevatedButton(onPressed: () => {}, child: const Text('KeyCloak'))
      ],),
    );
  }
}