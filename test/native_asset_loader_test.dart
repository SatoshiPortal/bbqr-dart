import 'package:flutter_test/flutter_test.dart';
import 'package:bbqr/bbqr.dart';

void main() {
  test('loads the default Native Assets library without an override', () async {
    await LibBbqr.init();
  });
}
