//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:api_client/src/model/author.dart';
import 'package:copy_with_extension/copy_with_extension.dart';
import 'package:json_annotation/json_annotation.dart';

part 'author_list_response.g.dart';


@CopyWith()
@JsonSerializable(
  checked: true,
  createToJson: true,
  disallowUnrecognizedKeys: false,
  explicitToJson: true,
)
class AuthorListResponse {
  /// Returns a new [AuthorListResponse] instance.
  AuthorListResponse({

    required  this.data,

    required  this.page,

    required  this.pageSize,

    required  this.pages,

    required  this.total,
  });

  @JsonKey(
    
    name: r'data',
    required: true,
    includeIfNull: false,
  )


  final List<Author> data;



          // minimum: 0
  @JsonKey(
    
    name: r'page',
    required: true,
    includeIfNull: false,
  )


  final int page;



          // minimum: 0
  @JsonKey(
    
    name: r'page_size',
    required: true,
    includeIfNull: false,
  )


  final int pageSize;



          // minimum: 0
  @JsonKey(
    
    name: r'pages',
    required: true,
    includeIfNull: false,
  )


  final int pages;



          // minimum: 0
  @JsonKey(
    
    name: r'total',
    required: true,
    includeIfNull: false,
  )


  final int total;





    @override
    bool operator ==(Object other) => identical(this, other) || other is AuthorListResponse &&
      other.data == data &&
      other.page == page &&
      other.pageSize == pageSize &&
      other.pages == pages &&
      other.total == total;

    @override
    int get hashCode =>
        data.hashCode +
        page.hashCode +
        pageSize.hashCode +
        pages.hashCode +
        total.hashCode;

  factory AuthorListResponse.fromJson(Map<String, dynamic> json) => _$AuthorListResponseFromJson(json);

  Map<String, dynamic> toJson() => _$AuthorListResponseToJson(this);

  @override
  String toString() {
    return toJson().toString();
  }

}

