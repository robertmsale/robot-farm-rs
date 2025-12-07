# my_api_client.model.Config

## Load the model package
```dart
import 'package:my_api_client/api.dart';
```

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workspacePath** | **String** | Absolute path of the workspace the server is running in. | [optional] 
**appendAgentsFile** | [**AppendFilesConfig**](AppendFilesConfig.md) |  | 
**models** | [**AgentModelOverrides**](AgentModelOverrides.md) |  | 
**reasoning** | [**AgentReasoningOverrides**](AgentReasoningOverrides.md) |  | 
**commands** | [**List<CommandConfig>**](CommandConfig.md) |  | [default to const []]
**postTurnChecks** | **List<String>** | Commands executed after each turn. | [default to const []]
**dockerOverrides** | [**DockerOverrides**](DockerOverrides.md) |  | 
**dirtyStagingAction** | **String** | Action to take when staging worktree is dirty during task completion. | [optional] [default to 'commit']
**onStagingChange** | **List<String>** | Command IDs to run after staging updates. | [optional] [default to const []]
**persistentThreads** | **bool** | Keep worker threads after task completion; still cleared when the feed is cleared. | [optional] [default to false]
**ghostCommits** | **bool** | Automatically commit worker changes after every turn. | [optional] [default to false]
**driftManager** | **bool** | (experimental) Attach worker reasoning traces to messages sent to the orchestrator. | [optional] [default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


