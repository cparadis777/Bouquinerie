import 'package:test/test.dart';
import 'package:api_client/api_client.dart';


/// tests for HandlersAuthorsApi
void main() {
  final instance = ApiClient().getHandlersAuthorsApi();

  group(HandlersAuthorsApi, () {
    //Future<AuthorListResponse> listAuthors({ int page, int pageSize }) async
    test('test listAuthors', () async {
      // TODO
    });

  });
}
