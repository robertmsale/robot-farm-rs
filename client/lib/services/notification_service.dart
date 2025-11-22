import 'package:flutter_local_notifications/flutter_local_notifications.dart';

class NotificationService {
  NotificationService._();

  static final NotificationService instance = NotificationService._();
  final FlutterLocalNotificationsPlugin _plugin =
      FlutterLocalNotificationsPlugin();
  bool _initialized = false;

  Future<void> init() async {
    if (_initialized) return;

    const iosSettings = DarwinInitializationSettings();
    const initSettings = InitializationSettings(iOS: iosSettings);
    await _plugin.initialize(initSettings);
    _initialized = true;
  }

  Future<bool> ensurePermission() async {
    await init();
    final granted = await _plugin
        .resolvePlatformSpecificImplementation<
            IOSFlutterLocalNotificationsPlugin>()
        ?.requestPermissions(alert: true, badge: true, sound: true);
    return granted ?? false;
  }

  Future<void> showLocal({
    required String title,
    required String body,
    int id = 0,
  }) async {
    await init();
    const details = NotificationDetails(
      iOS: DarwinNotificationDetails(presentAlert: true, presentSound: true),
    );
    await _plugin.show(id, title, body, details);
  }
}
