import 'package:bbqr/bbqr.dart';
import 'dart:convert';
import 'package:flutter/foundation.dart';
import 'package:integration_test/integration_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:bbqr/bbqr.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  setUpAll(() async => await LibBbqr.init());

  group('BBQr', () {
    test('Split and Join', () async {
      await LibBbqr.init();

      const template = "bacon bacon bacon bacon bacon bacon bacon bacon bacon";

      String large = template * 100;
      List<int> bytes = utf8.encode(large);

      // examples of different options
      SplitOptions options = defaultSplitOptions();

      SplitOptions options2 = SplitOptions(
        encoding: Encoding.zlib,
        minSplitNumber: 1,
        maxSplitNumber: 1000,
        minVersion: Version.v01,
        maxVersion: Version.v40,
      );

      Split split = Split.tryFromData(
        data: bytes,
        fileType: FileType.unicodeText,
        options: options,
      );

      Split split2 = Split.tryFromData(
        data: bytes,
        fileType: FileType.unicodeText,
        options: options2,
      );

      // options2 is the same as defaultSplitOptions
      assert(split.parts().length == split2.parts().length);
      assert(listEquals(split.parts(), split2.parts()));

      // continuous joiner with single part
      ContinuousJoiner joiner = ContinuousJoiner();

      assert(split.parts().length == 1);
      String part = split.parts().first;

      JoinResult result = joiner.addPart(part: part);

      if (result case JoinResult_Complete(:final joined)) {
        assert(listEquals(joined.data, bytes));
      }
      ;

      // continuous joiner with multiple parts
      ContinuousJoiner joiner2 = ContinuousJoiner();

      SplitOptions options3 = SplitOptions(
        encoding: Encoding.hex,
        minSplitNumber: 1,
        maxSplitNumber: 1000,
        minVersion: Version.v01,
        maxVersion: Version.v10,
      );

      Split split3 = Split.tryFromData(
        data: bytes,
        fileType: FileType.unicodeText,
        options: options3,
      );

      assert(split3.parts().length > 1);

      bool isComplete = false;
      for (String part in split3.parts()) {
        JoinResult result = joiner2.addPart(part: part);

        if (result case JoinResult_Complete(:final joined)) {
          isComplete = true;
          assert(listEquals(joined.data, bytes));
        }
      }

      assert(isComplete);

      // join all at once
      Joined joined = Joined.tryNewFromParts(parts: split3.parts());
      assert(listEquals(joined.data, bytes));
    });
  });
}
