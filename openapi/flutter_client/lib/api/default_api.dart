//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;


class DefaultApi {
  DefaultApi([ApiClient? apiClient]) : apiClient = apiClient ?? defaultApiClient;

  final ApiClient apiClient;

  /// Create workspace config
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [Config] config (required):
  Future<Response> createConfigWithHttpInfo(Config config,) async {
    // ignore: prefer_const_declarations
    final path = r'/config';

    // ignore: prefer_final_locals
    Object? postBody = config;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'POST',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Create workspace config
  ///
  /// Parameters:
  ///
  /// * [Config] config (required):
  Future<Config?> createConfig(Config config,) async {
    final response = await createConfigWithHttpInfo(config,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'Config',) as Config;
    
    }
    return null;
  }

  /// Create task
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [TaskCreateInput] taskCreateInput (required):
  Future<Response> createTaskWithHttpInfo(TaskCreateInput taskCreateInput,) async {
    // ignore: prefer_const_declarations
    final path = r'/tasks';

    // ignore: prefer_final_locals
    Object? postBody = taskCreateInput;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'POST',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Create task
  ///
  /// Parameters:
  ///
  /// * [TaskCreateInput] taskCreateInput (required):
  Future<Task?> createTask(TaskCreateInput taskCreateInput,) async {
    final response = await createTaskWithHttpInfo(taskCreateInput,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'Task',) as Task;
    
    }
    return null;
  }

  /// Create task dependency
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [TaskDependencyCreateInput] taskDependencyCreateInput (required):
  Future<Response> createTaskDependencyWithHttpInfo(TaskDependencyCreateInput taskDependencyCreateInput,) async {
    // ignore: prefer_const_declarations
    final path = r'/task-deps';

    // ignore: prefer_final_locals
    Object? postBody = taskDependencyCreateInput;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'POST',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Create task dependency
  ///
  /// Parameters:
  ///
  /// * [TaskDependencyCreateInput] taskDependencyCreateInput (required):
  Future<TaskDependency?> createTaskDependency(TaskDependencyCreateInput taskDependencyCreateInput,) async {
    final response = await createTaskDependencyWithHttpInfo(taskDependencyCreateInput,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'TaskDependency',) as TaskDependency;
    
    }
    return null;
  }

  /// Create task group
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [TaskGroupCreateInput] taskGroupCreateInput (required):
  Future<Response> createTaskGroupWithHttpInfo(TaskGroupCreateInput taskGroupCreateInput,) async {
    // ignore: prefer_const_declarations
    final path = r'/task-groups';

    // ignore: prefer_final_locals
    Object? postBody = taskGroupCreateInput;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'POST',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Create task group
  ///
  /// Parameters:
  ///
  /// * [TaskGroupCreateInput] taskGroupCreateInput (required):
  Future<TaskGroup?> createTaskGroup(TaskGroupCreateInput taskGroupCreateInput,) async {
    final response = await createTaskGroupWithHttpInfo(taskGroupCreateInput,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'TaskGroup',) as TaskGroup;
    
    }
    return null;
  }

  /// Create worker
  ///
  /// Note: This method returns the HTTP [Response].
  Future<Response> createWorkerWithHttpInfo() async {
    // ignore: prefer_const_declarations
    final path = r'/workers';

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'POST',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Create worker
  Future<Worker?> createWorker() async {
    final response = await createWorkerWithHttpInfo();
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'Worker',) as Worker;
    
    }
    return null;
  }

  /// Clear all messages
  ///
  /// Note: This method returns the HTTP [Response].
  Future<Response> deleteAllMessagesWithHttpInfo() async {
    // ignore: prefer_const_declarations
    final path = r'/message_queue';

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'DELETE',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Clear all messages
  Future<void> deleteAllMessages() async {
    final response = await deleteAllMessagesWithHttpInfo();
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
  }

  /// Delete workspace config
  ///
  /// Note: This method returns the HTTP [Response].
  Future<Response> deleteConfigWithHttpInfo() async {
    // ignore: prefer_const_declarations
    final path = r'/config';

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'DELETE',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Delete workspace config
  Future<void> deleteConfig() async {
    final response = await deleteConfigWithHttpInfo();
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
  }

  /// Clear feed
  ///
  /// Note: This method returns the HTTP [Response].
  Future<Response> deleteFeedWithHttpInfo() async {
    // ignore: prefer_const_declarations
    final path = r'/feed';

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'DELETE',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Clear feed
  Future<void> deleteFeed() async {
    final response = await deleteFeedWithHttpInfo();
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
  }

  /// Delete message by id
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [int] messageId (required):
  ///   Identifier of the message to delete.
  Future<Response> deleteMessageByIdWithHttpInfo(int messageId,) async {
    // ignore: prefer_const_declarations
    final path = r'/message_queue/{messageId}'
      .replaceAll('{messageId}', messageId.toString());

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'DELETE',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Delete message by id
  ///
  /// Parameters:
  ///
  /// * [int] messageId (required):
  ///   Identifier of the message to delete.
  Future<void> deleteMessageById(int messageId,) async {
    final response = await deleteMessageByIdWithHttpInfo(messageId,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
  }

  /// Delete messages to a recipient
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [String] sender (required):
  ///   Recipient display value (e.g., Orchestrator, Quality Assurance, ws7).
  Future<Response> deleteMessagesForRecipientWithHttpInfo(String sender,) async {
    // ignore: prefer_const_declarations
    final path = r'/message_queue/to/{sender}'
      .replaceAll('{sender}', sender);

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'DELETE',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Delete messages to a recipient
  ///
  /// Parameters:
  ///
  /// * [String] sender (required):
  ///   Recipient display value (e.g., Orchestrator, Quality Assurance, ws7).
  Future<void> deleteMessagesForRecipient(String sender,) async {
    final response = await deleteMessagesForRecipientWithHttpInfo(sender,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
  }

  /// Clear orchestrator session state
  ///
  /// Note: This method returns the HTTP [Response].
  Future<Response> deleteOrchestratorSessionWithHttpInfo() async {
    // ignore: prefer_const_declarations
    final path = r'/orchestrator/session';

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'DELETE',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Clear orchestrator session state
  Future<void> deleteOrchestratorSession() async {
    final response = await deleteOrchestratorSessionWithHttpInfo();
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
  }

  /// Delete task
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [int] taskId (required):
  ///   Unique identifier of the task.
  Future<Response> deleteTaskWithHttpInfo(int taskId,) async {
    // ignore: prefer_const_declarations
    final path = r'/tasks/{taskId}'
      .replaceAll('{taskId}', taskId.toString());

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'DELETE',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Delete task
  ///
  /// Parameters:
  ///
  /// * [int] taskId (required):
  ///   Unique identifier of the task.
  Future<void> deleteTask(int taskId,) async {
    final response = await deleteTaskWithHttpInfo(taskId,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
  }

  /// Delete task dependency
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [int] taskId (required):
  ///   Task that holds the dependency.
  ///
  /// * [int] dependsOnTaskId (required):
  ///   Task that is depended on.
  Future<Response> deleteTaskDependencyWithHttpInfo(int taskId, int dependsOnTaskId,) async {
    // ignore: prefer_const_declarations
    final path = r'/task-deps/{taskId}/{dependsOnTaskId}'
      .replaceAll('{taskId}', taskId.toString())
      .replaceAll('{dependsOnTaskId}', dependsOnTaskId.toString());

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'DELETE',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Delete task dependency
  ///
  /// Parameters:
  ///
  /// * [int] taskId (required):
  ///   Task that holds the dependency.
  ///
  /// * [int] dependsOnTaskId (required):
  ///   Task that is depended on.
  Future<void> deleteTaskDependency(int taskId, int dependsOnTaskId,) async {
    final response = await deleteTaskDependencyWithHttpInfo(taskId, dependsOnTaskId,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
  }

  /// Delete task group
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [int] taskGroupId (required):
  ///   Unique identifier of the task group.
  Future<Response> deleteTaskGroupWithHttpInfo(int taskGroupId,) async {
    // ignore: prefer_const_declarations
    final path = r'/task-groups/{taskGroupId}'
      .replaceAll('{taskGroupId}', taskGroupId.toString());

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'DELETE',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Delete task group
  ///
  /// Parameters:
  ///
  /// * [int] taskGroupId (required):
  ///   Unique identifier of the task group.
  Future<void> deleteTaskGroup(int taskGroupId,) async {
    final response = await deleteTaskGroupWithHttpInfo(taskGroupId,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
  }

  /// Delete worker
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [int] workerId (required):
  ///   Identifier of the worker.
  Future<Response> deleteWorkerWithHttpInfo(int workerId,) async {
    // ignore: prefer_const_declarations
    final path = r'/workers/{workerId}'
      .replaceAll('{workerId}', workerId.toString());

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'DELETE',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Delete worker
  ///
  /// Parameters:
  ///
  /// * [int] workerId (required):
  ///   Identifier of the worker.
  Future<void> deleteWorker(int workerId,) async {
    final response = await deleteWorkerWithHttpInfo(workerId,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
  }

  /// Clear worker session state
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [int] workerId (required):
  ///   Identifier of the worker whose session should be cleared.
  Future<Response> deleteWorkerSessionWithHttpInfo(int workerId,) async {
    // ignore: prefer_const_declarations
    final path = r'/workers/{workerId}/session'
      .replaceAll('{workerId}', workerId.toString());

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'DELETE',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Clear worker session state
  ///
  /// Parameters:
  ///
  /// * [int] workerId (required):
  ///   Identifier of the worker whose session should be cleared.
  Future<void> deleteWorkerSession(int workerId,) async {
    final response = await deleteWorkerSessionWithHttpInfo(workerId,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
  }

  /// Enqueue message
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [MessageEnqueueInput] messageEnqueueInput (required):
  Future<Response> enqueueMessageWithHttpInfo(MessageEnqueueInput messageEnqueueInput,) async {
    // ignore: prefer_const_declarations
    final path = r'/message_queue';

    // ignore: prefer_final_locals
    Object? postBody = messageEnqueueInput;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'POST',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Enqueue message
  ///
  /// Parameters:
  ///
  /// * [MessageEnqueueInput] messageEnqueueInput (required):
  Future<Message?> enqueueMessage(MessageEnqueueInput messageEnqueueInput,) async {
    final response = await enqueueMessageWithHttpInfo(messageEnqueueInput,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'Message',) as Message;
    
    }
    return null;
  }

  /// Execute a shell command for the orchestrator
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [ExecCommandInput] execCommandInput (required):
  Future<Response> execOrchestratorCommandWithHttpInfo(ExecCommandInput execCommandInput,) async {
    // ignore: prefer_const_declarations
    final path = r'/orchestrator/exec';

    // ignore: prefer_final_locals
    Object? postBody = execCommandInput;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'POST',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Execute a shell command for the orchestrator
  ///
  /// Parameters:
  ///
  /// * [ExecCommandInput] execCommandInput (required):
  Future<ExecResult?> execOrchestratorCommand(ExecCommandInput execCommandInput,) async {
    final response = await execOrchestratorCommandWithHttpInfo(execCommandInput,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'ExecResult',) as ExecResult;
    
    }
    return null;
  }

  /// Execute a shell command within a worker workspace
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [int] workerId (required):
  ///   Identifier of the worker whose workspace should be used for execution.
  ///
  /// * [ExecCommandInput] execCommandInput (required):
  Future<Response> execWorkerCommandWithHttpInfo(int workerId, ExecCommandInput execCommandInput,) async {
    // ignore: prefer_const_declarations
    final path = r'/workers/{workerId}/exec'
      .replaceAll('{workerId}', workerId.toString());

    // ignore: prefer_final_locals
    Object? postBody = execCommandInput;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'POST',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Execute a shell command within a worker workspace
  ///
  /// Parameters:
  ///
  /// * [int] workerId (required):
  ///   Identifier of the worker whose workspace should be used for execution.
  ///
  /// * [ExecCommandInput] execCommandInput (required):
  Future<ExecResult?> execWorkerCommand(int workerId, ExecCommandInput execCommandInput,) async {
    final response = await execWorkerCommandWithHttpInfo(workerId, execCommandInput,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'ExecResult',) as ExecResult;
    
    }
    return null;
  }

  /// Get active strategy
  ///
  /// Note: This method returns the HTTP [Response].
  Future<Response> getActiveStrategyWithHttpInfo() async {
    // ignore: prefer_const_declarations
    final path = r'/strategy';

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Get active strategy
  Future<ActiveStrategy?> getActiveStrategy() async {
    final response = await getActiveStrategyWithHttpInfo();
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'ActiveStrategy',) as ActiveStrategy;
    
    }
    return null;
  }

  /// Get workspace config
  ///
  /// Note: This method returns the HTTP [Response].
  Future<Response> getConfigWithHttpInfo() async {
    // ignore: prefer_const_declarations
    final path = r'/config';

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Get workspace config
  Future<Config?> getConfig() async {
    final response = await getConfigWithHttpInfo();
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'Config',) as Config;
    
    }
    return null;
  }

  /// Get feed entry
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [int] feedId (required):
  ///   Feed entry identifier.
  Future<Response> getFeedEntryWithHttpInfo(int feedId,) async {
    // ignore: prefer_const_declarations
    final path = r'/feed/{feedId}'
      .replaceAll('{feedId}', feedId.toString());

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Get feed entry
  ///
  /// Parameters:
  ///
  /// * [int] feedId (required):
  ///   Feed entry identifier.
  Future<Feed?> getFeedEntry(int feedId,) async {
    final response = await getFeedEntryWithHttpInfo(feedId,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'Feed',) as Feed;
    
    }
    return null;
  }

  /// Get full git status for a specific worktree, including diff hunks.
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [String] worktreeId (required):
  ///   Identifier of the worktree (`staging` or `ws{n}`).
  Future<Response> getGitStatusForWorktreeWithHttpInfo(String worktreeId,) async {
    // ignore: prefer_const_declarations
    final path = r'/git/status/{worktreeId}'
      .replaceAll('{worktreeId}', worktreeId);

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Get full git status for a specific worktree, including diff hunks.
  ///
  /// Parameters:
  ///
  /// * [String] worktreeId (required):
  ///   Identifier of the worktree (`staging` or `ws{n}`).
  Future<GitWorktreeStatus?> getGitStatusForWorktree(String worktreeId,) async {
    final response = await getGitStatusForWorktreeWithHttpInfo(worktreeId,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'GitWorktreeStatus',) as GitWorktreeStatus;
    
    }
    return null;
  }

  /// List git status information for all worktrees.
  ///
  /// Note: This method returns the HTTP [Response].
  Future<Response> getGitStatusSummaryWithHttpInfo() async {
    // ignore: prefer_const_declarations
    final path = r'/git/status';

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// List git status information for all worktrees.
  Future<GitStatusSummary?> getGitStatusSummary() async {
    final response = await getGitStatusSummaryWithHttpInfo();
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'GitStatusSummary',) as GitStatusSummary;
    
    }
    return null;
  }

  /// Health check
  ///
  /// Note: This method returns the HTTP [Response].
  Future<Response> getHealthzWithHttpInfo() async {
    // ignore: prefer_const_declarations
    final path = r'/healthz';

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Health check
  Future<GetHealthz200Response?> getHealthz() async {
    final response = await getHealthzWithHttpInfo();
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'GetHealthz200Response',) as GetHealthz200Response;
    
    }
    return null;
  }

  /// Get queue state
  ///
  /// Note: This method returns the HTTP [Response].
  Future<Response> getQueueStateWithHttpInfo() async {
    // ignore: prefer_const_declarations
    final path = r'/queue';

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Get queue state
  Future<QueueState?> getQueueState() async {
    final response = await getQueueStateWithHttpInfo();
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'QueueState',) as QueueState;
    
    }
    return null;
  }

  /// Get task
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [int] taskId (required):
  ///   Unique identifier of the task.
  Future<Response> getTaskWithHttpInfo(int taskId,) async {
    // ignore: prefer_const_declarations
    final path = r'/tasks/{taskId}'
      .replaceAll('{taskId}', taskId.toString());

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Get task
  ///
  /// Parameters:
  ///
  /// * [int] taskId (required):
  ///   Unique identifier of the task.
  Future<Task?> getTask(int taskId,) async {
    final response = await getTaskWithHttpInfo(taskId,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'Task',) as Task;
    
    }
    return null;
  }

  /// Get diff for a file within the task commit
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [String] file (required):
  ///   Relative path to the file for the diff.
  ///
  /// * [int] taskId (required):
  ///   Unique identifier of the task.
  Future<Response> getTaskCommitDiffWithHttpInfo(String file, int taskId,) async {
    // ignore: prefer_const_declarations
    final path = r'/tasks/{taskId}/commit/diff'
      .replaceAll('{taskId}', taskId.toString());

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

      queryParams.addAll(_queryParams('', 'file', file));

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Get diff for a file within the task commit
  ///
  /// Parameters:
  ///
  /// * [String] file (required):
  ///   Relative path to the file for the diff.
  ///
  /// * [int] taskId (required):
  ///   Unique identifier of the task.
  Future<String?> getTaskCommitDiff(String file, int taskId,) async {
    final response = await getTaskCommitDiffWithHttpInfo(file, taskId,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'String',) as String;
    
    }
    return null;
  }

  /// Get task commit info
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [int] taskId (required):
  ///   Unique identifier of the task.
  Future<Response> getTaskCommitInfoWithHttpInfo(int taskId,) async {
    // ignore: prefer_const_declarations
    final path = r'/tasks/{taskId}/commit'
      .replaceAll('{taskId}', taskId.toString());

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Get task commit info
  ///
  /// Parameters:
  ///
  /// * [int] taskId (required):
  ///   Unique identifier of the task.
  Future<CommitInfo?> getTaskCommitInfo(int taskId,) async {
    final response = await getTaskCommitInfoWithHttpInfo(taskId,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'CommitInfo',) as CommitInfo;
    
    }
    return null;
  }

  /// Get task group
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [int] taskGroupId (required):
  ///   Unique identifier of the task group.
  Future<Response> getTaskGroupWithHttpInfo(int taskGroupId,) async {
    // ignore: prefer_const_declarations
    final path = r'/task-groups/{taskGroupId}'
      .replaceAll('{taskGroupId}', taskGroupId.toString());

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Get task group
  ///
  /// Parameters:
  ///
  /// * [int] taskGroupId (required):
  ///   Unique identifier of the task group.
  Future<TaskGroup?> getTaskGroup(int taskGroupId,) async {
    final response = await getTaskGroupWithHttpInfo(taskGroupId,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'TaskGroup',) as TaskGroup;
    
    }
    return null;
  }

  /// Commit all changes in a worktree
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [String] worktreeId (required):
  ///
  /// * [GitCommitWorktreeIdPostRequest] gitCommitWorktreeIdPostRequest (required):
  Future<Response> gitCommitWorktreeIdPostWithHttpInfo(String worktreeId, GitCommitWorktreeIdPostRequest gitCommitWorktreeIdPostRequest,) async {
    // ignore: prefer_const_declarations
    final path = r'/git/commit/{worktreeId}'
      .replaceAll('{worktreeId}', worktreeId);

    // ignore: prefer_final_locals
    Object? postBody = gitCommitWorktreeIdPostRequest;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'POST',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Commit all changes in a worktree
  ///
  /// Parameters:
  ///
  /// * [String] worktreeId (required):
  ///
  /// * [GitCommitWorktreeIdPostRequest] gitCommitWorktreeIdPostRequest (required):
  Future<void> gitCommitWorktreeIdPost(String worktreeId, GitCommitWorktreeIdPostRequest gitCommitWorktreeIdPostRequest,) async {
    final response = await gitCommitWorktreeIdPostWithHttpInfo(worktreeId, gitCommitWorktreeIdPostRequest,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
  }

  /// Insert a message relative to another message
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [int] messageId (required):
  ///   Message identifier used as the relative insertion anchor.
  ///
  /// * [InsertMessage] insertMessage (required):
  Future<Response> insertMessageRelativeWithHttpInfo(int messageId, InsertMessage insertMessage,) async {
    // ignore: prefer_const_declarations
    final path = r'/message_queue/{messageId}/insert'
      .replaceAll('{messageId}', messageId.toString());

    // ignore: prefer_final_locals
    Object? postBody = insertMessage;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'PATCH',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Insert a message relative to another message
  ///
  /// Parameters:
  ///
  /// * [int] messageId (required):
  ///   Message identifier used as the relative insertion anchor.
  ///
  /// * [InsertMessage] insertMessage (required):
  Future<List<Message>?> insertMessageRelative(int messageId, InsertMessage insertMessage,) async {
    final response = await insertMessageRelativeWithHttpInfo(messageId, insertMessage,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      final responseBody = await _decodeBodyBytes(response);
      return (await apiClient.deserializeAsync(responseBody, 'List<Message>') as List)
        .cast<Message>()
        .toList(growable: false);

    }
    return null;
  }

  /// List feed events
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [String] source_:
  ///   Filter by source (Orchestrator, Quality Assurance, System, ws#).
  ///
  /// * [String] target:
  ///   Filter by target (Orchestrator, Quality Assurance, System, ws#).
  ///
  /// * [FeedLevel] status:
  ///   Filter by level (info, warning, error).
  ///
  /// * [FeedOrderField] orderBy:
  ///   Sort results by the specified Feed field.
  Future<Response> listFeedWithHttpInfo({ String? source_, String? target, FeedLevel? status, FeedOrderField? orderBy, }) async {
    // ignore: prefer_const_declarations
    final path = r'/feed';

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    if (source_ != null) {
      queryParams.addAll(_queryParams('', 'source', source_));
    }
    if (target != null) {
      queryParams.addAll(_queryParams('', 'target', target));
    }
    if (status != null) {
      queryParams.addAll(_queryParams('', 'status', status));
    }
    if (orderBy != null) {
      queryParams.addAll(_queryParams('', 'order_by', orderBy));
    }

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// List feed events
  ///
  /// Parameters:
  ///
  /// * [String] source_:
  ///   Filter by source (Orchestrator, Quality Assurance, System, ws#).
  ///
  /// * [String] target:
  ///   Filter by target (Orchestrator, Quality Assurance, System, ws#).
  ///
  /// * [FeedLevel] status:
  ///   Filter by level (info, warning, error).
  ///
  /// * [FeedOrderField] orderBy:
  ///   Sort results by the specified Feed field.
  Future<List<Feed>?> listFeed({ String? source_, String? target, FeedLevel? status, FeedOrderField? orderBy, }) async {
    final response = await listFeedWithHttpInfo( source_: source_, target: target, status: status, orderBy: orderBy, );
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      final responseBody = await _decodeBodyBytes(response);
      return (await apiClient.deserializeAsync(responseBody, 'List<Feed>') as List)
        .cast<Feed>()
        .toList(growable: false);

    }
    return null;
  }

  /// List messages in the queue
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [String] from:
  ///   Filter messages sent from the specified sender display value.
  ///
  /// * [String] to:
  ///   Filter messages sent to the specified recipient display value.
  Future<Response> listMessagesWithHttpInfo({ String? from, String? to, }) async {
    // ignore: prefer_const_declarations
    final path = r'/message_queue';

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    if (from != null) {
      queryParams.addAll(_queryParams('', 'from', from));
    }
    if (to != null) {
      queryParams.addAll(_queryParams('', 'to', to));
    }

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// List messages in the queue
  ///
  /// Parameters:
  ///
  /// * [String] from:
  ///   Filter messages sent from the specified sender display value.
  ///
  /// * [String] to:
  ///   Filter messages sent to the specified recipient display value.
  Future<List<Message>?> listMessages({ String? from, String? to, }) async {
    final response = await listMessagesWithHttpInfo( from: from, to: to, );
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      final responseBody = await _decodeBodyBytes(response);
      return (await apiClient.deserializeAsync(responseBody, 'List<Message>') as List)
        .cast<Message>()
        .toList(growable: false);

    }
    return null;
  }

  /// List dependencies for a task
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [int] taskId (required):
  ///   Task identifier to fetch dependencies for.
  Future<Response> listTaskDependenciesWithHttpInfo(int taskId,) async {
    // ignore: prefer_const_declarations
    final path = r'/task-deps';

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

      queryParams.addAll(_queryParams('', 'task_id', taskId));

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// List dependencies for a task
  ///
  /// Parameters:
  ///
  /// * [int] taskId (required):
  ///   Task identifier to fetch dependencies for.
  Future<List<int>?> listTaskDependencies(int taskId,) async {
    final response = await listTaskDependenciesWithHttpInfo(taskId,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      final responseBody = await _decodeBodyBytes(response);
      return (await apiClient.deserializeAsync(responseBody, 'List<int>') as List)
        .cast<int>()
        .toList(growable: false);

    }
    return null;
  }

  /// List task groups
  ///
  /// Note: This method returns the HTTP [Response].
  Future<Response> listTaskGroupsWithHttpInfo() async {
    // ignore: prefer_const_declarations
    final path = r'/task-groups';

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// List task groups
  Future<List<TaskGroup>?> listTaskGroups() async {
    final response = await listTaskGroupsWithHttpInfo();
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      final responseBody = await _decodeBodyBytes(response);
      return (await apiClient.deserializeAsync(responseBody, 'List<TaskGroup>') as List)
        .cast<TaskGroup>()
        .toList(growable: false);

    }
    return null;
  }

  /// List tasks
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [String] groupSlug:
  ///   Filter by owning task group's slug.
  ///
  /// * [String] slug:
  ///   Filter by task slug.
  ///
  /// * [String] title:
  ///   Filter by task title.
  ///
  /// * [String] commitHash:
  ///   Filter by commit hash.
  ///
  /// * [TaskStatus] status:
  ///   Filter by task status.
  ///
  /// * [String] owner:
  ///   Filter by owner display text (e.g., Orchestrator, Quality Assurance, ws42).
  Future<Response> listTasksWithHttpInfo({ String? groupSlug, String? slug, String? title, String? commitHash, TaskStatus? status, String? owner, }) async {
    // ignore: prefer_const_declarations
    final path = r'/tasks';

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    if (groupSlug != null) {
      queryParams.addAll(_queryParams('', 'group_slug', groupSlug));
    }
    if (slug != null) {
      queryParams.addAll(_queryParams('', 'slug', slug));
    }
    if (title != null) {
      queryParams.addAll(_queryParams('', 'title', title));
    }
    if (commitHash != null) {
      queryParams.addAll(_queryParams('', 'commit_hash', commitHash));
    }
    if (status != null) {
      queryParams.addAll(_queryParams('', 'status', status));
    }
    if (owner != null) {
      queryParams.addAll(_queryParams('', 'owner', owner));
    }

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// List tasks
  ///
  /// Parameters:
  ///
  /// * [String] groupSlug:
  ///   Filter by owning task group's slug.
  ///
  /// * [String] slug:
  ///   Filter by task slug.
  ///
  /// * [String] title:
  ///   Filter by task title.
  ///
  /// * [String] commitHash:
  ///   Filter by commit hash.
  ///
  /// * [TaskStatus] status:
  ///   Filter by task status.
  ///
  /// * [String] owner:
  ///   Filter by owner display text (e.g., Orchestrator, Quality Assurance, ws42).
  Future<List<Task>?> listTasks({ String? groupSlug, String? slug, String? title, String? commitHash, TaskStatus? status, String? owner, }) async {
    final response = await listTasksWithHttpInfo( groupSlug: groupSlug, slug: slug, title: title, commitHash: commitHash, status: status, owner: owner, );
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      final responseBody = await _decodeBodyBytes(response);
      return (await apiClient.deserializeAsync(responseBody, 'List<Task>') as List)
        .cast<Task>()
        .toList(growable: false);

    }
    return null;
  }

  /// List workers
  ///
  /// Note: This method returns the HTTP [Response].
  Future<Response> listWorkersWithHttpInfo() async {
    // ignore: prefer_const_declarations
    final path = r'/workers';

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// List workers
  Future<List<Worker>?> listWorkers() async {
    final response = await listWorkersWithHttpInfo();
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      final responseBody = await _decodeBodyBytes(response);
      return (await apiClient.deserializeAsync(responseBody, 'List<Worker>') as List)
        .cast<Worker>()
        .toList(growable: false);

    }
    return null;
  }

  /// Update active strategy
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [ActiveStrategy] activeStrategy (required):
  Future<Response> updateActiveStrategyWithHttpInfo(ActiveStrategy activeStrategy,) async {
    // ignore: prefer_const_declarations
    final path = r'/strategy';

    // ignore: prefer_final_locals
    Object? postBody = activeStrategy;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'PUT',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Update active strategy
  ///
  /// Parameters:
  ///
  /// * [ActiveStrategy] activeStrategy (required):
  Future<ActiveStrategy?> updateActiveStrategy(ActiveStrategy activeStrategy,) async {
    final response = await updateActiveStrategyWithHttpInfo(activeStrategy,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'ActiveStrategy',) as ActiveStrategy;
    
    }
    return null;
  }

  /// Update workspace config
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [Config] config (required):
  Future<Response> updateConfigWithHttpInfo(Config config,) async {
    // ignore: prefer_const_declarations
    final path = r'/config';

    // ignore: prefer_final_locals
    Object? postBody = config;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'PUT',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Update workspace config
  ///
  /// Parameters:
  ///
  /// * [Config] config (required):
  Future<Config?> updateConfig(Config config,) async {
    final response = await updateConfigWithHttpInfo(config,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'Config',) as Config;
    
    }
    return null;
  }

  /// Update queue state
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [QueueState] queueState (required):
  Future<Response> updateQueueStateWithHttpInfo(QueueState queueState,) async {
    // ignore: prefer_const_declarations
    final path = r'/queue';

    // ignore: prefer_final_locals
    Object? postBody = queueState;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'PUT',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Update queue state
  ///
  /// Parameters:
  ///
  /// * [QueueState] queueState (required):
  Future<QueueState?> updateQueueState(QueueState queueState,) async {
    final response = await updateQueueStateWithHttpInfo(queueState,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'QueueState',) as QueueState;
    
    }
    return null;
  }

  /// Update task
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [int] taskId (required):
  ///   Unique identifier of the task.
  ///
  /// * [TaskUpdateInput] taskUpdateInput (required):
  Future<Response> updateTaskWithHttpInfo(int taskId, TaskUpdateInput taskUpdateInput,) async {
    // ignore: prefer_const_declarations
    final path = r'/tasks/{taskId}'
      .replaceAll('{taskId}', taskId.toString());

    // ignore: prefer_final_locals
    Object? postBody = taskUpdateInput;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'PUT',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Update task
  ///
  /// Parameters:
  ///
  /// * [int] taskId (required):
  ///   Unique identifier of the task.
  ///
  /// * [TaskUpdateInput] taskUpdateInput (required):
  Future<Task?> updateTask(int taskId, TaskUpdateInput taskUpdateInput,) async {
    final response = await updateTaskWithHttpInfo(taskId, taskUpdateInput,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'Task',) as Task;
    
    }
    return null;
  }

  /// Update task group
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [int] taskGroupId (required):
  ///   Unique identifier of the task group.
  ///
  /// * [TaskGroupUpdateInput] taskGroupUpdateInput (required):
  Future<Response> updateTaskGroupWithHttpInfo(int taskGroupId, TaskGroupUpdateInput taskGroupUpdateInput,) async {
    // ignore: prefer_const_declarations
    final path = r'/task-groups/{taskGroupId}'
      .replaceAll('{taskGroupId}', taskGroupId.toString());

    // ignore: prefer_final_locals
    Object? postBody = taskGroupUpdateInput;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'PUT',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Update task group
  ///
  /// Parameters:
  ///
  /// * [int] taskGroupId (required):
  ///   Unique identifier of the task group.
  ///
  /// * [TaskGroupUpdateInput] taskGroupUpdateInput (required):
  Future<TaskGroup?> updateTaskGroup(int taskGroupId, TaskGroupUpdateInput taskGroupUpdateInput,) async {
    final response = await updateTaskGroupWithHttpInfo(taskGroupId, taskGroupUpdateInput,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'TaskGroup',) as TaskGroup;
    
    }
    return null;
  }
}
