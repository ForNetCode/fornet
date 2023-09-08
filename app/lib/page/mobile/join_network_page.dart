
import 'package:flutter/material.dart';

class JoinNetworkPage extends StatelessWidget {
  const JoinNetworkPage({super.key});


  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Join Network'),
      ),
      body:Scrollbar(child:ListView(padding: const EdgeInsets.symmetric(vertical: 20, horizontal: 15,),children: [
        ListTile(title: const Text('Scan QRCode', style: TextStyle(fontSize: 18),), onTap: (){

        },),
        ListTile(title: const Text('KeyCloak Login',style: TextStyle(fontSize: 18)), onTap: (){

        },),
      ],))
    );
  }
}