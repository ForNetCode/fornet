// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.80.1.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import "bridge_definitions.dart";
import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';

import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';

import 'dart:ffi' as ffi;

class FornetLibImpl implements FornetLib {
  final FornetLibPlatform _platform;
  factory FornetLibImpl(ExternalLibrary dylib) =>
      FornetLibImpl.raw(FornetLibPlatform(dylib));

  /// Only valid on web/WASM platforms.
  factory FornetLibImpl.wasm(FutureOr<WasmModule> module) =>
      FornetLibImpl(module as ExternalLibrary);
  FornetLibImpl.raw(this._platform);
  Future<String> getConfigPath({dynamic hint}) {
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_get_config_path(port_),
      parseSuccessData: _wire2api_String,
      constMeta: kGetConfigPathConstMeta,
      argValues: [],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kGetConfigPathConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "get_config_path",
        argNames: [],
      );

  Future<void> initRuntime(
      {required String configPath,
      required int workThread,
      required String logLevel,
      dynamic hint}) {
    var arg0 = _platform.api2wire_String(configPath);
    var arg1 = api2wire_usize(workThread);
    var arg2 = _platform.api2wire_String(logLevel);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_init_runtime(port_, arg0, arg1, arg2),
      parseSuccessData: _wire2api_unit,
      constMeta: kInitRuntimeConstMeta,
      argValues: [configPath, workThread, logLevel],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kInitRuntimeConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "init_runtime",
        argNames: ["configPath", "workThread", "logLevel"],
      );

  Future<String> joinNetwork({required String inviteCode, dynamic hint}) {
    var arg0 = _platform.api2wire_String(inviteCode);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_join_network(port_, arg0),
      parseSuccessData: _wire2api_String,
      constMeta: kJoinNetworkConstMeta,
      argValues: [inviteCode],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kJoinNetworkConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "join_network",
        argNames: ["inviteCode"],
      );

  Future<String> listNetwork({dynamic hint}) {
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_list_network(port_),
      parseSuccessData: _wire2api_String,
      constMeta: kListNetworkConstMeta,
      argValues: [],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kListNetworkConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "list_network",
        argNames: [],
      );

  Future<String> version({dynamic hint}) {
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_version(port_),
      parseSuccessData: _wire2api_String,
      constMeta: kVersionConstMeta,
      argValues: [],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kVersionConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "version",
        argNames: [],
      );

  Future<ClientMessage?> testParam(
      {required ClientMessage clientMessage, dynamic hint}) {
    var arg0 = _platform.api2wire_box_autoadd_client_message(clientMessage);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_test_param(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_client_message,
      constMeta: kTestParamConstMeta,
      argValues: [clientMessage],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kTestParamConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "test_param",
        argNames: ["clientMessage"],
      );

  void dispose() {
    _platform.dispose();
  }
// Section: wire2api

  String _wire2api_String(dynamic raw) {
    return raw as String;
  }

  List<String> _wire2api_StringList(dynamic raw) {
    return (raw as List<dynamic>).cast<String>();
  }

  ClientInfo _wire2api_box_autoadd_client_info(dynamic raw) {
    return _wire2api_client_info(raw);
  }

  ClientMessage _wire2api_box_autoadd_client_message(dynamic raw) {
    return _wire2api_client_message(raw);
  }

  Interface _wire2api_box_autoadd_interface(dynamic raw) {
    return _wire2api_interface(raw);
  }

  int _wire2api_box_autoadd_u32(dynamic raw) {
    return raw as int;
  }

  WrConfig _wire2api_box_autoadd_wr_config(dynamic raw) {
    return _wire2api_wr_config(raw);
  }

  ClientInfo _wire2api_client_info(dynamic raw) {
    switch (raw[0]) {
      case 0:
        return ClientInfo_Config(
          _wire2api_box_autoadd_wr_config(raw[1]),
        );
      case 1:
        return ClientInfo_Status(
          _wire2api_i32(raw[1]),
        );
      default:
        throw Exception("unreachable");
    }
  }

  ClientMessage _wire2api_client_message(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return ClientMessage(
      networkId: _wire2api_String(arr[0]),
      info: _wire2api_opt_box_autoadd_client_info(arr[1]),
    );
  }

  int _wire2api_i32(dynamic raw) {
    return raw as int;
  }

  Interface _wire2api_interface(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 10)
      throw Exception('unexpected arr length: expect 10 but see ${arr.length}');
    return Interface(
      name: _wire2api_opt_String(arr[0]),
      address: _wire2api_StringList(arr[1]),
      listenPort: _wire2api_i32(arr[2]),
      dns: _wire2api_StringList(arr[3]),
      mtu: _wire2api_opt_box_autoadd_u32(arr[4]),
      preUp: _wire2api_opt_String(arr[5]),
      postUp: _wire2api_opt_String(arr[6]),
      preDown: _wire2api_opt_String(arr[7]),
      postDown: _wire2api_opt_String(arr[8]),
      protocol: _wire2api_i32(arr[9]),
    );
  }

  List<Peer> _wire2api_list_peer(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_peer).toList();
  }

  String? _wire2api_opt_String(dynamic raw) {
    return raw == null ? null : _wire2api_String(raw);
  }

  ClientInfo? _wire2api_opt_box_autoadd_client_info(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_client_info(raw);
  }

  ClientMessage? _wire2api_opt_box_autoadd_client_message(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_client_message(raw);
  }

  Interface? _wire2api_opt_box_autoadd_interface(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_interface(raw);
  }

  int? _wire2api_opt_box_autoadd_u32(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_u32(raw);
  }

  Peer _wire2api_peer(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 5)
      throw Exception('unexpected arr length: expect 5 but see ${arr.length}');
    return Peer(
      endpoint: _wire2api_opt_String(arr[0]),
      allowedIp: _wire2api_StringList(arr[1]),
      publicKey: _wire2api_String(arr[2]),
      persistenceKeepAlive: _wire2api_u32(arr[3]),
      address: _wire2api_StringList(arr[4]),
    );
  }

  int _wire2api_u32(dynamic raw) {
    return raw as int;
  }

  int _wire2api_u8(dynamic raw) {
    return raw as int;
  }

  Uint8List _wire2api_uint_8_list(dynamic raw) {
    return raw as Uint8List;
  }

  void _wire2api_unit(dynamic raw) {
    return;
  }

  WrConfig _wire2api_wr_config(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 3)
      throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
    return WrConfig(
      interface: _wire2api_opt_box_autoadd_interface(arr[0]),
      peers: _wire2api_list_peer(arr[1]),
      typ: _wire2api_i32(arr[2]),
    );
  }
}

// Section: api2wire

@protected
int api2wire_i32(int raw) {
  return raw;
}

@protected
int api2wire_u32(int raw) {
  return raw;
}

@protected
int api2wire_u8(int raw) {
  return raw;
}

@protected
int api2wire_usize(int raw) {
  return raw;
}

// Section: finalizer

class FornetLibPlatform extends FlutterRustBridgeBase<FornetLibWire> {
  FornetLibPlatform(ffi.DynamicLibrary dylib) : super(FornetLibWire(dylib));

// Section: api2wire

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_String(String raw) {
    return api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  @protected
  ffi.Pointer<wire_StringList> api2wire_StringList(List<String> raw) {
    final ans = inner.new_StringList_0(raw.length);
    for (var i = 0; i < raw.length; i++) {
      ans.ref.ptr[i] = api2wire_String(raw[i]);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_ClientInfo> api2wire_box_autoadd_client_info(
      ClientInfo raw) {
    final ptr = inner.new_box_autoadd_client_info_0();
    _api_fill_to_wire_client_info(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_ClientMessage> api2wire_box_autoadd_client_message(
      ClientMessage raw) {
    final ptr = inner.new_box_autoadd_client_message_0();
    _api_fill_to_wire_client_message(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_Interface> api2wire_box_autoadd_interface(Interface raw) {
    final ptr = inner.new_box_autoadd_interface_0();
    _api_fill_to_wire_interface(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<ffi.Uint32> api2wire_box_autoadd_u32(int raw) {
    return inner.new_box_autoadd_u32_0(api2wire_u32(raw));
  }

  @protected
  ffi.Pointer<wire_WrConfig> api2wire_box_autoadd_wr_config(WrConfig raw) {
    final ptr = inner.new_box_autoadd_wr_config_0();
    _api_fill_to_wire_wr_config(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_list_peer> api2wire_list_peer(List<Peer> raw) {
    final ans = inner.new_list_peer_0(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      _api_fill_to_wire_peer(raw[i], ans.ref.ptr[i]);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_opt_String(String? raw) {
    return raw == null ? ffi.nullptr : api2wire_String(raw);
  }

  @protected
  ffi.Pointer<wire_ClientInfo> api2wire_opt_box_autoadd_client_info(
      ClientInfo? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_client_info(raw);
  }

  @protected
  ffi.Pointer<wire_Interface> api2wire_opt_box_autoadd_interface(
      Interface? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_interface(raw);
  }

  @protected
  ffi.Pointer<ffi.Uint32> api2wire_opt_box_autoadd_u32(int? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_u32(raw);
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list_0(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

// Section: finalizer

// Section: api_fill_to_wire

  void _api_fill_to_wire_box_autoadd_client_info(
      ClientInfo apiObj, ffi.Pointer<wire_ClientInfo> wireObj) {
    _api_fill_to_wire_client_info(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_client_message(
      ClientMessage apiObj, ffi.Pointer<wire_ClientMessage> wireObj) {
    _api_fill_to_wire_client_message(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_interface(
      Interface apiObj, ffi.Pointer<wire_Interface> wireObj) {
    _api_fill_to_wire_interface(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_wr_config(
      WrConfig apiObj, ffi.Pointer<wire_WrConfig> wireObj) {
    _api_fill_to_wire_wr_config(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_client_info(
      ClientInfo apiObj, wire_ClientInfo wireObj) {
    if (apiObj is ClientInfo_Config) {
      var pre_field0 = api2wire_box_autoadd_wr_config(apiObj.field0);
      wireObj.tag = 0;
      wireObj.kind = inner.inflate_ClientInfo_Config();
      wireObj.kind.ref.Config.ref.field0 = pre_field0;
      return;
    }
    if (apiObj is ClientInfo_Status) {
      var pre_field0 = api2wire_i32(apiObj.field0);
      wireObj.tag = 1;
      wireObj.kind = inner.inflate_ClientInfo_Status();
      wireObj.kind.ref.Status.ref.field0 = pre_field0;
      return;
    }
  }

  void _api_fill_to_wire_client_message(
      ClientMessage apiObj, wire_ClientMessage wireObj) {
    wireObj.network_id = api2wire_String(apiObj.networkId);
    wireObj.info = api2wire_opt_box_autoadd_client_info(apiObj.info);
  }

  void _api_fill_to_wire_interface(Interface apiObj, wire_Interface wireObj) {
    wireObj.name = api2wire_opt_String(apiObj.name);
    wireObj.address = api2wire_StringList(apiObj.address);
    wireObj.listen_port = api2wire_i32(apiObj.listenPort);
    wireObj.dns = api2wire_StringList(apiObj.dns);
    wireObj.mtu = api2wire_opt_box_autoadd_u32(apiObj.mtu);
    wireObj.pre_up = api2wire_opt_String(apiObj.preUp);
    wireObj.post_up = api2wire_opt_String(apiObj.postUp);
    wireObj.pre_down = api2wire_opt_String(apiObj.preDown);
    wireObj.post_down = api2wire_opt_String(apiObj.postDown);
    wireObj.protocol = api2wire_i32(apiObj.protocol);
  }

  void _api_fill_to_wire_opt_box_autoadd_client_info(
      ClientInfo? apiObj, ffi.Pointer<wire_ClientInfo> wireObj) {
    if (apiObj != null)
      _api_fill_to_wire_box_autoadd_client_info(apiObj, wireObj);
  }

  void _api_fill_to_wire_opt_box_autoadd_interface(
      Interface? apiObj, ffi.Pointer<wire_Interface> wireObj) {
    if (apiObj != null)
      _api_fill_to_wire_box_autoadd_interface(apiObj, wireObj);
  }

  void _api_fill_to_wire_peer(Peer apiObj, wire_Peer wireObj) {
    wireObj.endpoint = api2wire_opt_String(apiObj.endpoint);
    wireObj.allowed_ip = api2wire_StringList(apiObj.allowedIp);
    wireObj.public_key = api2wire_String(apiObj.publicKey);
    wireObj.persistence_keep_alive = api2wire_u32(apiObj.persistenceKeepAlive);
    wireObj.address = api2wire_StringList(apiObj.address);
  }

  void _api_fill_to_wire_wr_config(WrConfig apiObj, wire_WrConfig wireObj) {
    wireObj.interface = api2wire_opt_box_autoadd_interface(apiObj.interface);
    wireObj.peers = api2wire_list_peer(apiObj.peers);
    wireObj.typ = api2wire_i32(apiObj.typ);
  }
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.
// ignore_for_file: type=lint

/// generated by flutter_rust_bridge
class FornetLibWire implements FlutterRustBridgeWireBase {
  @internal
  late final dartApi = DartApiDl(init_frb_dart_api_dl);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  FornetLibWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  FornetLibWire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>(
          'store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr
      .asFunction<void Function(DartPostCObjectFnType)>();

  Object get_dart_object(
    int ptr,
  ) {
    return _get_dart_object(
      ptr,
    );
  }

  late final _get_dart_objectPtr =
      _lookup<ffi.NativeFunction<ffi.Handle Function(ffi.UintPtr)>>(
          'get_dart_object');
  late final _get_dart_object =
      _get_dart_objectPtr.asFunction<Object Function(int)>();

  void drop_dart_object(
    int ptr,
  ) {
    return _drop_dart_object(
      ptr,
    );
  }

  late final _drop_dart_objectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.UintPtr)>>(
          'drop_dart_object');
  late final _drop_dart_object =
      _drop_dart_objectPtr.asFunction<void Function(int)>();

  int new_dart_opaque(
    Object handle,
  ) {
    return _new_dart_opaque(
      handle,
    );
  }

  late final _new_dart_opaquePtr =
      _lookup<ffi.NativeFunction<ffi.UintPtr Function(ffi.Handle)>>(
          'new_dart_opaque');
  late final _new_dart_opaque =
      _new_dart_opaquePtr.asFunction<int Function(Object)>();

  int init_frb_dart_api_dl(
    ffi.Pointer<ffi.Void> obj,
  ) {
    return _init_frb_dart_api_dl(
      obj,
    );
  }

  late final _init_frb_dart_api_dlPtr =
      _lookup<ffi.NativeFunction<ffi.IntPtr Function(ffi.Pointer<ffi.Void>)>>(
          'init_frb_dart_api_dl');
  late final _init_frb_dart_api_dl = _init_frb_dart_api_dlPtr
      .asFunction<int Function(ffi.Pointer<ffi.Void>)>();

  void wire_get_config_path(
    int port_,
  ) {
    return _wire_get_config_path(
      port_,
    );
  }

  late final _wire_get_config_pathPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_get_config_path');
  late final _wire_get_config_path =
      _wire_get_config_pathPtr.asFunction<void Function(int)>();

  void wire_init_runtime(
    int port_,
    ffi.Pointer<wire_uint_8_list> config_path,
    int work_thread,
    ffi.Pointer<wire_uint_8_list> log_level,
  ) {
    return _wire_init_runtime(
      port_,
      config_path,
      work_thread,
      log_level,
    );
  }

  late final _wire_init_runtimePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64,
              ffi.Pointer<wire_uint_8_list>,
              ffi.UintPtr,
              ffi.Pointer<wire_uint_8_list>)>>('wire_init_runtime');
  late final _wire_init_runtime = _wire_init_runtimePtr.asFunction<
      void Function(int, ffi.Pointer<wire_uint_8_list>, int,
          ffi.Pointer<wire_uint_8_list>)>();

  void wire_join_network(
    int port_,
    ffi.Pointer<wire_uint_8_list> invite_code,
  ) {
    return _wire_join_network(
      port_,
      invite_code,
    );
  }

  late final _wire_join_networkPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>('wire_join_network');
  late final _wire_join_network = _wire_join_networkPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_list_network(
    int port_,
  ) {
    return _wire_list_network(
      port_,
    );
  }

  late final _wire_list_networkPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_list_network');
  late final _wire_list_network =
      _wire_list_networkPtr.asFunction<void Function(int)>();

  void wire_version(
    int port_,
  ) {
    return _wire_version(
      port_,
    );
  }

  late final _wire_versionPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>('wire_version');
  late final _wire_version = _wire_versionPtr.asFunction<void Function(int)>();

  void wire_test_param(
    int port_,
    ffi.Pointer<wire_ClientMessage> client_message,
  ) {
    return _wire_test_param(
      port_,
      client_message,
    );
  }

  late final _wire_test_paramPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_ClientMessage>)>>('wire_test_param');
  late final _wire_test_param = _wire_test_paramPtr
      .asFunction<void Function(int, ffi.Pointer<wire_ClientMessage>)>();

  ffi.Pointer<wire_StringList> new_StringList_0(
    int len,
  ) {
    return _new_StringList_0(
      len,
    );
  }

  late final _new_StringList_0Ptr = _lookup<
          ffi.NativeFunction<ffi.Pointer<wire_StringList> Function(ffi.Int32)>>(
      'new_StringList_0');
  late final _new_StringList_0 = _new_StringList_0Ptr
      .asFunction<ffi.Pointer<wire_StringList> Function(int)>();

  ffi.Pointer<wire_ClientInfo> new_box_autoadd_client_info_0() {
    return _new_box_autoadd_client_info_0();
  }

  late final _new_box_autoadd_client_info_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_ClientInfo> Function()>>(
          'new_box_autoadd_client_info_0');
  late final _new_box_autoadd_client_info_0 = _new_box_autoadd_client_info_0Ptr
      .asFunction<ffi.Pointer<wire_ClientInfo> Function()>();

  ffi.Pointer<wire_ClientMessage> new_box_autoadd_client_message_0() {
    return _new_box_autoadd_client_message_0();
  }

  late final _new_box_autoadd_client_message_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_ClientMessage> Function()>>(
          'new_box_autoadd_client_message_0');
  late final _new_box_autoadd_client_message_0 =
      _new_box_autoadd_client_message_0Ptr
          .asFunction<ffi.Pointer<wire_ClientMessage> Function()>();

  ffi.Pointer<wire_Interface> new_box_autoadd_interface_0() {
    return _new_box_autoadd_interface_0();
  }

  late final _new_box_autoadd_interface_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_Interface> Function()>>(
          'new_box_autoadd_interface_0');
  late final _new_box_autoadd_interface_0 = _new_box_autoadd_interface_0Ptr
      .asFunction<ffi.Pointer<wire_Interface> Function()>();

  ffi.Pointer<ffi.Uint32> new_box_autoadd_u32_0(
    int value,
  ) {
    return _new_box_autoadd_u32_0(
      value,
    );
  }

  late final _new_box_autoadd_u32_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Uint32> Function(ffi.Uint32)>>(
          'new_box_autoadd_u32_0');
  late final _new_box_autoadd_u32_0 = _new_box_autoadd_u32_0Ptr
      .asFunction<ffi.Pointer<ffi.Uint32> Function(int)>();

  ffi.Pointer<wire_WrConfig> new_box_autoadd_wr_config_0() {
    return _new_box_autoadd_wr_config_0();
  }

  late final _new_box_autoadd_wr_config_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_WrConfig> Function()>>(
          'new_box_autoadd_wr_config_0');
  late final _new_box_autoadd_wr_config_0 = _new_box_autoadd_wr_config_0Ptr
      .asFunction<ffi.Pointer<wire_WrConfig> Function()>();

  ffi.Pointer<wire_list_peer> new_list_peer_0(
    int len,
  ) {
    return _new_list_peer_0(
      len,
    );
  }

  late final _new_list_peer_0Ptr = _lookup<
          ffi.NativeFunction<ffi.Pointer<wire_list_peer> Function(ffi.Int32)>>(
      'new_list_peer_0');
  late final _new_list_peer_0 = _new_list_peer_0Ptr
      .asFunction<ffi.Pointer<wire_list_peer> Function(int)>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list_0(
    int len,
  ) {
    return _new_uint_8_list_0(
      len,
    );
  }

  late final _new_uint_8_list_0Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_uint_8_list> Function(
              ffi.Int32)>>('new_uint_8_list_0');
  late final _new_uint_8_list_0 = _new_uint_8_list_0Ptr
      .asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

  ffi.Pointer<ClientInfoKind> inflate_ClientInfo_Config() {
    return _inflate_ClientInfo_Config();
  }

  late final _inflate_ClientInfo_ConfigPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ClientInfoKind> Function()>>(
          'inflate_ClientInfo_Config');
  late final _inflate_ClientInfo_Config = _inflate_ClientInfo_ConfigPtr
      .asFunction<ffi.Pointer<ClientInfoKind> Function()>();

  ffi.Pointer<ClientInfoKind> inflate_ClientInfo_Status() {
    return _inflate_ClientInfo_Status();
  }

  late final _inflate_ClientInfo_StatusPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ClientInfoKind> Function()>>(
          'inflate_ClientInfo_Status');
  late final _inflate_ClientInfo_Status = _inflate_ClientInfo_StatusPtr
      .asFunction<ffi.Pointer<ClientInfoKind> Function()>();

  void free_WireSyncReturn(
    WireSyncReturn ptr,
  ) {
    return _free_WireSyncReturn(
      ptr,
    );
  }

  late final _free_WireSyncReturnPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturn)>>(
          'free_WireSyncReturn');
  late final _free_WireSyncReturn =
      _free_WireSyncReturnPtr.asFunction<void Function(WireSyncReturn)>();
}

final class _Dart_Handle extends ffi.Opaque {}

final class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_StringList extends ffi.Struct {
  external ffi.Pointer<ffi.Pointer<wire_uint_8_list>> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_Interface extends ffi.Struct {
  external ffi.Pointer<wire_uint_8_list> name;

  external ffi.Pointer<wire_StringList> address;

  @ffi.Int32()
  external int listen_port;

  external ffi.Pointer<wire_StringList> dns;

  external ffi.Pointer<ffi.Uint32> mtu;

  external ffi.Pointer<wire_uint_8_list> pre_up;

  external ffi.Pointer<wire_uint_8_list> post_up;

  external ffi.Pointer<wire_uint_8_list> pre_down;

  external ffi.Pointer<wire_uint_8_list> post_down;

  @ffi.Int32()
  external int protocol;
}

final class wire_Peer extends ffi.Struct {
  external ffi.Pointer<wire_uint_8_list> endpoint;

  external ffi.Pointer<wire_StringList> allowed_ip;

  external ffi.Pointer<wire_uint_8_list> public_key;

  @ffi.Uint32()
  external int persistence_keep_alive;

  external ffi.Pointer<wire_StringList> address;
}

final class wire_list_peer extends ffi.Struct {
  external ffi.Pointer<wire_Peer> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_WrConfig extends ffi.Struct {
  external ffi.Pointer<wire_Interface> interface1;

  external ffi.Pointer<wire_list_peer> peers;

  @ffi.Int32()
  external int typ;
}

final class wire_ClientInfo_Config extends ffi.Struct {
  external ffi.Pointer<wire_WrConfig> field0;
}

final class wire_ClientInfo_Status extends ffi.Struct {
  @ffi.Int32()
  external int field0;
}

final class ClientInfoKind extends ffi.Union {
  external ffi.Pointer<wire_ClientInfo_Config> Config;

  external ffi.Pointer<wire_ClientInfo_Status> Status;
}

final class wire_ClientInfo extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external ffi.Pointer<ClientInfoKind> kind;
}

final class wire_ClientMessage extends ffi.Struct {
  external ffi.Pointer<wire_uint_8_list> network_id;

  external ffi.Pointer<wire_ClientInfo> info;
}

typedef DartPostCObjectFnType = ffi.Pointer<
    ffi.NativeFunction<
        ffi.Bool Function(DartPort port_id, ffi.Pointer<ffi.Void> message)>>;
typedef DartPort = ffi.Int64;
