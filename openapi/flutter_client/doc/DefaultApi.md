# my_api_client.api.DefaultApi

## Load the API package
```dart
import 'package:my_api_client/api.dart';
```

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createConfig**](DefaultApi.md#createconfig) | **POST** /config | Create workspace config
[**createTask**](DefaultApi.md#createtask) | **POST** /tasks | Create task
[**createTaskDependency**](DefaultApi.md#createtaskdependency) | **POST** /task-deps | Create task dependency
[**createTaskGroup**](DefaultApi.md#createtaskgroup) | **POST** /task-groups | Create task group
[**createWorker**](DefaultApi.md#createworker) | **POST** /workers | Create worker
[**deleteAllMessages**](DefaultApi.md#deleteallmessages) | **DELETE** /message_queue | Clear all messages
[**deleteConfig**](DefaultApi.md#deleteconfig) | **DELETE** /config | Delete workspace config
[**deleteFeed**](DefaultApi.md#deletefeed) | **DELETE** /feed | Clear feed
[**deleteMessageById**](DefaultApi.md#deletemessagebyid) | **DELETE** /message_queue/{messageId} | Delete message by id
[**deleteMessagesForRecipient**](DefaultApi.md#deletemessagesforrecipient) | **DELETE** /message_queue/to/{sender} | Delete messages to a recipient
[**deleteOrchestratorSession**](DefaultApi.md#deleteorchestratorsession) | **DELETE** /orchestrator/session | Clear orchestrator session state
[**deleteTask**](DefaultApi.md#deletetask) | **DELETE** /tasks/{taskId} | Delete task
[**deleteTaskDependency**](DefaultApi.md#deletetaskdependency) | **DELETE** /task-deps/{taskId}/{dependsOnTaskId} | Delete task dependency
[**deleteTaskGroup**](DefaultApi.md#deletetaskgroup) | **DELETE** /task-groups/{taskGroupId} | Delete task group
[**deleteWorker**](DefaultApi.md#deleteworker) | **DELETE** /workers/{workerId} | Delete worker
[**deleteWorkerSession**](DefaultApi.md#deleteworkersession) | **DELETE** /workers/{workerId}/session | Clear worker session state
[**getActiveStrategy**](DefaultApi.md#getactivestrategy) | **GET** /strategy | Get active strategy
[**getConfig**](DefaultApi.md#getconfig) | **GET** /config | Get workspace config
[**getHealthz**](DefaultApi.md#gethealthz) | **GET** /healthz | Health check
[**getTask**](DefaultApi.md#gettask) | **GET** /tasks/{taskId} | Get task
[**getTaskCommitDiff**](DefaultApi.md#gettaskcommitdiff) | **GET** /tasks/{taskId}/commit/diff | Get diff for a file within the task commit
[**getTaskCommitInfo**](DefaultApi.md#gettaskcommitinfo) | **GET** /tasks/{taskId}/commit | Get task commit info
[**getTaskGroup**](DefaultApi.md#gettaskgroup) | **GET** /task-groups/{taskGroupId} | Get task group
[**insertMessageRelative**](DefaultApi.md#insertmessagerelative) | **PATCH** /message_queue/{messageId}/insert | Insert a message relative to another message
[**listFeed**](DefaultApi.md#listfeed) | **GET** /feed | List feed events
[**listMessages**](DefaultApi.md#listmessages) | **GET** /message_queue | List messages in the queue
[**listTaskDependencies**](DefaultApi.md#listtaskdependencies) | **GET** /task-deps | List dependencies for a task
[**listTaskGroups**](DefaultApi.md#listtaskgroups) | **GET** /task-groups | List task groups
[**listTasks**](DefaultApi.md#listtasks) | **GET** /tasks | List tasks
[**listWorkers**](DefaultApi.md#listworkers) | **GET** /workers | List workers
[**updateActiveStrategy**](DefaultApi.md#updateactivestrategy) | **PUT** /strategy | Update active strategy
[**updateConfig**](DefaultApi.md#updateconfig) | **PUT** /config | Update workspace config
[**updateTask**](DefaultApi.md#updatetask) | **PUT** /tasks/{taskId} | Update task
[**updateTaskGroup**](DefaultApi.md#updatetaskgroup) | **PUT** /task-groups/{taskGroupId} | Update task group


# **createConfig**
> Config createConfig(config)

Create workspace config

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final config = Config(); // Config | 

try {
    final result = api_instance.createConfig(config);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createConfig: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **config** | [**Config**](Config.md)|  | 

### Return type

[**Config**](Config.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createTask**
> Task createTask(taskCreateInput)

Create task

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final taskCreateInput = TaskCreateInput(); // TaskCreateInput | 

try {
    final result = api_instance.createTask(taskCreateInput);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createTask: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **taskCreateInput** | [**TaskCreateInput**](TaskCreateInput.md)|  | 

### Return type

[**Task**](Task.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createTaskDependency**
> TaskDependency createTaskDependency(taskDependencyCreateInput)

Create task dependency

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final taskDependencyCreateInput = TaskDependencyCreateInput(); // TaskDependencyCreateInput | 

try {
    final result = api_instance.createTaskDependency(taskDependencyCreateInput);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createTaskDependency: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **taskDependencyCreateInput** | [**TaskDependencyCreateInput**](TaskDependencyCreateInput.md)|  | 

### Return type

[**TaskDependency**](TaskDependency.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createTaskGroup**
> TaskGroup createTaskGroup(taskGroupCreateInput)

Create task group

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final taskGroupCreateInput = TaskGroupCreateInput(); // TaskGroupCreateInput | 

try {
    final result = api_instance.createTaskGroup(taskGroupCreateInput);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createTaskGroup: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **taskGroupCreateInput** | [**TaskGroupCreateInput**](TaskGroupCreateInput.md)|  | 

### Return type

[**TaskGroup**](TaskGroup.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createWorker**
> Worker createWorker()

Create worker

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();

try {
    final result = api_instance.createWorker();
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createWorker: $e\n');
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

[**Worker**](Worker.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteAllMessages**
> deleteAllMessages()

Clear all messages

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();

try {
    api_instance.deleteAllMessages();
} catch (e) {
    print('Exception when calling DefaultApi->deleteAllMessages: $e\n');
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteConfig**
> deleteConfig()

Delete workspace config

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();

try {
    api_instance.deleteConfig();
} catch (e) {
    print('Exception when calling DefaultApi->deleteConfig: $e\n');
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteFeed**
> deleteFeed()

Clear feed

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();

try {
    api_instance.deleteFeed();
} catch (e) {
    print('Exception when calling DefaultApi->deleteFeed: $e\n');
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteMessageById**
> deleteMessageById(messageId)

Delete message by id

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final messageId = 789; // int | Identifier of the message to delete.

try {
    api_instance.deleteMessageById(messageId);
} catch (e) {
    print('Exception when calling DefaultApi->deleteMessageById: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **messageId** | **int**| Identifier of the message to delete. | 

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteMessagesForRecipient**
> deleteMessagesForRecipient(sender)

Delete messages to a recipient

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final sender = sender_example; // String | Recipient display value (e.g., Orchestrator, Quality Assurance, ws7).

try {
    api_instance.deleteMessagesForRecipient(sender);
} catch (e) {
    print('Exception when calling DefaultApi->deleteMessagesForRecipient: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sender** | **String**| Recipient display value (e.g., Orchestrator, Quality Assurance, ws7). | 

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteOrchestratorSession**
> deleteOrchestratorSession()

Clear orchestrator session state

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();

try {
    api_instance.deleteOrchestratorSession();
} catch (e) {
    print('Exception when calling DefaultApi->deleteOrchestratorSession: $e\n');
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteTask**
> deleteTask(taskId)

Delete task

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final taskId = 789; // int | Unique identifier of the task.

try {
    api_instance.deleteTask(taskId);
} catch (e) {
    print('Exception when calling DefaultApi->deleteTask: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **taskId** | **int**| Unique identifier of the task. | 

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteTaskDependency**
> deleteTaskDependency(taskId, dependsOnTaskId)

Delete task dependency

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final taskId = 789; // int | Task that holds the dependency.
final dependsOnTaskId = 789; // int | Task that is depended on.

try {
    api_instance.deleteTaskDependency(taskId, dependsOnTaskId);
} catch (e) {
    print('Exception when calling DefaultApi->deleteTaskDependency: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **taskId** | **int**| Task that holds the dependency. | 
 **dependsOnTaskId** | **int**| Task that is depended on. | 

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteTaskGroup**
> deleteTaskGroup(taskGroupId)

Delete task group

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final taskGroupId = 789; // int | Unique identifier of the task group.

try {
    api_instance.deleteTaskGroup(taskGroupId);
} catch (e) {
    print('Exception when calling DefaultApi->deleteTaskGroup: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **taskGroupId** | **int**| Unique identifier of the task group. | 

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteWorker**
> deleteWorker(workerId)

Delete worker

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final workerId = 789; // int | Identifier of the worker.

try {
    api_instance.deleteWorker(workerId);
} catch (e) {
    print('Exception when calling DefaultApi->deleteWorker: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **workerId** | **int**| Identifier of the worker. | 

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteWorkerSession**
> deleteWorkerSession(workerId)

Clear worker session state

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final workerId = 789; // int | Identifier of the worker whose session should be cleared.

try {
    api_instance.deleteWorkerSession(workerId);
} catch (e) {
    print('Exception when calling DefaultApi->deleteWorkerSession: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **workerId** | **int**| Identifier of the worker whose session should be cleared. | 

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getActiveStrategy**
> ActiveStrategy getActiveStrategy()

Get active strategy

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();

try {
    final result = api_instance.getActiveStrategy();
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getActiveStrategy: $e\n');
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

[**ActiveStrategy**](ActiveStrategy.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getConfig**
> Config getConfig()

Get workspace config

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();

try {
    final result = api_instance.getConfig();
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getConfig: $e\n');
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

[**Config**](Config.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

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

# **getTask**
> Task getTask(taskId)

Get task

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final taskId = 789; // int | Unique identifier of the task.

try {
    final result = api_instance.getTask(taskId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTask: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **taskId** | **int**| Unique identifier of the task. | 

### Return type

[**Task**](Task.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getTaskCommitDiff**
> String getTaskCommitDiff(file, taskId)

Get diff for a file within the task commit

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final file = file_example; // String | Relative path to the file for the diff.
final taskId = 789; // int | Unique identifier of the task.

try {
    final result = api_instance.getTaskCommitDiff(file, taskId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTaskCommitDiff: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **file** | **String**| Relative path to the file for the diff. | 
 **taskId** | **int**| Unique identifier of the task. | 

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getTaskCommitInfo**
> CommitInfo getTaskCommitInfo(taskId)

Get task commit info

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final taskId = 789; // int | Unique identifier of the task.

try {
    final result = api_instance.getTaskCommitInfo(taskId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTaskCommitInfo: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **taskId** | **int**| Unique identifier of the task. | 

### Return type

[**CommitInfo**](CommitInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getTaskGroup**
> TaskGroup getTaskGroup(taskGroupId)

Get task group

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final taskGroupId = 789; // int | Unique identifier of the task group.

try {
    final result = api_instance.getTaskGroup(taskGroupId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTaskGroup: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **taskGroupId** | **int**| Unique identifier of the task group. | 

### Return type

[**TaskGroup**](TaskGroup.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **insertMessageRelative**
> List<Message> insertMessageRelative(messageId, insertMessage)

Insert a message relative to another message

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final messageId = 789; // int | Message identifier used as the relative insertion anchor.
final insertMessage = InsertMessage(); // InsertMessage | 

try {
    final result = api_instance.insertMessageRelative(messageId, insertMessage);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->insertMessageRelative: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **messageId** | **int**| Message identifier used as the relative insertion anchor. | 
 **insertMessage** | [**InsertMessage**](InsertMessage.md)|  | 

### Return type

[**List<Message>**](Message.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listFeed**
> List<Feed> listFeed(source_, target, status, orderBy)

List feed events

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final source_ = source__example; // String | Filter by source (Orchestrator, Quality Assurance, System, ws#).
final target = target_example; // String | Filter by target (Orchestrator, Quality Assurance, System, ws#).
final status = ; // FeedLevel | Filter by level (info, warning, error).
final orderBy = ; // FeedOrderField | Sort results by the specified Feed field.

try {
    final result = api_instance.listFeed(source_, target, status, orderBy);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->listFeed: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **source_** | **String**| Filter by source (Orchestrator, Quality Assurance, System, ws#). | [optional] 
 **target** | **String**| Filter by target (Orchestrator, Quality Assurance, System, ws#). | [optional] 
 **status** | [**FeedLevel**](.md)| Filter by level (info, warning, error). | [optional] 
 **orderBy** | [**FeedOrderField**](.md)| Sort results by the specified Feed field. | [optional] 

### Return type

[**List<Feed>**](Feed.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listMessages**
> List<Message> listMessages(from, to)

List messages in the queue

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final from = from_example; // String | Filter messages sent from the specified sender display value.
final to = to_example; // String | Filter messages sent to the specified recipient display value.

try {
    final result = api_instance.listMessages(from, to);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->listMessages: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **from** | **String**| Filter messages sent from the specified sender display value. | [optional] 
 **to** | **String**| Filter messages sent to the specified recipient display value. | [optional] 

### Return type

[**List<Message>**](Message.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listTaskDependencies**
> List<int> listTaskDependencies(taskId)

List dependencies for a task

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final taskId = 789; // int | Task identifier to fetch dependencies for.

try {
    final result = api_instance.listTaskDependencies(taskId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->listTaskDependencies: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **taskId** | **int**| Task identifier to fetch dependencies for. | 

### Return type

**List<int>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listTaskGroups**
> List<TaskGroup> listTaskGroups()

List task groups

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();

try {
    final result = api_instance.listTaskGroups();
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->listTaskGroups: $e\n');
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

[**List<TaskGroup>**](TaskGroup.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listTasks**
> List<Task> listTasks(groupSlug, slug, title, commitHash, status, owner)

List tasks

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final groupSlug = groupSlug_example; // String | Filter by owning task group's slug.
final slug = slug_example; // String | Filter by task slug.
final title = title_example; // String | Filter by task title.
final commitHash = commitHash_example; // String | Filter by commit hash.
final status = ; // TaskStatus | Filter by task status.
final owner = owner_example; // String | Filter by owner display text (e.g., Orchestrator, Quality Assurance, ws42).

try {
    final result = api_instance.listTasks(groupSlug, slug, title, commitHash, status, owner);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->listTasks: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **groupSlug** | **String**| Filter by owning task group's slug. | [optional] 
 **slug** | **String**| Filter by task slug. | [optional] 
 **title** | **String**| Filter by task title. | [optional] 
 **commitHash** | **String**| Filter by commit hash. | [optional] 
 **status** | [**TaskStatus**](.md)| Filter by task status. | [optional] 
 **owner** | **String**| Filter by owner display text (e.g., Orchestrator, Quality Assurance, ws42). | [optional] 

### Return type

[**List<Task>**](Task.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listWorkers**
> List<Worker> listWorkers()

List workers

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();

try {
    final result = api_instance.listWorkers();
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->listWorkers: $e\n');
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

[**List<Worker>**](Worker.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateActiveStrategy**
> ActiveStrategy updateActiveStrategy(activeStrategy)

Update active strategy

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final activeStrategy = ActiveStrategy(); // ActiveStrategy | 

try {
    final result = api_instance.updateActiveStrategy(activeStrategy);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateActiveStrategy: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **activeStrategy** | [**ActiveStrategy**](ActiveStrategy.md)|  | 

### Return type

[**ActiveStrategy**](ActiveStrategy.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateConfig**
> Config updateConfig(config)

Update workspace config

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final config = Config(); // Config | 

try {
    final result = api_instance.updateConfig(config);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateConfig: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **config** | [**Config**](Config.md)|  | 

### Return type

[**Config**](Config.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateTask**
> Task updateTask(taskId, taskUpdateInput)

Update task

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final taskId = 789; // int | Unique identifier of the task.
final taskUpdateInput = TaskUpdateInput(); // TaskUpdateInput | 

try {
    final result = api_instance.updateTask(taskId, taskUpdateInput);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateTask: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **taskId** | **int**| Unique identifier of the task. | 
 **taskUpdateInput** | [**TaskUpdateInput**](TaskUpdateInput.md)|  | 

### Return type

[**Task**](Task.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateTaskGroup**
> TaskGroup updateTaskGroup(taskGroupId, taskGroupUpdateInput)

Update task group

### Example
```dart
import 'package:my_api_client/api.dart';

final api_instance = DefaultApi();
final taskGroupId = 789; // int | Unique identifier of the task group.
final taskGroupUpdateInput = TaskGroupUpdateInput(); // TaskGroupUpdateInput | 

try {
    final result = api_instance.updateTaskGroup(taskGroupId, taskGroupUpdateInput);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateTaskGroup: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **taskGroupId** | **int**| Unique identifier of the task group. | 
 **taskGroupUpdateInput** | [**TaskGroupUpdateInput**](TaskGroupUpdateInput.md)|  | 

### Return type

[**TaskGroup**](TaskGroup.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

