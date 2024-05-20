// ignore_for_file: avoid_print
import 'package:bbqr_dart/bbqr.dart';
import 'package:test/test.dart';
import 'dart:convert';
import 'package:flutter/foundation.dart';

void main() {
  group('BBQr', () {
    test('Split and Join', () async {
      await LibBbqr.init();

      const template = "bacon bacon bacon bacon bacon bacon bacon bacon bacon";

      String large = template * 100;
      List<int> bytes = utf8.encode(large);

      SplitOptions options = defaultSplitOptions();

      Split split = await Split.tryFromData(
          data: bytes, fileType: FileType.unicodeText, options: options);

      SplitOptions options2 = SplitOptions(
          encoding: Encoding.zlib,
          minSplitNumber: 1,
          maxSplitNumber: 1000,
          minVersion: Version.v01,
          maxVersion: Version.v40);

      Split split2 = await Split.tryFromData(
          data: bytes, fileType: FileType.unicodeText, options: options2);

      assert(split.parts().length == split2.parts().length);
      assert(split.parts().length == 1);

      ContinuousJoiner joiner = ContinuousJoiner();

      String part = split.parts().first;

      JoinResult result = joiner.addPart(part: part);

      if (result case JoinResult_Complete(:final joined)) {
        assert(listEquals(joined.data, bytes));
      }
      ;

      ContinuousJoiner joiner2 = ContinuousJoiner();

      SplitOptions options3 = SplitOptions(
          encoding: Encoding.hex,
          minSplitNumber: 1,
          maxSplitNumber: 1000,
          minVersion: Version.v01,
          maxVersion: Version.v10);

      Split split3 = await Split.tryFromData(
          data: bytes, fileType: FileType.unicodeText, options: options3);

      for (String part in split3.parts()) {
        JoinResult result = joiner2.addPart(part: part);

        if (result case JoinResult_Complete(:final joined)) {
          assert(listEquals(joined.data, bytes));
        }
      }
    });
  });
}
