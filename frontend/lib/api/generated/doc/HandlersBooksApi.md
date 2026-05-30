# api_client.api.HandlersBooksApi

## Load the API package
```dart
import 'package:api_client/api.dart';
```

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**getBook**](HandlersBooksApi.md#getbook) | **GET** /api/books/{id} | 
[**listBooks**](HandlersBooksApi.md#listbooks) | **GET** /api/books | 


# **getBook**
> BookResponse getBook(id)



### Example
```dart
import 'package:api_client/api.dart';

final api = ApiClient().getHandlersBooksApi();
final String id = 38400000-8cf0-11bd-b23e-10b96e4ef00d; // String | 

try {
    final response = api.getBook(id);
    print(response);
} on DioException catch (e) {
    print('Exception when calling HandlersBooksApi->getBook: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**|  | 

### Return type

[**BookResponse**](BookResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listBooks**
> BookListResponse listBooks(page, pageSize)



### Example
```dart
import 'package:api_client/api.dart';

final api = ApiClient().getHandlersBooksApi();
final int page = 789; // int | 
final int pageSize = 789; // int | 

try {
    final response = api.listBooks(page, pageSize);
    print(response);
} on DioException catch (e) {
    print('Exception when calling HandlersBooksApi->listBooks: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **page** | **int**|  | [optional] 
 **pageSize** | **int**|  | [optional] 

### Return type

[**BookListResponse**](BookListResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

