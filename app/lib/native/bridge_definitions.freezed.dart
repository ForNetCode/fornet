// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'bridge_definitions.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$ServerMessage {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            String networkId, String reason, bool deleteNetwork)
        stopWr,
    required TResult Function(String field0, PeerChange field1) syncPeers,
    required TResult Function(String field0, WrConfig field1) syncConfig,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String networkId, String reason, bool deleteNetwork)?
        stopWr,
    TResult? Function(String field0, PeerChange field1)? syncPeers,
    TResult? Function(String field0, WrConfig field1)? syncConfig,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String networkId, String reason, bool deleteNetwork)?
        stopWr,
    TResult Function(String field0, PeerChange field1)? syncPeers,
    TResult Function(String field0, WrConfig field1)? syncConfig,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ServerMessage_StopWR value) stopWr,
    required TResult Function(ServerMessage_SyncPeers value) syncPeers,
    required TResult Function(ServerMessage_SyncConfig value) syncConfig,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ServerMessage_StopWR value)? stopWr,
    TResult? Function(ServerMessage_SyncPeers value)? syncPeers,
    TResult? Function(ServerMessage_SyncConfig value)? syncConfig,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ServerMessage_StopWR value)? stopWr,
    TResult Function(ServerMessage_SyncPeers value)? syncPeers,
    TResult Function(ServerMessage_SyncConfig value)? syncConfig,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ServerMessageCopyWith<$Res> {
  factory $ServerMessageCopyWith(
          ServerMessage value, $Res Function(ServerMessage) then) =
      _$ServerMessageCopyWithImpl<$Res, ServerMessage>;
}

/// @nodoc
class _$ServerMessageCopyWithImpl<$Res, $Val extends ServerMessage>
    implements $ServerMessageCopyWith<$Res> {
  _$ServerMessageCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$ServerMessage_StopWRCopyWith<$Res> {
  factory _$$ServerMessage_StopWRCopyWith(_$ServerMessage_StopWR value,
          $Res Function(_$ServerMessage_StopWR) then) =
      __$$ServerMessage_StopWRCopyWithImpl<$Res>;
  @useResult
  $Res call({String networkId, String reason, bool deleteNetwork});
}

/// @nodoc
class __$$ServerMessage_StopWRCopyWithImpl<$Res>
    extends _$ServerMessageCopyWithImpl<$Res, _$ServerMessage_StopWR>
    implements _$$ServerMessage_StopWRCopyWith<$Res> {
  __$$ServerMessage_StopWRCopyWithImpl(_$ServerMessage_StopWR _value,
      $Res Function(_$ServerMessage_StopWR) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? networkId = null,
    Object? reason = null,
    Object? deleteNetwork = null,
  }) {
    return _then(_$ServerMessage_StopWR(
      networkId: null == networkId
          ? _value.networkId
          : networkId // ignore: cast_nullable_to_non_nullable
              as String,
      reason: null == reason
          ? _value.reason
          : reason // ignore: cast_nullable_to_non_nullable
              as String,
      deleteNetwork: null == deleteNetwork
          ? _value.deleteNetwork
          : deleteNetwork // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc

class _$ServerMessage_StopWR implements ServerMessage_StopWR {
  const _$ServerMessage_StopWR(
      {required this.networkId,
      required this.reason,
      required this.deleteNetwork});

  @override
  final String networkId;
  @override
  final String reason;
  @override
  final bool deleteNetwork;

  @override
  String toString() {
    return 'ServerMessage.stopWr(networkId: $networkId, reason: $reason, deleteNetwork: $deleteNetwork)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ServerMessage_StopWR &&
            (identical(other.networkId, networkId) ||
                other.networkId == networkId) &&
            (identical(other.reason, reason) || other.reason == reason) &&
            (identical(other.deleteNetwork, deleteNetwork) ||
                other.deleteNetwork == deleteNetwork));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, networkId, reason, deleteNetwork);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ServerMessage_StopWRCopyWith<_$ServerMessage_StopWR> get copyWith =>
      __$$ServerMessage_StopWRCopyWithImpl<_$ServerMessage_StopWR>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            String networkId, String reason, bool deleteNetwork)
        stopWr,
    required TResult Function(String field0, PeerChange field1) syncPeers,
    required TResult Function(String field0, WrConfig field1) syncConfig,
  }) {
    return stopWr(networkId, reason, deleteNetwork);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String networkId, String reason, bool deleteNetwork)?
        stopWr,
    TResult? Function(String field0, PeerChange field1)? syncPeers,
    TResult? Function(String field0, WrConfig field1)? syncConfig,
  }) {
    return stopWr?.call(networkId, reason, deleteNetwork);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String networkId, String reason, bool deleteNetwork)?
        stopWr,
    TResult Function(String field0, PeerChange field1)? syncPeers,
    TResult Function(String field0, WrConfig field1)? syncConfig,
    required TResult orElse(),
  }) {
    if (stopWr != null) {
      return stopWr(networkId, reason, deleteNetwork);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ServerMessage_StopWR value) stopWr,
    required TResult Function(ServerMessage_SyncPeers value) syncPeers,
    required TResult Function(ServerMessage_SyncConfig value) syncConfig,
  }) {
    return stopWr(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ServerMessage_StopWR value)? stopWr,
    TResult? Function(ServerMessage_SyncPeers value)? syncPeers,
    TResult? Function(ServerMessage_SyncConfig value)? syncConfig,
  }) {
    return stopWr?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ServerMessage_StopWR value)? stopWr,
    TResult Function(ServerMessage_SyncPeers value)? syncPeers,
    TResult Function(ServerMessage_SyncConfig value)? syncConfig,
    required TResult orElse(),
  }) {
    if (stopWr != null) {
      return stopWr(this);
    }
    return orElse();
  }
}

abstract class ServerMessage_StopWR implements ServerMessage {
  const factory ServerMessage_StopWR(
      {required final String networkId,
      required final String reason,
      required final bool deleteNetwork}) = _$ServerMessage_StopWR;

  String get networkId;
  String get reason;
  bool get deleteNetwork;
  @JsonKey(ignore: true)
  _$$ServerMessage_StopWRCopyWith<_$ServerMessage_StopWR> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ServerMessage_SyncPeersCopyWith<$Res> {
  factory _$$ServerMessage_SyncPeersCopyWith(_$ServerMessage_SyncPeers value,
          $Res Function(_$ServerMessage_SyncPeers) then) =
      __$$ServerMessage_SyncPeersCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0, PeerChange field1});
}

/// @nodoc
class __$$ServerMessage_SyncPeersCopyWithImpl<$Res>
    extends _$ServerMessageCopyWithImpl<$Res, _$ServerMessage_SyncPeers>
    implements _$$ServerMessage_SyncPeersCopyWith<$Res> {
  __$$ServerMessage_SyncPeersCopyWithImpl(_$ServerMessage_SyncPeers _value,
      $Res Function(_$ServerMessage_SyncPeers) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
  }) {
    return _then(_$ServerMessage_SyncPeers(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
      null == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as PeerChange,
    ));
  }
}

/// @nodoc

class _$ServerMessage_SyncPeers implements ServerMessage_SyncPeers {
  const _$ServerMessage_SyncPeers(this.field0, this.field1);

  @override
  final String field0;
  @override
  final PeerChange field1;

  @override
  String toString() {
    return 'ServerMessage.syncPeers(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ServerMessage_SyncPeers &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ServerMessage_SyncPeersCopyWith<_$ServerMessage_SyncPeers> get copyWith =>
      __$$ServerMessage_SyncPeersCopyWithImpl<_$ServerMessage_SyncPeers>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            String networkId, String reason, bool deleteNetwork)
        stopWr,
    required TResult Function(String field0, PeerChange field1) syncPeers,
    required TResult Function(String field0, WrConfig field1) syncConfig,
  }) {
    return syncPeers(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String networkId, String reason, bool deleteNetwork)?
        stopWr,
    TResult? Function(String field0, PeerChange field1)? syncPeers,
    TResult? Function(String field0, WrConfig field1)? syncConfig,
  }) {
    return syncPeers?.call(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String networkId, String reason, bool deleteNetwork)?
        stopWr,
    TResult Function(String field0, PeerChange field1)? syncPeers,
    TResult Function(String field0, WrConfig field1)? syncConfig,
    required TResult orElse(),
  }) {
    if (syncPeers != null) {
      return syncPeers(field0, field1);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ServerMessage_StopWR value) stopWr,
    required TResult Function(ServerMessage_SyncPeers value) syncPeers,
    required TResult Function(ServerMessage_SyncConfig value) syncConfig,
  }) {
    return syncPeers(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ServerMessage_StopWR value)? stopWr,
    TResult? Function(ServerMessage_SyncPeers value)? syncPeers,
    TResult? Function(ServerMessage_SyncConfig value)? syncConfig,
  }) {
    return syncPeers?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ServerMessage_StopWR value)? stopWr,
    TResult Function(ServerMessage_SyncPeers value)? syncPeers,
    TResult Function(ServerMessage_SyncConfig value)? syncConfig,
    required TResult orElse(),
  }) {
    if (syncPeers != null) {
      return syncPeers(this);
    }
    return orElse();
  }
}

abstract class ServerMessage_SyncPeers implements ServerMessage {
  const factory ServerMessage_SyncPeers(
      final String field0, final PeerChange field1) = _$ServerMessage_SyncPeers;

  String get field0;
  PeerChange get field1;
  @JsonKey(ignore: true)
  _$$ServerMessage_SyncPeersCopyWith<_$ServerMessage_SyncPeers> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ServerMessage_SyncConfigCopyWith<$Res> {
  factory _$$ServerMessage_SyncConfigCopyWith(_$ServerMessage_SyncConfig value,
          $Res Function(_$ServerMessage_SyncConfig) then) =
      __$$ServerMessage_SyncConfigCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0, WrConfig field1});
}

/// @nodoc
class __$$ServerMessage_SyncConfigCopyWithImpl<$Res>
    extends _$ServerMessageCopyWithImpl<$Res, _$ServerMessage_SyncConfig>
    implements _$$ServerMessage_SyncConfigCopyWith<$Res> {
  __$$ServerMessage_SyncConfigCopyWithImpl(_$ServerMessage_SyncConfig _value,
      $Res Function(_$ServerMessage_SyncConfig) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
  }) {
    return _then(_$ServerMessage_SyncConfig(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
      null == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as WrConfig,
    ));
  }
}

/// @nodoc

class _$ServerMessage_SyncConfig implements ServerMessage_SyncConfig {
  const _$ServerMessage_SyncConfig(this.field0, this.field1);

  @override
  final String field0;
  @override
  final WrConfig field1;

  @override
  String toString() {
    return 'ServerMessage.syncConfig(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ServerMessage_SyncConfig &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ServerMessage_SyncConfigCopyWith<_$ServerMessage_SyncConfig>
      get copyWith =>
          __$$ServerMessage_SyncConfigCopyWithImpl<_$ServerMessage_SyncConfig>(
              this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            String networkId, String reason, bool deleteNetwork)
        stopWr,
    required TResult Function(String field0, PeerChange field1) syncPeers,
    required TResult Function(String field0, WrConfig field1) syncConfig,
  }) {
    return syncConfig(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String networkId, String reason, bool deleteNetwork)?
        stopWr,
    TResult? Function(String field0, PeerChange field1)? syncPeers,
    TResult? Function(String field0, WrConfig field1)? syncConfig,
  }) {
    return syncConfig?.call(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String networkId, String reason, bool deleteNetwork)?
        stopWr,
    TResult Function(String field0, PeerChange field1)? syncPeers,
    TResult Function(String field0, WrConfig field1)? syncConfig,
    required TResult orElse(),
  }) {
    if (syncConfig != null) {
      return syncConfig(field0, field1);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ServerMessage_StopWR value) stopWr,
    required TResult Function(ServerMessage_SyncPeers value) syncPeers,
    required TResult Function(ServerMessage_SyncConfig value) syncConfig,
  }) {
    return syncConfig(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ServerMessage_StopWR value)? stopWr,
    TResult? Function(ServerMessage_SyncPeers value)? syncPeers,
    TResult? Function(ServerMessage_SyncConfig value)? syncConfig,
  }) {
    return syncConfig?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ServerMessage_StopWR value)? stopWr,
    TResult Function(ServerMessage_SyncPeers value)? syncPeers,
    TResult Function(ServerMessage_SyncConfig value)? syncConfig,
    required TResult orElse(),
  }) {
    if (syncConfig != null) {
      return syncConfig(this);
    }
    return orElse();
  }
}

abstract class ServerMessage_SyncConfig implements ServerMessage {
  const factory ServerMessage_SyncConfig(
      final String field0, final WrConfig field1) = _$ServerMessage_SyncConfig;

  String get field0;
  WrConfig get field1;
  @JsonKey(ignore: true)
  _$$ServerMessage_SyncConfigCopyWith<_$ServerMessage_SyncConfig>
      get copyWith => throw _privateConstructorUsedError;
}
