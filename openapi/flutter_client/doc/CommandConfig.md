# my_api_client.model.CommandConfig

## Load the model package
```dart
import 'package:my_api_client/api.dart';
```

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**exec** | **List<String>** | Shell command and arguments to execute. | [default to const []]
**stdoutSuccessMessage** | **String** |  | [optional] 
**hidden** | **bool** |  | [optional] [default to false]
**timeoutSeconds** | **int** |  | [optional] 
**cwd** | **String** | Override working directory for this command. Paths may be absolute or relative to the workspace root. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


