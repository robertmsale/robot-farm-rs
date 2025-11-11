# my_api_client.model.Feed

## Load the model package
```dart
import 'package:my_api_client/api.dart';
```

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **int** |  | 
**source_** | **String** | Emitter of the feed entry (Orchestrator, Quality Assurance, System, ws#). | 
**target** | **String** | Recipient focus (Orchestrator, Quality Assurance, System, ws#). | 
**ts** | **int** | Unix timestamp in seconds. | 
**level** | [**FeedLevel**](FeedLevel.md) |  | 
**text** | **String** | Human-friendly summary. | 
**raw** | **String** | Raw log payload. | 
**category** | **String** | Category tag for the entry. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


