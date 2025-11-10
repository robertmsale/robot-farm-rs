import 'package:web_socket_channel/web_socket_channel.dart';

import 'ws_channel_stub.dart'
    if (dart.library.io) 'ws_channel_io.dart'
    if (dart.library.html) 'ws_channel_web.dart';

WebSocketChannel createWebSocketChannel(Uri uri) =>
    createPlatformWebSocketChannel(uri);
