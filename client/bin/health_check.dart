// ignore_for_file: avoid_print

import 'package:my_api_client/api.dart' as api;

Future<void> main() async {
  final client = api.ApiClient(basePath: 'http://localhost:8080');
  final defaultApi = api.DefaultApi(client);
  try {
    final response = await defaultApi.getHealthz();
    print('Response: ${response?.status}');
  } catch (e, st) {
    print('Error: $e');
    print(st);
  }
}
