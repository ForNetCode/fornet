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
mixin _$ClientInfo {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(WrConfig field0) config,
    required TResult Function(int field0) status,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(WrConfig field0)? config,
    TResult? Function(int field0)? status,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(WrConfig field0)? config,
    TResult Function(int field0)? status,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ClientInfo_Config value) config,
    required TResult Function(ClientInfo_Status value) status,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ClientInfo_Config value)? config,
    TResult? Function(ClientInfo_Status value)? status,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ClientInfo_Config value)? config,
    TResult Function(ClientInfo_Status value)? status,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ClientInfoCopyWith<$Res> {
  factory $ClientInfoCopyWith(
          ClientInfo value, $Res Function(ClientInfo) then) =
      _$ClientInfoCopyWithImpl<$Res, ClientInfo>;
}

/// @nodoc
class _$ClientInfoCopyWithImpl<$Res, $Val extends ClientInfo>
    implements $ClientInfoCopyWith<$Res> {
  _$ClientInfoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$ClientInfo_ConfigCopyWith<$Res> {
  factory _$$ClientInfo_ConfigCopyWith(
          _$ClientInfo_Config value, $Res Function(_$ClientInfo_Config) then) =
      __$$ClientInfo_ConfigCopyWithImpl<$Res>;
  @useResult
  $Res call({WrConfig field0});
}

/// @nodoc
class __$$ClientInfo_ConfigCopyWithImpl<$Res>
    extends _$ClientInfoCopyWithImpl<$Res, _$ClientInfo_Config>
    implements _$$ClientInfo_ConfigCopyWith<$Res> {
  __$$ClientInfo_ConfigCopyWithImpl(
      _$ClientInfo_Config _value, $Res Function(_$ClientInfo_Config) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$ClientInfo_Config(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as WrConfig,
    ));
  }
}

/// @nodoc

class _$ClientInfo_Config implements ClientInfo_Config {
  const _$ClientInfo_Config(this.field0);

  @override
  final WrConfig field0;

  @override
  String toString() {
    return 'ClientInfo.config(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ClientInfo_Config &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ClientInfo_ConfigCopyWith<_$ClientInfo_Config> get copyWith =>
      __$$ClientInfo_ConfigCopyWithImpl<_$ClientInfo_Config>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(WrConfig field0) config,
    required TResult Function(int field0) status,
  }) {
    return config(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(WrConfig field0)? config,
    TResult? Function(int field0)? status,
  }) {
    return config?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(WrConfig field0)? config,
    TResult Function(int field0)? status,
    required TResult orElse(),
  }) {
    if (config != null) {
      return config(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ClientInfo_Config value) config,
    required TResult Function(ClientInfo_Status value) status,
  }) {
    return config(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ClientInfo_Config value)? config,
    TResult? Function(ClientInfo_Status value)? status,
  }) {
    return config?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ClientInfo_Config value)? config,
    TResult Function(ClientInfo_Status value)? status,
    required TResult orElse(),
  }) {
    if (config != null) {
      return config(this);
    }
    return orElse();
  }
}

abstract class ClientInfo_Config implements ClientInfo {
  const factory ClientInfo_Config(final WrConfig field0) = _$ClientInfo_Config;

  @override
  WrConfig get field0;
  @JsonKey(ignore: true)
  _$$ClientInfo_ConfigCopyWith<_$ClientInfo_Config> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ClientInfo_StatusCopyWith<$Res> {
  factory _$$ClientInfo_StatusCopyWith(
          _$ClientInfo_Status value, $Res Function(_$ClientInfo_Status) then) =
      __$$ClientInfo_StatusCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$ClientInfo_StatusCopyWithImpl<$Res>
    extends _$ClientInfoCopyWithImpl<$Res, _$ClientInfo_Status>
    implements _$$ClientInfo_StatusCopyWith<$Res> {
  __$$ClientInfo_StatusCopyWithImpl(
      _$ClientInfo_Status _value, $Res Function(_$ClientInfo_Status) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$ClientInfo_Status(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$ClientInfo_Status implements ClientInfo_Status {
  const _$ClientInfo_Status(this.field0);

  @override
  final int field0;

  @override
  String toString() {
    return 'ClientInfo.status(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ClientInfo_Status &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ClientInfo_StatusCopyWith<_$ClientInfo_Status> get copyWith =>
      __$$ClientInfo_StatusCopyWithImpl<_$ClientInfo_Status>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(WrConfig field0) config,
    required TResult Function(int field0) status,
  }) {
    return status(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(WrConfig field0)? config,
    TResult? Function(int field0)? status,
  }) {
    return status?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(WrConfig field0)? config,
    TResult Function(int field0)? status,
    required TResult orElse(),
  }) {
    if (status != null) {
      return status(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ClientInfo_Config value) config,
    required TResult Function(ClientInfo_Status value) status,
  }) {
    return status(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ClientInfo_Config value)? config,
    TResult? Function(ClientInfo_Status value)? status,
  }) {
    return status?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ClientInfo_Config value)? config,
    TResult Function(ClientInfo_Status value)? status,
    required TResult orElse(),
  }) {
    if (status != null) {
      return status(this);
    }
    return orElse();
  }
}

abstract class ClientInfo_Status implements ClientInfo {
  const factory ClientInfo_Status(final int field0) = _$ClientInfo_Status;

  @override
  int get field0;
  @JsonKey(ignore: true)
  _$$ClientInfo_StatusCopyWith<_$ClientInfo_Status> get copyWith =>
      throw _privateConstructorUsedError;
}
