// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'types.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$JoinResult {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is JoinResult);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'JoinResult()';
}


}

/// @nodoc
class $JoinResultCopyWith<$Res>  {
$JoinResultCopyWith(JoinResult _, $Res Function(JoinResult) __);
}


/// @nodoc


class JoinResult_NotStarted extends JoinResult {
  const JoinResult_NotStarted(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is JoinResult_NotStarted);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'JoinResult.notStarted()';
}


}




/// @nodoc


class JoinResult_InProgress extends JoinResult {
  const JoinResult_InProgress({required this.partsLeft}): super._();
  

/// The number of parts left to join
 final  BigInt partsLeft;

/// Create a copy of JoinResult
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$JoinResult_InProgressCopyWith<JoinResult_InProgress> get copyWith => _$JoinResult_InProgressCopyWithImpl<JoinResult_InProgress>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is JoinResult_InProgress&&(identical(other.partsLeft, partsLeft) || other.partsLeft == partsLeft));
}


@override
int get hashCode => Object.hash(runtimeType,partsLeft);

@override
String toString() {
  return 'JoinResult.inProgress(partsLeft: $partsLeft)';
}


}

/// @nodoc
abstract mixin class $JoinResult_InProgressCopyWith<$Res> implements $JoinResultCopyWith<$Res> {
  factory $JoinResult_InProgressCopyWith(JoinResult_InProgress value, $Res Function(JoinResult_InProgress) _then) = _$JoinResult_InProgressCopyWithImpl;
@useResult
$Res call({
 BigInt partsLeft
});




}
/// @nodoc
class _$JoinResult_InProgressCopyWithImpl<$Res>
    implements $JoinResult_InProgressCopyWith<$Res> {
  _$JoinResult_InProgressCopyWithImpl(this._self, this._then);

  final JoinResult_InProgress _self;
  final $Res Function(JoinResult_InProgress) _then;

/// Create a copy of JoinResult
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? partsLeft = null,}) {
  return _then(JoinResult_InProgress(
partsLeft: null == partsLeft ? _self.partsLeft : partsLeft // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class JoinResult_Complete extends JoinResult {
  const JoinResult_Complete({required this.joined}): super._();
  

 final  Joined joined;

/// Create a copy of JoinResult
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$JoinResult_CompleteCopyWith<JoinResult_Complete> get copyWith => _$JoinResult_CompleteCopyWithImpl<JoinResult_Complete>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is JoinResult_Complete&&(identical(other.joined, joined) || other.joined == joined));
}


@override
int get hashCode => Object.hash(runtimeType,joined);

@override
String toString() {
  return 'JoinResult.complete(joined: $joined)';
}


}

/// @nodoc
abstract mixin class $JoinResult_CompleteCopyWith<$Res> implements $JoinResultCopyWith<$Res> {
  factory $JoinResult_CompleteCopyWith(JoinResult_Complete value, $Res Function(JoinResult_Complete) _then) = _$JoinResult_CompleteCopyWithImpl;
@useResult
$Res call({
 Joined joined
});




}
/// @nodoc
class _$JoinResult_CompleteCopyWithImpl<$Res>
    implements $JoinResult_CompleteCopyWith<$Res> {
  _$JoinResult_CompleteCopyWithImpl(this._self, this._then);

  final JoinResult_Complete _self;
  final $Res Function(JoinResult_Complete) _then;

/// Create a copy of JoinResult
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? joined = null,}) {
  return _then(JoinResult_Complete(
joined: null == joined ? _self.joined : joined // ignore: cast_nullable_to_non_nullable
as Joined,
  ));
}


}

// dart format on
