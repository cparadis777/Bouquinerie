//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:copy_with_extension/copy_with_extension.dart';
import 'package:json_annotation/json_annotation.dart';

part 'author.g.dart';


@CopyWith()
@JsonSerializable(
  checked: true,
  createToJson: true,
  disallowUnrecognizedKeys: false,
  explicitToJson: true,
)
class Author {
  /// Returns a new [Author] instance.
  Author({

    required  this.id,

    required  this.name,

    required  this.sortName,
  });

  @JsonKey(
    
    name: r'id',
    required: true,
    includeIfNull: false,
  )


  final String id;



  @JsonKey(
    
    name: r'name',
    required: true,
    includeIfNull: false,
  )


  final String name;



  @JsonKey(
    
    name: r'sort_name',
    required: true,
    includeIfNull: false,
  )


  final String sortName;





    @override
    bool operator ==(Object other) => identical(this, other) || other is Author &&
      other.id == id &&
      other.name == name &&
      other.sortName == sortName;

    @override
    int get hashCode =>
        id.hashCode +
        name.hashCode +
        sortName.hashCode;

  factory Author.fromJson(Map<String, dynamic> json) => _$AuthorFromJson(json);

  Map<String, dynamic> toJson() => _$AuthorToJson(this);

  @override
  String toString() {
    return toJson().toString();
  }

}

