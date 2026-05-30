//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:copy_with_extension/copy_with_extension.dart';
import 'package:json_annotation/json_annotation.dart';

part 'identifier.g.dart';


@CopyWith()
@JsonSerializable(
  checked: true,
  createToJson: true,
  disallowUnrecognizedKeys: false,
  explicitToJson: true,
)
class Identifier {
  /// Returns a new [Identifier] instance.
  Identifier({

    required  this.bookId,

    required  this.id,

    required  this.source_,

    required  this.value,
  });

  @JsonKey(
    
    name: r'book_id',
    required: true,
    includeIfNull: false,
  )


  final String bookId;



  @JsonKey(
    
    name: r'id',
    required: true,
    includeIfNull: false,
  )


  final String id;



  @JsonKey(
    
    name: r'source',
    required: true,
    includeIfNull: false,
  )


  final String source_;



  @JsonKey(
    
    name: r'value',
    required: true,
    includeIfNull: false,
  )


  final String value;





    @override
    bool operator ==(Object other) => identical(this, other) || other is Identifier &&
      other.bookId == bookId &&
      other.id == id &&
      other.source_ == source_ &&
      other.value == value;

    @override
    int get hashCode =>
        bookId.hashCode +
        id.hashCode +
        source_.hashCode +
        value.hashCode;

  factory Identifier.fromJson(Map<String, dynamic> json) => _$IdentifierFromJson(json);

  Map<String, dynamic> toJson() => _$IdentifierToJson(this);

  @override
  String toString() {
    return toJson().toString();
  }

}

