# api_client.api.HandlersSeriesApi

## Load the API package
```dart
import 'package:api_client/api.dart';
```

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**listSeries**](HandlersSeriesApi.md#listseries) | **GET** /api/series | 


# **listSeries**
> SeriesListResponse listSeries(page, pageSize)



### Example
```dart
import 'package:api_client/api.dart';

final api = ApiClient().getHandlersSeriesApi();
final int page = 789; // int | 
final int pageSize = 789; // int | 

try {
    final response = api.listSeries(page, pageSize);
    print(response);
} on DioException catch (e) {
    print('Exception when calling HandlersSeriesApi->listSeries: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **page** | **int**|  | [optional] 
 **pageSize** | **int**|  | [optional] 

### Return type

[**SeriesListResponse**](SeriesListResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

