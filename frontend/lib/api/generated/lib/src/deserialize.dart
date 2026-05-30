import 'package:api_client/src/model/author.dart';
import 'package:api_client/src/model/author_list_response.dart';
import 'package:api_client/src/model/book.dart';
import 'package:api_client/src/model/book_list_response.dart';
import 'package:api_client/src/model/book_response.dart';
import 'package:api_client/src/model/identifier.dart';
import 'package:api_client/src/model/series.dart';
import 'package:api_client/src/model/series_list_response.dart';

final _regList = RegExp(r'^List<(.*)>$');
final _regSet = RegExp(r'^Set<(.*)>$');
final _regMap = RegExp(r'^Map<String,(.*)>$');

  ReturnType deserialize<ReturnType, BaseType>(dynamic value, String targetType, {bool growable= true}) {
      switch (targetType) {
        case 'String':
          return '$value' as ReturnType;
        case 'int':
          return (value is int ? value : int.parse('$value')) as ReturnType;
        case 'bool':
          if (value is bool) {
            return value as ReturnType;
          }
          final valueString = '$value'.toLowerCase();
          return (valueString == 'true' || valueString == '1') as ReturnType;
        case 'double':
          return (value is double ? value : double.parse('$value')) as ReturnType;
        case 'Author':
          return Author.fromJson(value as Map<String, dynamic>) as ReturnType;
        case 'AuthorListResponse':
          return AuthorListResponse.fromJson(value as Map<String, dynamic>) as ReturnType;
        case 'Book':
          return Book.fromJson(value as Map<String, dynamic>) as ReturnType;
        case 'BookListResponse':
          return BookListResponse.fromJson(value as Map<String, dynamic>) as ReturnType;
        case 'BookResponse':
          return BookResponse.fromJson(value as Map<String, dynamic>) as ReturnType;
        case 'Identifier':
          return Identifier.fromJson(value as Map<String, dynamic>) as ReturnType;
        case 'Series':
          return Series.fromJson(value as Map<String, dynamic>) as ReturnType;
        case 'SeriesListResponse':
          return SeriesListResponse.fromJson(value as Map<String, dynamic>) as ReturnType;
        default:
          RegExpMatch? match;

          if (value is List && (match = _regList.firstMatch(targetType)) != null) {
            targetType = match![1]!; // ignore: parameter_assignments
            return value
              .map<BaseType>((dynamic v) => deserialize<BaseType, BaseType>(v, targetType, growable: growable))
              .toList(growable: growable) as ReturnType;
          }
          if (value is Set && (match = _regSet.firstMatch(targetType)) != null) {
            targetType = match![1]!; // ignore: parameter_assignments
            return value
              .map<BaseType>((dynamic v) => deserialize<BaseType, BaseType>(v, targetType, growable: growable))
              .toSet() as ReturnType;
          }
          if (value is Map && (match = _regMap.firstMatch(targetType)) != null) {
            targetType = match![1]!.trim(); // ignore: parameter_assignments
            return Map<String, BaseType>.fromIterables(
              value.keys as Iterable<String>,
              value.values.map((dynamic v) => deserialize<BaseType, BaseType>(v, targetType, growable: growable)),
            ) as ReturnType;
          }
          break;
    }
    throw Exception('Cannot deserialize');
  }