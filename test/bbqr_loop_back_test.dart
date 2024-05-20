// ignore_for_file: avoid_print
import 'package:bbqr_dart/bbqr.dart';
import 'package:test/test.dart';
import 'dart:convert';

void main() {
  group('BBQr', () {
    test('Split and Join', () async {
      await LibBbqr.init();

      const template =
          "bacon bacon bacon bacon bacon bacon bacon bacon bacon";

      String large = template * 1000;
      List<int> bytes = utf8.encode(large);

      SplitOptions options = defaultSplitOptions();
      Split split = await Split.tryFromData(data: bytes, fileType: FileType.unicodeText, options: options);

      ContinuousJoiner joiner = await ContinuousJoiner();

    });
  });
}
