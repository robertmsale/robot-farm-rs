import 'package:my_api_client/api.dart' as robot_farm_api;

class QueueMessageViewModel {
  const QueueMessageViewModel(this.message);

  final robot_farm_api.Message message;

  String get title => message.message;
  String get subtitle => 'from ${message.from} → ${message.to}';
}
