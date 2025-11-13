# my_api_client.model.GitStatusFileChange

## Load the model package
```dart
import 'package:my_api_client/api.dart';
```

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**path** | **String** | Current file path relative to the worktree root. | 
**oldPath** | **String** | Previous path when the file was renamed or moved. | [optional] 
**statusCode** | **String** | Two-character porcelain status code (e.g., M?, R?). | 
**additions** | **int** | Number of added lines reported by git. | 
**deletions** | **int** | Number of removed lines reported by git. | 
**hunks** | [**List<GitStatusHunk>**](GitStatusHunk.md) | Diff hunks for this file when requested. | [optional] [default to const []]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


