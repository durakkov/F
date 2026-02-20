import 'package:flutter_test/flutter_test.dart';
import 'package:woxel/main.dart';

void main() {
  testWidgets('Woxel renders', (tester) async {
    await tester.pumpWidget(const WoxelApp());
    expect(find.text('Woxel'), findsOneWidget);
  });
}
