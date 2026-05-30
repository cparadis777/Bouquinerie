// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'series_list_response.dart';

// **************************************************************************
// CopyWithGenerator
// **************************************************************************

abstract class _$SeriesListResponseCWProxy {
  SeriesListResponse data(List<Series> data);

  SeriesListResponse page(int page);

  SeriesListResponse pageSize(int pageSize);

  SeriesListResponse pages(int pages);

  SeriesListResponse total(int total);

  /// This function **does support** nullification of nullable fields. All `null` values passed to `non-nullable` fields will be ignored. You can also use `SeriesListResponse(...).copyWith.fieldName(...)` to override fields one at a time with nullification support.
  ///
  /// Usage
  /// ```dart
  /// SeriesListResponse(...).copyWith(id: 12, name: "My name")
  /// ````
  SeriesListResponse call({
    List<Series> data,
    int page,
    int pageSize,
    int pages,
    int total,
  });
}

/// Proxy class for `copyWith` functionality. This is a callable class and can be used as follows: `instanceOfSeriesListResponse.copyWith(...)`. Additionally contains functions for specific fields e.g. `instanceOfSeriesListResponse.copyWith.fieldName(...)`
class _$SeriesListResponseCWProxyImpl implements _$SeriesListResponseCWProxy {
  const _$SeriesListResponseCWProxyImpl(this._value);

  final SeriesListResponse _value;

  @override
  SeriesListResponse data(List<Series> data) => this(data: data);

  @override
  SeriesListResponse page(int page) => this(page: page);

  @override
  SeriesListResponse pageSize(int pageSize) => this(pageSize: pageSize);

  @override
  SeriesListResponse pages(int pages) => this(pages: pages);

  @override
  SeriesListResponse total(int total) => this(total: total);

  @override
  /// This function **does support** nullification of nullable fields. All `null` values passed to `non-nullable` fields will be ignored. You can also use `SeriesListResponse(...).copyWith.fieldName(...)` to override fields one at a time with nullification support.
  ///
  /// Usage
  /// ```dart
  /// SeriesListResponse(...).copyWith(id: 12, name: "My name")
  /// ````
  SeriesListResponse call({
    Object? data = const $CopyWithPlaceholder(),
    Object? page = const $CopyWithPlaceholder(),
    Object? pageSize = const $CopyWithPlaceholder(),
    Object? pages = const $CopyWithPlaceholder(),
    Object? total = const $CopyWithPlaceholder(),
  }) {
    return SeriesListResponse(
      data: data == const $CopyWithPlaceholder()
          ? _value.data
          // ignore: cast_nullable_to_non_nullable
          : data as List<Series>,
      page: page == const $CopyWithPlaceholder()
          ? _value.page
          // ignore: cast_nullable_to_non_nullable
          : page as int,
      pageSize: pageSize == const $CopyWithPlaceholder()
          ? _value.pageSize
          // ignore: cast_nullable_to_non_nullable
          : pageSize as int,
      pages: pages == const $CopyWithPlaceholder()
          ? _value.pages
          // ignore: cast_nullable_to_non_nullable
          : pages as int,
      total: total == const $CopyWithPlaceholder()
          ? _value.total
          // ignore: cast_nullable_to_non_nullable
          : total as int,
    );
  }
}

extension $SeriesListResponseCopyWith on SeriesListResponse {
  /// Returns a callable class that can be used as follows: `instanceOfSeriesListResponse.copyWith(...)` or like so:`instanceOfSeriesListResponse.copyWith.fieldName(...)`.
  // ignore: library_private_types_in_public_api
  _$SeriesListResponseCWProxy get copyWith =>
      _$SeriesListResponseCWProxyImpl(this);
}

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SeriesListResponse _$SeriesListResponseFromJson(Map<String, dynamic> json) =>
    $checkedCreate('SeriesListResponse', json, ($checkedConvert) {
      $checkKeys(
        json,
        requiredKeys: const ['data', 'page', 'page_size', 'pages', 'total'],
      );
      final val = SeriesListResponse(
        data: $checkedConvert(
          'data',
          (v) => (v as List<dynamic>)
              .map((e) => Series.fromJson(e as Map<String, dynamic>))
              .toList(),
        ),
        page: $checkedConvert('page', (v) => (v as num).toInt()),
        pageSize: $checkedConvert('page_size', (v) => (v as num).toInt()),
        pages: $checkedConvert('pages', (v) => (v as num).toInt()),
        total: $checkedConvert('total', (v) => (v as num).toInt()),
      );
      return val;
    }, fieldKeyMap: const {'pageSize': 'page_size'});

Map<String, dynamic> _$SeriesListResponseToJson(SeriesListResponse instance) =>
    <String, dynamic>{
      'data': instance.data.map((e) => e.toJson()).toList(),
      'page': instance.page,
      'page_size': instance.pageSize,
      'pages': instance.pages,
      'total': instance.total,
    };
