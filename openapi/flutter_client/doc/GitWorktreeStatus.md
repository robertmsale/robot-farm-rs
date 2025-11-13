# my_api_client.model.GitWorktreeStatus

## Load the model package
```dart
import 'package:my_api_client/api.dart';
```

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Logical identifier (staging or ws{n}). | 
**path** | **String** | Absolute filesystem path to the worktree. | 
**branch** | **String** | Currently checked out branch. | 
**upstream** | **String** | Configured upstream tracking branch, if any. | [optional] 
**ahead** | **int** | Number of commits ahead of upstream. | 
**behind** | **int** | Number of commits behind upstream. | 
**isDirty** | **bool** | True when there are local modifications. | 
**files** | [**List<GitStatusFileChange>**](GitStatusFileChange.md) | Per-file change metadata. | [default to const []]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


