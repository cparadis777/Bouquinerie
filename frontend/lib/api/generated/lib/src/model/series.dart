//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:copy_with_extension/copy_with_extension.dart';
import 'package:json_annotation/json_annotation.dart';

part 'series.g.dart';


@CopyWith()
@JsonSerializable(
  checked: true,
  createToJson: true,
  disallowUnrecognizedKeys: false,
  explicitToJson: true,
)
class Series {
  /// Returns a new [Series] instance.
  Series({

    required  this.createdAt,

    required  this.description,

    required  this.id,

    required  this.name,

    required  this.updatedAt,
  });

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
    
    name: r'name',
    required: true,
    includeIfNull: false,
  )


  final String name;



  @JsonKey(
    
    name: r'updated_at',
    required: true,
    includeIfNull: false,
  )


  final DateTime updatedAt;





    @override
    bool operator ==(Object other) => identical(this, other) || other is Series &&
      other.createdAt == createdAt &&
      other.description == description &&
      other.id == id &&
      other.name == name &&
      other.updatedAt == updatedAt;

    @override
    int get hashCode =>
        createdAt.hashCode +
        description.hashCode +
        id.hashCode +
        name.hashCode +
        updatedAt.hashCode;

  factory Series.fromJson(Map<String, dynamic> json) => _$SeriesFromJson(json);

  Map<String, dynamic> toJson() => _$SeriesToJson(this);

  @override
  String toString() {
    return toJson().toString();
  }

}

