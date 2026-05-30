// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'series.dart';

// **************************************************************************
// CopyWithGenerator
// **************************************************************************

abstract class _$SeriesCWProxy {
  Series createdAt(DateTime createdAt);

  Series description(String description);

  Series id(String id);

  Series name(String name);

  Series updatedAt(DateTime updatedAt);

  /// This function **does support** nullification of nullable fields. All `null` values passed to `non-nullable` fields will be ignored. You can also use `Series(...).copyWith.fieldName(...)` to override fields one at a time with nullification support.
  ///
  /// Usage
  /// ```dart
  /// Series(...).copyWith(id: 12, name: "My name")
  /// ````
  Series call({
    DateTime createdAt,
    String description,
    String id,
    String name,
    DateTime updatedAt,
  });
}

/// Proxy class for `copyWith` functionality. This is a callable class and can be used as follows: `instanceOfSeries.copyWith(...)`. Additionally contains functions for specific fields e.g. `instanceOfSeries.copyWith.fieldName(...)`
class _$SeriesCWProxyImpl implements _$SeriesCWProxy {
  const _$SeriesCWProxyImpl(this._value);

  final Series _value;

  @override
  Series createdAt(DateTime createdAt) => this(createdAt: createdAt);

  @override
  Series description(String description) => this(description: description);

  @override
  Series id(String id) => this(id: id);

  @override
  Series name(String name) => this(name: name);

  @override
  Series updatedAt(DateTime updatedAt) => this(updatedAt: updatedAt);

  @override
  /// This function **does support** nullification of nullable fields. All `null` values passed to `non-nullable` fields will be ignored. You can also use `Series(...).copyWith.fieldName(...)` to override fields one at a time with nullification support.
  ///
  /// Usage
  /// ```dart
  /// Series(...).copyWith(id: 12, name: "My name")
  /// ````
  Series call({
    Object? createdAt = const $CopyWithPlaceholder(),
    Object? description = const $CopyWithPlaceholder(),
    Object? id = const $CopyWithPlaceholder(),
    Object? name = const $CopyWithPlaceholder(),
    Object? updatedAt = const $CopyWithPlaceholder(),
  }) {
    return Series(
      createdAt: createdAt == const $CopyWithPlaceholder()
          ? _value.createdAt
          // ignore: cast_nullable_to_non_nullable
          : createdAt as DateTime,
      description: description == const $CopyWithPlaceholder()
          ? _value.description
          // ignore: cast_nullable_to_non_nullable
          : description as String,
      id: id == const $CopyWithPlaceholder()
          ? _value.id
          // ignore: cast_nullable_to_non_nullable
          : id as String,
      name: name == const $CopyWithPlaceholder()
          ? _value.name
          // ignore: cast_nullable_to_non_nullable
          : name as String,
      updatedAt: updatedAt == const $CopyWithPlaceholder()
          ? _value.updatedAt
          // ignore: cast_nullable_to_non_nullable
          : updatedAt as DateTime,
    );
  }
}

extension $SeriesCopyWith on Series {
  /// Returns a callable class that can be used as follows: `instanceOfSeries.copyWith(...)` or like so:`instanceOfSeries.copyWith.fieldName(...)`.
  // ignore: library_private_types_in_public_api
  _$SeriesCWProxy get copyWith => _$SeriesCWProxyImpl(this);
}

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Series _$SeriesFromJson(Map<String, dynamic> json) => $checkedCreate(
  'Series',
  json,
  ($checkedConvert) {
    $checkKeys(
      json,
      requiredKeys: const [
        'created_at',
        'description',
        'id',
        'name',
        'updated_at',
      ],
    );
    final val = Series(
      createdAt: $checkedConvert(
        'created_at',
        (v) => DateTime.parse(v as String),
      ),
      description: $checkedConvert('description', (v) => v as String),
      id: $checkedConvert('id', (v) => v as String),
      name: $checkedConvert('name', (v) => v as String),
      updatedAt: $checkedConvert(
        'updated_at',
        (v) => DateTime.parse(v as String),
      ),
    );
    return val;
  },
  fieldKeyMap: const {'createdAt': 'created_at', 'updatedAt': 'updated_at'},
);

Map<String, dynamic> _$SeriesToJson(Series instance) => <String, dynamic>{
  'created_at': instance.createdAt.toIso8601String(),
  'description': instance.description,
  'id': instance.id,
  'name': instance.name,
  'updated_at': instance.updatedAt.toIso8601String(),
};
