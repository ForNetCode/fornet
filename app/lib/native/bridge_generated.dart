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

  Stream<ForNetFlutterMessage> initRuntime(
      {required String configPath,
      required int workThread,
      required String logLevel,
      dynamic hint}) {
    var arg0 = _platform.api2wire_String(configPath);
    var arg1 = api2wire_usize(workThread);
    var arg2 = _platform.api2wire_String(logLevel);
    return _platform.executeStream(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_init_runtime(port_, arg0, arg1, arg2),
      parseSuccessData: _wire2api_for_net_flutter_message,
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

  Future<List<String>> listNetwork({dynamic hint}) {
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_list_network(port_),
      parseSuccessData: _wire2api_StringList,
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

  Future<void> start(
      {required String networkId, required int rawFd, dynamic hint}) {
    var arg0 = _platform.api2wire_String(networkId);
    var arg1 = api2wire_i32(rawFd);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_start(port_, arg0, arg1),
      parseSuccessData: _wire2api_unit,
      constMeta: kStartConstMeta,
      argValues: [networkId, rawFd],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kStartConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "start",
        argNames: ["networkId", "rawFd"],
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

  ForNetFlutterMessage _wire2api_for_net_flutter_message(dynamic raw) {
    return ForNetFlutterMessage.values[raw as int];
  }

  int _wire2api_i32(dynamic raw) {
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
}

// Section: api2wire

@protected
int api2wire_i32(int raw) {
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
  ffi.Pointer<wire_uint_8_list> api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list_0(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

// Section: finalizer

// Section: api_fill_to_wire
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

  void wire_start(
    int port_,
    ffi.Pointer<wire_uint_8_list> network_id,
    int raw_fd,
  ) {
    return _wire_start(
      port_,
      network_id,
      raw_fd,
    );
  }

  late final _wire_startPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Pointer<wire_uint_8_list>,
              ffi.Int32)>>('wire_start');
  late final _wire_start = _wire_startPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>, int)>();

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

typedef DartPostCObjectFnType = ffi.Pointer<
    ffi.NativeFunction<
        ffi.Bool Function(DartPort port_id, ffi.Pointer<ffi.Void> message)>>;
typedef DartPort = ffi.Int64;
