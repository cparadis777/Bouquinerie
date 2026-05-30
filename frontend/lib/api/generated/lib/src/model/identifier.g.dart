// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'identifier.dart';

// **************************************************************************
// CopyWithGenerator
// **************************************************************************

abstract class _$IdentifierCWProxy {
  Identifier bookId(String bookId);

  Identifier id(String id);

  Identifier source_(String source_);

  Identifier value(String value);

  /// This function **does support** nullification of nullable fields. All `null` values passed to `non-nullable` fields will be ignored. You can also use `Identifier(...).copyWith.fieldName(...)` to override fields one at a time with nullification support.
  ///
  /// Usage
  /// ```dart
  /// Identifier(...).copyWith(id: 12, name: "My name")
  /// ````
  Identifier call({String bookId, String id, String source_, String value});
}

/// Proxy class for `copyWith` functionality. This is a callable class and can be used as follows: `instanceOfIdentifier.copyWith(...)`. Additionally contains functions for specific fields e.g. `instanceOfIdentifier.copyWith.fieldName(...)`
class _$IdentifierCWProxyImpl implements _$IdentifierCWProxy {
  const _$IdentifierCWProxyImpl(this._value);

  final Identifier _value;

  @override
  Identifier bookId(String bookId) => this(bookId: bookId);

  @override
  Identifier id(String id) => this(id: id);

  @override
  Identifier source_(String source_) => this(source_: source_);

  @override
  Identifier value(String value) => this(value: value);

  @override
  /// This function **does support** nullification of nullable fields. All `null` values passed to `non-nullable` fields will be ignored. You can also use `Identifier(...).copyWith.fieldName(...)` to override fields one at a time with nullification support.
  ///
  /// Usage
  /// ```dart
  /// Identifier(...).copyWith(id: 12, name: "My name")
  /// ````
  Identifier call({
    Object? bookId = const $CopyWithPlaceholder(),
    Object? id = const $CopyWithPlaceholder(),
    Object? source_ = const $CopyWithPlaceholder(),
    Object? value = const $CopyWithPlaceholder(),
  }) {
    return Identifier(
      bookId: bookId == const $CopyWithPlaceholder()
          ? _value.bookId
          // ignore: cast_nullable_to_non_nullable
          : bookId as String,
      id: id == const $CopyWithPlaceholder()
          ? _value.id
          // ignore: cast_nullable_to_non_nullable
          : id as String,
      source_: source_ == const $CopyWithPlaceholder()
          ? _value.source_
          // ignore: cast_nullable_to_non_nullable
          : source_ as String,
      value: value == const $CopyWithPlaceholder()
          ? _value.value
          // ignore: cast_nullable_to_non_nullable
          : value as String,
    );
  }
}

extension $IdentifierCopyWith on Identifier {
  /// Returns a callable class that can be used as follows: `instanceOfIdentifier.copyWith(...)` or like so:`instanceOfIdentifier.copyWith.fieldName(...)`.
  // ignore: library_private_types_in_public_api
  _$IdentifierCWProxy get copyWith => _$IdentifierCWProxyImpl(this);
}

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Identifier _$IdentifierFromJson(Map<String, dynamic> json) => $checkedCreate(
  'Identifier',
  json,
  ($checkedConvert) {
    $checkKeys(json, requiredKeys: const ['book_id', 'id', 'source', 'value']);
    final val = Identifier(
      bookId: $checkedConvert('book_id', (v) => v as String),
      id: $checkedConvert('id', (v) => v as String),
      source_: $checkedConvert('source', (v) => v as String),
      value: $checkedConvert('value', (v) => v as String),
    );
    return val;
  },
  fieldKeyMap: const {'bookId': 'book_id', 'source_': 'source'},
);

Map<String, dynamic> _$IdentifierToJson(Identifier instance) =>
    <String, dynamic>{
      'book_id': instance.bookId,
      'id': instance.id,
      'source': instance.source_,
      'value': instance.value,
    };
