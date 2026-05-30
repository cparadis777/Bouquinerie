//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:api_client/src/model/series.dart';
import 'package:api_client/src/model/author.dart';
import 'package:api_client/src/model/book.dart';
import 'package:api_client/src/model/identifier.dart';
import 'package:copy_with_extension/copy_with_extension.dart';
import 'package:json_annotation/json_annotation.dart';

part 'book_response.g.dart';


@CopyWith()
@JsonSerializable(
  checked: true,
  createToJson: true,
  disallowUnrecognizedKeys: false,
  explicitToJson: true,
)
class BookResponse {
  /// Returns a new [BookResponse] instance.
  BookResponse({

    required  this.authors,

    required  this.book,

    required  this.identifiers,

    required  this.series,
  });

  @JsonKey(
    
    name: r'authors',
    required: true,
    includeIfNull: false,
  )


  final List<Author> authors;



  @JsonKey(
    
    name: r'book',
    required: true,
    includeIfNull: false,
  )


  final Book book;



  @JsonKey(
    
    name: r'identifiers',
    required: true,
    includeIfNull: false,
  )


  final List<Identifier> identifiers;



  @JsonKey(
    
    name: r'series',
    required: true,
    includeIfNull: false,
  )


  final List<Series> series;





    @override
    bool operator ==(Object other) => identical(this, other) || other is BookResponse &&
      other.authors == authors &&
      other.book == book &&
      other.identifiers == identifiers &&
      other.series == series;

    @override
    int get hashCode =>
        authors.hashCode +
        book.hashCode +
        identifiers.hashCode +
        series.hashCode;

  factory BookResponse.fromJson(Map<String, dynamic> json) => _$BookResponseFromJson(json);

  Map<String, dynamic> toJson() => _$BookResponseToJson(this);

  @override
  String toString() {
    return toJson().toString();
  }

}

