# my_api_client.model.TaskUpdateInput

## Load the model package
```dart
import 'package:my_api_client/api.dart';
```

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**groupId** | **int** |  | [optional] 
**slug** | **String** |  | [optional] 
**title** | **String** |  | [optional] 
**commitHash** | **String** | Commit hash associated with the task. | [optional] 
**status** | [**TaskStatus**](TaskStatus.md) |  | [optional] 
**owner** | **String** | Owner information encoded as display text (\"Orchestrator\", \"Quality Assurance\", or worker handles like \"ws42\"). | [optional] 
**description** | **String** | Detailed description of the task. | [optional] 
**modelOverride** | **String** |  | [optional] 
**reasoningOverride** | **String** |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


