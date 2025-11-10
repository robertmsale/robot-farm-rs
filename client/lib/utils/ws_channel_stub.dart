import 'package:web_socket_channel/web_socket_channel.dart';

WebSocketChannel createPlatformWebSocketChannel(Uri uri) =>
    throw UnsupportedError('WebSockets are not supported on this platform.');
