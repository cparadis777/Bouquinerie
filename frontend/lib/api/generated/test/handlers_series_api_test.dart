import 'package:test/test.dart';
import 'package:api_client/api_client.dart';


/// tests for HandlersSeriesApi
void main() {
  final instance = ApiClient().getHandlersSeriesApi();

  group(HandlersSeriesApi, () {
    //Future<SeriesListResponse> listSeries({ int page, int pageSize }) async
    test('test listSeries', () async {
      // TODO
    });

  });
}
