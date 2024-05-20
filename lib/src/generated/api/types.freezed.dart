// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'types.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$JoinResult {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() notStarted,
    required TResult Function(int partsLeft) inProgress,
    required TResult Function(Joined joined) complete,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? notStarted,
    TResult? Function(int partsLeft)? inProgress,
    TResult? Function(Joined joined)? complete,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? notStarted,
    TResult Function(int partsLeft)? inProgress,
    TResult Function(Joined joined)? complete,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(JoinResult_NotStarted value) notStarted,
    required TResult Function(JoinResult_InProgress value) inProgress,
    required TResult Function(JoinResult_Complete value) complete,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(JoinResult_NotStarted value)? notStarted,
    TResult? Function(JoinResult_InProgress value)? inProgress,
    TResult? Function(JoinResult_Complete value)? complete,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(JoinResult_NotStarted value)? notStarted,
    TResult Function(JoinResult_InProgress value)? inProgress,
    TResult Function(JoinResult_Complete value)? complete,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $JoinResultCopyWith<$Res> {
  factory $JoinResultCopyWith(
          JoinResult value, $Res Function(JoinResult) then) =
      _$JoinResultCopyWithImpl<$Res, JoinResult>;
}

/// @nodoc
class _$JoinResultCopyWithImpl<$Res, $Val extends JoinResult>
    implements $JoinResultCopyWith<$Res> {
  _$JoinResultCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$JoinResult_NotStartedImplCopyWith<$Res> {
  factory _$$JoinResult_NotStartedImplCopyWith(
          _$JoinResult_NotStartedImpl value,
          $Res Function(_$JoinResult_NotStartedImpl) then) =
      __$$JoinResult_NotStartedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$JoinResult_NotStartedImplCopyWithImpl<$Res>
    extends _$JoinResultCopyWithImpl<$Res, _$JoinResult_NotStartedImpl>
    implements _$$JoinResult_NotStartedImplCopyWith<$Res> {
  __$$JoinResult_NotStartedImplCopyWithImpl(_$JoinResult_NotStartedImpl _value,
      $Res Function(_$JoinResult_NotStartedImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$JoinResult_NotStartedImpl extends JoinResult_NotStarted {
  const _$JoinResult_NotStartedImpl() : super._();

  @override
  String toString() {
    return 'JoinResult.notStarted()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$JoinResult_NotStartedImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() notStarted,
    required TResult Function(int partsLeft) inProgress,
    required TResult Function(Joined joined) complete,
  }) {
    return notStarted();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? notStarted,
    TResult? Function(int partsLeft)? inProgress,
    TResult? Function(Joined joined)? complete,
  }) {
    return notStarted?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? notStarted,
    TResult Function(int partsLeft)? inProgress,
    TResult Function(Joined joined)? complete,
    required TResult orElse(),
  }) {
    if (notStarted != null) {
      return notStarted();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(JoinResult_NotStarted value) notStarted,
    required TResult Function(JoinResult_InProgress value) inProgress,
    required TResult Function(JoinResult_Complete value) complete,
  }) {
    return notStarted(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(JoinResult_NotStarted value)? notStarted,
    TResult? Function(JoinResult_InProgress value)? inProgress,
    TResult? Function(JoinResult_Complete value)? complete,
  }) {
    return notStarted?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(JoinResult_NotStarted value)? notStarted,
    TResult Function(JoinResult_InProgress value)? inProgress,
    TResult Function(JoinResult_Complete value)? complete,
    required TResult orElse(),
  }) {
    if (notStarted != null) {
      return notStarted(this);
    }
    return orElse();
  }
}

abstract class JoinResult_NotStarted extends JoinResult {
  const factory JoinResult_NotStarted() = _$JoinResult_NotStartedImpl;
  const JoinResult_NotStarted._() : super._();
}

/// @nodoc
abstract class _$$JoinResult_InProgressImplCopyWith<$Res> {
  factory _$$JoinResult_InProgressImplCopyWith(
          _$JoinResult_InProgressImpl value,
          $Res Function(_$JoinResult_InProgressImpl) then) =
      __$$JoinResult_InProgressImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int partsLeft});
}

/// @nodoc
class __$$JoinResult_InProgressImplCopyWithImpl<$Res>
    extends _$JoinResultCopyWithImpl<$Res, _$JoinResult_InProgressImpl>
    implements _$$JoinResult_InProgressImplCopyWith<$Res> {
  __$$JoinResult_InProgressImplCopyWithImpl(_$JoinResult_InProgressImpl _value,
      $Res Function(_$JoinResult_InProgressImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? partsLeft = null,
  }) {
    return _then(_$JoinResult_InProgressImpl(
      partsLeft: null == partsLeft
          ? _value.partsLeft
          : partsLeft // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$JoinResult_InProgressImpl extends JoinResult_InProgress {
  const _$JoinResult_InProgressImpl({required this.partsLeft}) : super._();

  /// The number of parts left to join
  @override
  final int partsLeft;

  @override
  String toString() {
    return 'JoinResult.inProgress(partsLeft: $partsLeft)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$JoinResult_InProgressImpl &&
            (identical(other.partsLeft, partsLeft) ||
                other.partsLeft == partsLeft));
  }

  @override
  int get hashCode => Object.hash(runtimeType, partsLeft);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$JoinResult_InProgressImplCopyWith<_$JoinResult_InProgressImpl>
      get copyWith => __$$JoinResult_InProgressImplCopyWithImpl<
          _$JoinResult_InProgressImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() notStarted,
    required TResult Function(int partsLeft) inProgress,
    required TResult Function(Joined joined) complete,
  }) {
    return inProgress(partsLeft);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? notStarted,
    TResult? Function(int partsLeft)? inProgress,
    TResult? Function(Joined joined)? complete,
  }) {
    return inProgress?.call(partsLeft);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? notStarted,
    TResult Function(int partsLeft)? inProgress,
    TResult Function(Joined joined)? complete,
    required TResult orElse(),
  }) {
    if (inProgress != null) {
      return inProgress(partsLeft);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(JoinResult_NotStarted value) notStarted,
    required TResult Function(JoinResult_InProgress value) inProgress,
    required TResult Function(JoinResult_Complete value) complete,
  }) {
    return inProgress(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(JoinResult_NotStarted value)? notStarted,
    TResult? Function(JoinResult_InProgress value)? inProgress,
    TResult? Function(JoinResult_Complete value)? complete,
  }) {
    return inProgress?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(JoinResult_NotStarted value)? notStarted,
    TResult Function(JoinResult_InProgress value)? inProgress,
    TResult Function(JoinResult_Complete value)? complete,
    required TResult orElse(),
  }) {
    if (inProgress != null) {
      return inProgress(this);
    }
    return orElse();
  }
}

abstract class JoinResult_InProgress extends JoinResult {
  const factory JoinResult_InProgress({required final int partsLeft}) =
      _$JoinResult_InProgressImpl;
  const JoinResult_InProgress._() : super._();

  /// The number of parts left to join
  int get partsLeft;
  @JsonKey(ignore: true)
  _$$JoinResult_InProgressImplCopyWith<_$JoinResult_InProgressImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$JoinResult_CompleteImplCopyWith<$Res> {
  factory _$$JoinResult_CompleteImplCopyWith(_$JoinResult_CompleteImpl value,
          $Res Function(_$JoinResult_CompleteImpl) then) =
      __$$JoinResult_CompleteImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Joined joined});
}

/// @nodoc
class __$$JoinResult_CompleteImplCopyWithImpl<$Res>
    extends _$JoinResultCopyWithImpl<$Res, _$JoinResult_CompleteImpl>
    implements _$$JoinResult_CompleteImplCopyWith<$Res> {
  __$$JoinResult_CompleteImplCopyWithImpl(_$JoinResult_CompleteImpl _value,
      $Res Function(_$JoinResult_CompleteImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? joined = null,
  }) {
    return _then(_$JoinResult_CompleteImpl(
      joined: null == joined
          ? _value.joined
          : joined // ignore: cast_nullable_to_non_nullable
              as Joined,
    ));
  }
}

/// @nodoc

class _$JoinResult_CompleteImpl extends JoinResult_Complete {
  const _$JoinResult_CompleteImpl({required this.joined}) : super._();

  @override
  final Joined joined;

  @override
  String toString() {
    return 'JoinResult.complete(joined: $joined)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$JoinResult_CompleteImpl &&
            (identical(other.joined, joined) || other.joined == joined));
  }

  @override
  int get hashCode => Object.hash(runtimeType, joined);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$JoinResult_CompleteImplCopyWith<_$JoinResult_CompleteImpl> get copyWith =>
      __$$JoinResult_CompleteImplCopyWithImpl<_$JoinResult_CompleteImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() notStarted,
    required TResult Function(int partsLeft) inProgress,
    required TResult Function(Joined joined) complete,
  }) {
    return complete(joined);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? notStarted,
    TResult? Function(int partsLeft)? inProgress,
    TResult? Function(Joined joined)? complete,
  }) {
    return complete?.call(joined);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? notStarted,
    TResult Function(int partsLeft)? inProgress,
    TResult Function(Joined joined)? complete,
    required TResult orElse(),
  }) {
    if (complete != null) {
      return complete(joined);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(JoinResult_NotStarted value) notStarted,
    required TResult Function(JoinResult_InProgress value) inProgress,
    required TResult Function(JoinResult_Complete value) complete,
  }) {
    return complete(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(JoinResult_NotStarted value)? notStarted,
    TResult? Function(JoinResult_InProgress value)? inProgress,
    TResult? Function(JoinResult_Complete value)? complete,
  }) {
    return complete?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(JoinResult_NotStarted value)? notStarted,
    TResult Function(JoinResult_InProgress value)? inProgress,
    TResult Function(JoinResult_Complete value)? complete,
    required TResult orElse(),
  }) {
    if (complete != null) {
      return complete(this);
    }
    return orElse();
  }
}

abstract class JoinResult_Complete extends JoinResult {
  const factory JoinResult_Complete({required final Joined joined}) =
      _$JoinResult_CompleteImpl;
  const JoinResult_Complete._() : super._();

  Joined get joined;
  @JsonKey(ignore: true)
  _$$JoinResult_CompleteImplCopyWith<_$JoinResult_CompleteImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
