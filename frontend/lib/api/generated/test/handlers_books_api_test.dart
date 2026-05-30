import 'package:test/test.dart';
import 'package:api_client/api_client.dart';


/// tests for HandlersBooksApi
void main() {
  final instance = ApiClient().getHandlersBooksApi();

  group(HandlersBooksApi, () {
    //Future<BookResponse> getBook(String id) async
    test('test getBook', () async {
      // TODO
    });

    //Future<BookListResponse> listBooks({ int page, int pageSize }) async
    test('test listBooks', () async {
      // TODO
    });

  });
}
