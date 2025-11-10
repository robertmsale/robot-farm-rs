import 'package:client/main.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:get/get.dart';

void main() {
  setUp(() {
    Get.reset();
  });

  testWidgets('Displays connection form', (tester) async {
    Get.put(ConnectionController(), permanent: true);

    await tester.pumpWidget(const RobotFarmApp());

    expect(find.text('Robot Farm Client'), findsOneWidget);
    expect(find.text('Server host'), findsOneWidget);
    expect(find.text('Connect'), findsOneWidget);
  });
}
