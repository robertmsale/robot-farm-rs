# my_api_client.model.Config

## Load the model package
```dart
import 'package:my_api_client/api.dart';
```

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**appendAgentsFile** | [**AppendFilesConfig**](AppendFilesConfig.md) |  | 
**models** | [**AgentModelOverrides**](AgentModelOverrides.md) |  | 
**reasoning** | [**AgentReasoningOverrides**](AgentReasoningOverrides.md) |  | 
**commands** | [**List<CommandConfig>**](CommandConfig.md) |  | [default to const []]
**postTurnChecks** | **List<String>** | Commands executed after each turn. | [default to const []]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


