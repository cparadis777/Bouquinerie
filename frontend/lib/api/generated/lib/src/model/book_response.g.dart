// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'book_response.dart';

// **************************************************************************
// CopyWithGenerator
// **************************************************************************

abstract class _$BookResponseCWProxy {
  BookResponse authors(List<Author> authors);

  BookResponse book(Book book);

  BookResponse identifiers(List<Identifier> identifiers);

  BookResponse series(List<Series> series);

  /// This function **does support** nullification of nullable fields. All `null` values passed to `non-nullable` fields will be ignored. You can also use `BookResponse(...).copyWith.fieldName(...)` to override fields one at a time with nullification support.
  ///
  /// Usage
  /// ```dart
  /// BookResponse(...).copyWith(id: 12, name: "My name")
  /// ````
  BookResponse call({
    List<Author> authors,
    Book book,
    List<Identifier> identifiers,
    List<Series> series,
  });
}

/// Proxy class for `copyWith` functionality. This is a callable class and can be used as follows: `instanceOfBookResponse.copyWith(...)`. Additionally contains functions for specific fields e.g. `instanceOfBookResponse.copyWith.fieldName(...)`
class _$BookResponseCWProxyImpl implements _$BookResponseCWProxy {
  const _$BookResponseCWProxyImpl(this._value);

  final BookResponse _value;

  @override
  BookResponse authors(List<Author> authors) => this(authors: authors);

  @override
  BookResponse book(Book book) => this(book: book);

  @override
  BookResponse identifiers(List<Identifier> identifiers) =>
      this(identifiers: identifiers);

  @override
  BookResponse series(List<Series> series) => this(series: series);

  @override
  /// This function **does support** nullification of nullable fields. All `null` values passed to `non-nullable` fields will be ignored. You can also use `BookResponse(...).copyWith.fieldName(...)` to override fields one at a time with nullification support.
  ///
  /// Usage
  /// ```dart
  /// BookResponse(...).copyWith(id: 12, name: "My name")
  /// ````
  BookResponse call({
    Object? authors = const $CopyWithPlaceholder(),
    Object? book = const $CopyWithPlaceholder(),
    Object? identifiers = const $CopyWithPlaceholder(),
    Object? series = const $CopyWithPlaceholder(),
  }) {
    return BookResponse(
      authors: authors == const $CopyWithPlaceholder()
          ? _value.authors
          // ignore: cast_nullable_to_non_nullable
          : authors as List<Author>,
      book: book == const $CopyWithPlaceholder()
          ? _value.book
          // ignore: cast_nullable_to_non_nullable
          : book as Book,
      identifiers: identifiers == const $CopyWithPlaceholder()
          ? _value.identifiers
          // ignore: cast_nullable_to_non_nullable
          : identifiers as List<Identifier>,
      series: series == const $CopyWithPlaceholder()
          ? _value.series
          // ignore: cast_nullable_to_non_nullable
          : series as List<Series>,
    );
  }
}

extension $BookResponseCopyWith on BookResponse {
  /// Returns a callable class that can be used as follows: `instanceOfBookResponse.copyWith(...)` or like so:`instanceOfBookResponse.copyWith.fieldName(...)`.
  // ignore: library_private_types_in_public_api
  _$BookResponseCWProxy get copyWith => _$BookResponseCWProxyImpl(this);
}

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

BookResponse _$BookResponseFromJson(Map<String, dynamic> json) =>
    $checkedCreate('BookResponse', json, ($checkedConvert) {
      $checkKeys(
        json,
        requiredKeys: const ['authors', 'book', 'identifiers', 'series'],
      );
      final val = BookResponse(
        authors: $checkedConvert(
          'authors',
          (v) => (v as List<dynamic>)
              .map((e) => Author.fromJson(e as Map<String, dynamic>))
              .toList(),
        ),
        book: $checkedConvert(
          'book',
          (v) => Book.fromJson(v as Map<String, dynamic>),
        ),
        identifiers: $checkedConvert(
          'identifiers',
          (v) => (v as List<dynamic>)
              .map((e) => Identifier.fromJson(e as Map<String, dynamic>))
              .toList(),
        ),
        series: $checkedConvert(
          'series',
          (v) => (v as List<dynamic>)
              .map((e) => Series.fromJson(e as Map<String, dynamic>))
              .toList(),
        ),
      );
      return val;
    });

Map<String, dynamic> _$BookResponseToJson(BookResponse instance) =>
    <String, dynamic>{
      'authors': instance.authors.map((e) => e.toJson()).toList(),
      'book': instance.book.toJson(),
      'identifiers': instance.identifiers.map((e) => e.toJson()).toList(),
      'series': instance.series.map((e) => e.toJson()).toList(),
    };
