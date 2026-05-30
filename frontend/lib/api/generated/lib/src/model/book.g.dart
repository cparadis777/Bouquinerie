// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'book.dart';

// **************************************************************************
// CopyWithGenerator
// **************************************************************************

abstract class _$BookCWProxy {
  Book coverPath(String coverPath);

  Book createdAt(DateTime createdAt);

  Book description(String description);

  Book id(String id);

  Book isbn(String isbn);

  Book language(String language);

  Book pageCount(int pageCount);

  Book publishedDate(DateTime publishedDate);

  Book publisher(String publisher);

  Book sortTitle(String sortTitle);

  Book subtitle(String subtitle);

  Book title(String title);

  Book updatedAt(DateTime updatedAt);

  /// This function **does support** nullification of nullable fields. All `null` values passed to `non-nullable` fields will be ignored. You can also use `Book(...).copyWith.fieldName(...)` to override fields one at a time with nullification support.
  ///
  /// Usage
  /// ```dart
  /// Book(...).copyWith(id: 12, name: "My name")
  /// ````
  Book call({
    String coverPath,
    DateTime createdAt,
    String description,
    String id,
    String isbn,
    String language,
    int pageCount,
    DateTime publishedDate,
    String publisher,
    String sortTitle,
    String subtitle,
    String title,
    DateTime updatedAt,
  });
}

/// Proxy class for `copyWith` functionality. This is a callable class and can be used as follows: `instanceOfBook.copyWith(...)`. Additionally contains functions for specific fields e.g. `instanceOfBook.copyWith.fieldName(...)`
class _$BookCWProxyImpl implements _$BookCWProxy {
  const _$BookCWProxyImpl(this._value);

  final Book _value;

  @override
  Book coverPath(String coverPath) => this(coverPath: coverPath);

  @override
  Book createdAt(DateTime createdAt) => this(createdAt: createdAt);

  @override
  Book description(String description) => this(description: description);

  @override
  Book id(String id) => this(id: id);

  @override
  Book isbn(String isbn) => this(isbn: isbn);

  @override
  Book language(String language) => this(language: language);

  @override
  Book pageCount(int pageCount) => this(pageCount: pageCount);

  @override
  Book publishedDate(DateTime publishedDate) =>
      this(publishedDate: publishedDate);

  @override
  Book publisher(String publisher) => this(publisher: publisher);

  @override
  Book sortTitle(String sortTitle) => this(sortTitle: sortTitle);

  @override
  Book subtitle(String subtitle) => this(subtitle: subtitle);

  @override
  Book title(String title) => this(title: title);

  @override
  Book updatedAt(DateTime updatedAt) => this(updatedAt: updatedAt);

  @override
  /// This function **does support** nullification of nullable fields. All `null` values passed to `non-nullable` fields will be ignored. You can also use `Book(...).copyWith.fieldName(...)` to override fields one at a time with nullification support.
  ///
  /// Usage
  /// ```dart
  /// Book(...).copyWith(id: 12, name: "My name")
  /// ````
  Book call({
    Object? coverPath = const $CopyWithPlaceholder(),
    Object? createdAt = const $CopyWithPlaceholder(),
    Object? description = const $CopyWithPlaceholder(),
    Object? id = const $CopyWithPlaceholder(),
    Object? isbn = const $CopyWithPlaceholder(),
    Object? language = const $CopyWithPlaceholder(),
    Object? pageCount = const $CopyWithPlaceholder(),
    Object? publishedDate = const $CopyWithPlaceholder(),
    Object? publisher = const $CopyWithPlaceholder(),
    Object? sortTitle = const $CopyWithPlaceholder(),
    Object? subtitle = const $CopyWithPlaceholder(),
    Object? title = const $CopyWithPlaceholder(),
    Object? updatedAt = const $CopyWithPlaceholder(),
  }) {
    return Book(
      coverPath: coverPath == const $CopyWithPlaceholder()
          ? _value.coverPath
          // ignore: cast_nullable_to_non_nullable
          : coverPath as String,
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
      isbn: isbn == const $CopyWithPlaceholder()
          ? _value.isbn
          // ignore: cast_nullable_to_non_nullable
          : isbn as String,
      language: language == const $CopyWithPlaceholder()
          ? _value.language
          // ignore: cast_nullable_to_non_nullable
          : language as String,
      pageCount: pageCount == const $CopyWithPlaceholder()
          ? _value.pageCount
          // ignore: cast_nullable_to_non_nullable
          : pageCount as int,
      publishedDate: publishedDate == const $CopyWithPlaceholder()
          ? _value.publishedDate
          // ignore: cast_nullable_to_non_nullable
          : publishedDate as DateTime,
      publisher: publisher == const $CopyWithPlaceholder()
          ? _value.publisher
          // ignore: cast_nullable_to_non_nullable
          : publisher as String,
      sortTitle: sortTitle == const $CopyWithPlaceholder()
          ? _value.sortTitle
          // ignore: cast_nullable_to_non_nullable
          : sortTitle as String,
      subtitle: subtitle == const $CopyWithPlaceholder()
          ? _value.subtitle
          // ignore: cast_nullable_to_non_nullable
          : subtitle as String,
      title: title == const $CopyWithPlaceholder()
          ? _value.title
          // ignore: cast_nullable_to_non_nullable
          : title as String,
      updatedAt: updatedAt == const $CopyWithPlaceholder()
          ? _value.updatedAt
          // ignore: cast_nullable_to_non_nullable
          : updatedAt as DateTime,
    );
  }
}

extension $BookCopyWith on Book {
  /// Returns a callable class that can be used as follows: `instanceOfBook.copyWith(...)` or like so:`instanceOfBook.copyWith.fieldName(...)`.
  // ignore: library_private_types_in_public_api
  _$BookCWProxy get copyWith => _$BookCWProxyImpl(this);
}

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Book _$BookFromJson(Map<String, dynamic> json) => $checkedCreate(
  'Book',
  json,
  ($checkedConvert) {
    $checkKeys(
      json,
      requiredKeys: const [
        'cover_path',
        'created_at',
        'description',
        'id',
        'isbn',
        'language',
        'page_count',
        'published_date',
        'publisher',
        'sort_title',
        'subtitle',
        'title',
        'updated_at',
      ],
    );
    final val = Book(
      coverPath: $checkedConvert('cover_path', (v) => v as String),
      createdAt: $checkedConvert(
        'created_at',
        (v) => DateTime.parse(v as String),
      ),
      description: $checkedConvert('description', (v) => v as String),
      id: $checkedConvert('id', (v) => v as String),
      isbn: $checkedConvert('isbn', (v) => v as String),
      language: $checkedConvert('language', (v) => v as String),
      pageCount: $checkedConvert('page_count', (v) => (v as num).toInt()),
      publishedDate: $checkedConvert(
        'published_date',
        (v) => DateTime.parse(v as String),
      ),
      publisher: $checkedConvert('publisher', (v) => v as String),
      sortTitle: $checkedConvert('sort_title', (v) => v as String),
      subtitle: $checkedConvert('subtitle', (v) => v as String),
      title: $checkedConvert('title', (v) => v as String),
      updatedAt: $checkedConvert(
        'updated_at',
        (v) => DateTime.parse(v as String),
      ),
    );
    return val;
  },
  fieldKeyMap: const {
    'coverPath': 'cover_path',
    'createdAt': 'created_at',
    'pageCount': 'page_count',
    'publishedDate': 'published_date',
    'sortTitle': 'sort_title',
    'updatedAt': 'updated_at',
  },
);

Map<String, dynamic> _$BookToJson(Book instance) => <String, dynamic>{
  'cover_path': instance.coverPath,
  'created_at': instance.createdAt.toIso8601String(),
  'description': instance.description,
  'id': instance.id,
  'isbn': instance.isbn,
  'language': instance.language,
  'page_count': instance.pageCount,
  'published_date': instance.publishedDate.toIso8601String(),
  'publisher': instance.publisher,
  'sort_title': instance.sortTitle,
  'subtitle': instance.subtitle,
  'title': instance.title,
  'updated_at': instance.updatedAt.toIso8601String(),
};
