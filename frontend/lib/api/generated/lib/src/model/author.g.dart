// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'author.dart';

// **************************************************************************
// CopyWithGenerator
// **************************************************************************

abstract class _$AuthorCWProxy {
  Author id(String id);

  Author name(String name);

  Author sortName(String sortName);

  /// This function **does support** nullification of nullable fields. All `null` values passed to `non-nullable` fields will be ignored. You can also use `Author(...).copyWith.fieldName(...)` to override fields one at a time with nullification support.
  ///
  /// Usage
  /// ```dart
  /// Author(...).copyWith(id: 12, name: "My name")
  /// ````
  Author call({String id, String name, String sortName});
}

/// Proxy class for `copyWith` functionality. This is a callable class and can be used as follows: `instanceOfAuthor.copyWith(...)`. Additionally contains functions for specific fields e.g. `instanceOfAuthor.copyWith.fieldName(...)`
class _$AuthorCWProxyImpl implements _$AuthorCWProxy {
  const _$AuthorCWProxyImpl(this._value);

  final Author _value;

  @override
  Author id(String id) => this(id: id);

  @override
  Author name(String name) => this(name: name);

  @override
  Author sortName(String sortName) => this(sortName: sortName);

  @override
  /// This function **does support** nullification of nullable fields. All `null` values passed to `non-nullable` fields will be ignored. You can also use `Author(...).copyWith.fieldName(...)` to override fields one at a time with nullification support.
  ///
  /// Usage
  /// ```dart
  /// Author(...).copyWith(id: 12, name: "My name")
  /// ````
  Author call({
    Object? id = const $CopyWithPlaceholder(),
    Object? name = const $CopyWithPlaceholder(),
    Object? sortName = const $CopyWithPlaceholder(),
  }) {
    return Author(
      id: id == const $CopyWithPlaceholder()
          ? _value.id
          // ignore: cast_nullable_to_non_nullable
          : id as String,
      name: name == const $CopyWithPlaceholder()
          ? _value.name
          // ignore: cast_nullable_to_non_nullable
          : name as String,
      sortName: sortName == const $CopyWithPlaceholder()
          ? _value.sortName
          // ignore: cast_nullable_to_non_nullable
          : sortName as String,
    );
  }
}

extension $AuthorCopyWith on Author {
  /// Returns a callable class that can be used as follows: `instanceOfAuthor.copyWith(...)` or like so:`instanceOfAuthor.copyWith.fieldName(...)`.
  // ignore: library_private_types_in_public_api
  _$AuthorCWProxy get copyWith => _$AuthorCWProxyImpl(this);
}

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Author _$AuthorFromJson(Map<String, dynamic> json) =>
    $checkedCreate('Author', json, ($checkedConvert) {
      $checkKeys(json, requiredKeys: const ['id', 'name', 'sort_name']);
      final val = Author(
        id: $checkedConvert('id', (v) => v as String),
        name: $checkedConvert('name', (v) => v as String),
        sortName: $checkedConvert('sort_name', (v) => v as String),
      );
      return val;
    }, fieldKeyMap: const {'sortName': 'sort_name'});

Map<String, dynamic> _$AuthorToJson(Author instance) => <String, dynamic>{
  'id': instance.id,
  'name': instance.name,
  'sort_name': instance.sortName,
};
