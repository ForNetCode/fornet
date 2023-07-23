// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.72.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';

abstract class FornetLib {
  Future<int> testOne({required int a, required int b, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kTestOneConstMeta;

  Future<int> testTwo({required int a, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kTestTwoConstMeta;

  Future<String> getConfigPath({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kGetConfigPathConstMeta;

  Future<void> initRuntime(
      {required String configPath,
      required int workThread,
      required String logLevel,
      dynamic hint});

  FlutterRustBridgeTaskConstMeta get kInitRuntimeConstMeta;

  Future<String> joinNetwork({required String inviteCode, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kJoinNetworkConstMeta;

  Future<String> listNetwork({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kListNetworkConstMeta;
}
