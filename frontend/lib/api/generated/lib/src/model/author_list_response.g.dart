// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'author_list_response.dart';

// **************************************************************************
// CopyWithGenerator
// **************************************************************************

abstract class _$AuthorListResponseCWProxy {
  AuthorListResponse data(List<Author> data);

  AuthorListResponse page(int page);

  AuthorListResponse pageSize(int pageSize);

  AuthorListResponse pages(int pages);

  AuthorListResponse total(int total);

  /// This function **does support** nullification of nullable fields. All `null` values passed to `non-nullable` fields will be ignored. You can also use `AuthorListResponse(...).copyWith.fieldName(...)` to override fields one at a time with nullification support.
  ///
  /// Usage
  /// ```dart
  /// AuthorListResponse(...).copyWith(id: 12, name: "My name")
  /// ````
  AuthorListResponse call({
    List<Author> data,
    int page,
    int pageSize,
    int pages,
    int total,
  });
}

/// Proxy class for `copyWith` functionality. This is a callable class and can be used as follows: `instanceOfAuthorListResponse.copyWith(...)`. Additionally contains functions for specific fields e.g. `instanceOfAuthorListResponse.copyWith.fieldName(...)`
class _$AuthorListResponseCWProxyImpl implements _$AuthorListResponseCWProxy {
  const _$AuthorListResponseCWProxyImpl(this._value);

  final AuthorListResponse _value;

  @override
  AuthorListResponse data(List<Author> data) => this(data: data);

  @override
  AuthorListResponse page(int page) => this(page: page);

  @override
  AuthorListResponse pageSize(int pageSize) => this(pageSize: pageSize);

  @override
  AuthorListResponse pages(int pages) => this(pages: pages);

  @override
  AuthorListResponse total(int total) => this(total: total);

  @override
  /// This function **does support** nullification of nullable fields. All `null` values passed to `non-nullable` fields will be ignored. You can also use `AuthorListResponse(...).copyWith.fieldName(...)` to override fields one at a time with nullification support.
  ///
  /// Usage
  /// ```dart
  /// AuthorListResponse(...).copyWith(id: 12, name: "My name")
  /// ````
  AuthorListResponse call({
    Object? data = const $CopyWithPlaceholder(),
    Object? page = const $CopyWithPlaceholder(),
    Object? pageSize = const $CopyWithPlaceholder(),
    Object? pages = const $CopyWithPlaceholder(),
    Object? total = const $CopyWithPlaceholder(),
  }) {
    return AuthorListResponse(
      data: data == const $CopyWithPlaceholder()
          ? _value.data
          // ignore: cast_nullable_to_non_nullable
          : data as List<Author>,
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

extension $AuthorListResponseCopyWith on AuthorListResponse {
  /// Returns a callable class that can be used as follows: `instanceOfAuthorListResponse.copyWith(...)` or like so:`instanceOfAuthorListResponse.copyWith.fieldName(...)`.
  // ignore: library_private_types_in_public_api
  _$AuthorListResponseCWProxy get copyWith =>
      _$AuthorListResponseCWProxyImpl(this);
}

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

AuthorListResponse _$AuthorListResponseFromJson(Map<String, dynamic> json) =>
    $checkedCreate('AuthorListResponse', json, ($checkedConvert) {
      $checkKeys(
        json,
        requiredKeys: const ['data', 'page', 'page_size', 'pages', 'total'],
      );
      final val = AuthorListResponse(
        data: $checkedConvert(
          'data',
          (v) => (v as List<dynamic>)
              .map((e) => Author.fromJson(e as Map<String, dynamic>))
              .toList(),
        ),
        page: $checkedConvert('page', (v) => (v as num).toInt()),
        pageSize: $checkedConvert('page_size', (v) => (v as num).toInt()),
        pages: $checkedConvert('pages', (v) => (v as num).toInt()),
        total: $checkedConvert('total', (v) => (v as num).toInt()),
      );
      return val;
    }, fieldKeyMap: const {'pageSize': 'page_size'});

Map<String, dynamic> _$AuthorListResponseToJson(AuthorListResponse instance) =>
    <String, dynamic>{
      'data': instance.data.map((e) => e.toJson()).toList(),
      'page': instance.page,
      'page_size': instance.pageSize,
      'pages': instance.pages,
      'total': instance.total,
    };
