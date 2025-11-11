//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

library openapi.api;

import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:collection/collection.dart';
import 'package:http/http.dart';
import 'package:intl/intl.dart';
import 'package:meta/meta.dart';

part 'api_client.dart';
part 'api_helper.dart';
part 'api_exception.dart';
part 'auth/authentication.dart';
part 'auth/api_key_auth.dart';
part 'auth/oauth.dart';
part 'auth/http_basic_auth.dart';
part 'auth/http_bearer_auth.dart';

part 'api/default_api.dart';

part 'model/active_strategy.dart';
part 'model/append_files_config.dart';
part 'model/command_config.dart';
part 'model/commit_info.dart';
part 'model/config.dart';
part 'model/feed.dart';
part 'model/feed_level.dart';
part 'model/feed_order_field.dart';
part 'model/get_healthz200_response.dart';
part 'model/insert_message.dart';
part 'model/insert_message_one_of.dart';
part 'model/insert_message_one_of1.dart';
part 'model/message.dart';
part 'model/strategy.dart';
part 'model/task.dart';
part 'model/task_create_input.dart';
part 'model/task_dependency.dart';
part 'model/task_dependency_create_input.dart';
part 'model/task_group.dart';
part 'model/task_group_create_input.dart';
part 'model/task_group_status.dart';
part 'model/task_group_update_input.dart';
part 'model/task_status.dart';
part 'model/task_update_input.dart';
part 'model/worker.dart';
part 'model/worker_state.dart';


/// An [ApiClient] instance that uses the default values obtained from
/// the OpenAPI specification file.
var defaultApiClient = ApiClient();

const _delimiters = {'csv': ',', 'ssv': ' ', 'tsv': '\t', 'pipes': '|'};
const _dateEpochMarker = 'epoch';
const _deepEquality = DeepCollectionEquality();
final _dateFormatter = DateFormat('yyyy-MM-dd');
final _regList = RegExp(r'^List<(.*)>$');
final _regSet = RegExp(r'^Set<(.*)>$');
final _regMap = RegExp(r'^Map<String,(.*)>$');

bool _isEpochMarker(String? pattern) => pattern == _dateEpochMarker || pattern == '/$_dateEpochMarker/';
