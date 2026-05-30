// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'book_list_response.dart';

// **************************************************************************
// CopyWithGenerator
// **************************************************************************

abstract class _$BookListResponseCWProxy {
  BookListResponse data(List<Book> data);

  BookListResponse page(int page);

  BookListResponse pageSize(int pageSize);

  BookListResponse pages(int pages);

  BookListResponse total(int total);

  /// This function **does support** nullification of nullable fields. All `null` values passed to `non-nullable` fields will be ignored. You can also use `BookListResponse(...).copyWith.fieldName(...)` to override fields one at a time with nullification support.
  ///
  /// Usage
  /// ```dart
  /// BookListResponse(...).copyWith(id: 12, name: "My name")
  /// ````
  BookListResponse call({
    List<Book> data,
    int page,
    int pageSize,
    int pages,
    int total,
  });
}

/// Proxy class for `copyWith` functionality. This is a callable class and can be used as follows: `instanceOfBookListResponse.copyWith(...)`. Additionally contains functions for specific fields e.g. `instanceOfBookListResponse.copyWith.fieldName(...)`
class _$BookListResponseCWProxyImpl implements _$BookListResponseCWProxy {
  const _$BookListResponseCWProxyImpl(this._value);

  final BookListResponse _value;

  @override
  BookListResponse data(List<Book> data) => this(data: data);

  @override
  BookListResponse page(int page) => this(page: page);

  @override
  BookListResponse pageSize(int pageSize) => this(pageSize: pageSize);

  @override
  BookListResponse pages(int pages) => this(pages: pages);

  @override
  BookListResponse total(int total) => this(total: total);

  @override
  /// This function **does support** nullification of nullable fields. All `null` values passed to `non-nullable` fields will be ignored. You can also use `BookListResponse(...).copyWith.fieldName(...)` to override fields one at a time with nullification support.
  ///
  /// Usage
  /// ```dart
  /// BookListResponse(...).copyWith(id: 12, name: "My name")
  /// ````
  BookListResponse call({
    Object? data = const $CopyWithPlaceholder(),
    Object? page = const $CopyWithPlaceholder(),
    Object? pageSize = const $CopyWithPlaceholder(),
    Object? pages = const $CopyWithPlaceholder(),
    Object? total = const $CopyWithPlaceholder(),
  }) {
    return BookListResponse(
      data: data == const $CopyWithPlaceholder()
          ? _value.data
          // ignore: cast_nullable_to_non_nullable
          : data as List<Book>,
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

extension $BookListResponseCopyWith on BookListResponse {
  /// Returns a callable class that can be used as follows: `instanceOfBookListResponse.copyWith(...)` or like so:`instanceOfBookListResponse.copyWith.fieldName(...)`.
  // ignore: library_private_types_in_public_api
  _$BookListResponseCWProxy get copyWith => _$BookListResponseCWProxyImpl(this);
}

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

BookListResponse _$BookListResponseFromJson(Map<String, dynamic> json) =>
    $checkedCreate('BookListResponse', json, ($checkedConvert) {
      $checkKeys(
        json,
        requiredKeys: const ['data', 'page', 'page_size', 'pages', 'total'],
      );
      final val = BookListResponse(
        data: $checkedConvert(
          'data',
          (v) => (v as List<dynamic>)
              .map((e) => Book.fromJson(e as Map<String, dynamic>))
              .toList(),
        ),
        page: $checkedConvert('page', (v) => (v as num).toInt()),
        pageSize: $checkedConvert('page_size', (v) => (v as num).toInt()),
        pages: $checkedConvert('pages', (v) => (v as num).toInt()),
        total: $checkedConvert('total', (v) => (v as num).toInt()),
      );
      return val;
    }, fieldKeyMap: const {'pageSize': 'page_size'});

Map<String, dynamic> _$BookListResponseToJson(BookListResponse instance) =>
    <String, dynamic>{
      'data': instance.data.map((e) => e.toJson()).toList(),
      'page': instance.page,
      'page_size': instance.pageSize,
      'pages': instance.pages,
      'total': instance.total,
    };
