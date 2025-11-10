# my_api_client.api.DefaultApi

## Load the API package
```dart
import 'package:my_api_client/api.dart';
```

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**getHealthz**](DefaultApi.md#gethealthz) | **GET** /healthz | Health check


# **getHealthz**
> GetHealthz200Response getHealthz()

Health check

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();

try {
    final result = api_instance.getHealthz();
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getHealthz: $e\n');
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

[**GetHealthz200Response**](GetHealthz200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

