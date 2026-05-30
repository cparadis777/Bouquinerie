//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:copy_with_extension/copy_with_extension.dart';
import 'package:json_annotation/json_annotation.dart';

part 'book.g.dart';


@CopyWith()
@JsonSerializable(
  checked: true,
  createToJson: true,
  disallowUnrecognizedKeys: false,
  explicitToJson: true,
)
class Book {
  /// Returns a new [Book] instance.
  Book({

    required  this.coverPath,

    required  this.createdAt,

    required  this.description,

    required  this.id,

    required  this.isbn,

    required  this.language,

    required  this.pageCount,

    required  this.publishedDate,

    required  this.publisher,

    required  this.sortTitle,

    required  this.subtitle,

    required  this.title,

    required  this.updatedAt,
  });

  @JsonKey(
    
    name: r'cover_path',
    required: true,
    includeIfNull: false,
  )


  final String coverPath;



  @JsonKey(
    
    name: r'created_at',
    required: true,
    includeIfNull: false,
  )


  final DateTime createdAt;



  @JsonKey(
    
    name: r'description',
    required: true,
    includeIfNull: false,
  )


  final String description;



  @JsonKey(
    
    name: r'id',
    required: true,
    includeIfNull: false,
  )


  final String id;



  @JsonKey(
    
    name: r'isbn',
    required: true,
    includeIfNull: false,
  )


  final String isbn;



  @JsonKey(
    
    name: r'language',
    required: true,
    includeIfNull: false,
  )


  final String language;



  @JsonKey(
    
    name: r'page_count',
    required: true,
    includeIfNull: false,
  )


  final int pageCount;



  @JsonKey(
    
    name: r'published_date',
    required: true,
    includeIfNull: false,
  )


  final DateTime publishedDate;



  @JsonKey(
    
    name: r'publisher',
    required: true,
    includeIfNull: false,
  )


  final String publisher;



  @JsonKey(
    
    name: r'sort_title',
    required: true,
    includeIfNull: false,
  )


  final String sortTitle;



  @JsonKey(
    
    name: r'subtitle',
    required: true,
    includeIfNull: false,
  )


  final String subtitle;



  @JsonKey(
    
    name: r'title',
    required: true,
    includeIfNull: false,
  )


  final String title;



  @JsonKey(
    
    name: r'updated_at',
    required: true,
    includeIfNull: false,
  )


  final DateTime updatedAt;





    @override
    bool operator ==(Object other) => identical(this, other) || other is Book &&
      other.coverPath == coverPath &&
      other.createdAt == createdAt &&
      other.description == description &&
      other.id == id &&
      other.isbn == isbn &&
      other.language == language &&
      other.pageCount == pageCount &&
      other.publishedDate == publishedDate &&
      other.publisher == publisher &&
      other.sortTitle == sortTitle &&
      other.subtitle == subtitle &&
      other.title == title &&
      other.updatedAt == updatedAt;

    @override
    int get hashCode =>
        coverPath.hashCode +
        createdAt.hashCode +
        description.hashCode +
        id.hashCode +
        isbn.hashCode +
        language.hashCode +
        pageCount.hashCode +
        publishedDate.hashCode +
        publisher.hashCode +
        sortTitle.hashCode +
        subtitle.hashCode +
        title.hashCode +
        updatedAt.hashCode;

  factory Book.fromJson(Map<String, dynamic> json) => _$BookFromJson(json);

  Map<String, dynamic> toJson() => _$BookToJson(this);

  @override
  String toString() {
    return toJson().toString();
  }

}

