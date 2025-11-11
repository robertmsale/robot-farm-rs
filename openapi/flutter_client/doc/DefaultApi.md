# my_api_client.api.DefaultApi

## Load the API package
```dart
import 'package:my_api_client/api.dart';
```

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createTask**](DefaultApi.md#createtask) | **POST** /tasks | Create task
[**createTaskDependency**](DefaultApi.md#createtaskdependency) | **POST** /task-deps | Create task dependency
[**createTaskGroup**](DefaultApi.md#createtaskgroup) | **POST** /task-groups | Create task group
[**createWorker**](DefaultApi.md#createworker) | **POST** /workers | Create worker
[**deleteTask**](DefaultApi.md#deletetask) | **DELETE** /tasks/{taskId} | Delete task
[**deleteTaskDependency**](DefaultApi.md#deletetaskdependency) | **DELETE** /task-deps/{taskId}/{dependsOnTaskId} | Delete task dependency
[**deleteTaskGroup**](DefaultApi.md#deletetaskgroup) | **DELETE** /task-groups/{taskGroupId} | Delete task group
[**deleteWorker**](DefaultApi.md#deleteworker) | **DELETE** /workers/{workerId} | Delete worker
[**getHealthz**](DefaultApi.md#gethealthz) | **GET** /healthz | Health check
[**getTask**](DefaultApi.md#gettask) | **GET** /tasks/{taskId} | Get task
[**getTaskGroup**](DefaultApi.md#gettaskgroup) | **GET** /task-groups/{taskGroupId} | Get task group
[**listTaskDependencies**](DefaultApi.md#listtaskdependencies) | **GET** /task-deps | List dependencies for a task
[**listTaskGroups**](DefaultApi.md#listtaskgroups) | **GET** /task-groups | List task groups
[**listTasks**](DefaultApi.md#listtasks) | **GET** /tasks | List tasks
[**listWorkers**](DefaultApi.md#listworkers) | **GET** /workers | List workers
[**updateTask**](DefaultApi.md#updatetask) | **PUT** /tasks/{taskId} | Update task
[**updateTaskGroup**](DefaultApi.md#updatetaskgroup) | **PUT** /task-groups/{taskGroupId} | Update task group


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

