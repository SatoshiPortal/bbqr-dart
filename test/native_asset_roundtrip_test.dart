import 'dart:convert';

import 'package:bbqr/bbqr.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  setUpAll(LibBbqr.init);
  tearDownAll(LibBbqr.dispose);

  test('round trips compressed JSON through Native Assets', () async {
    final payload = utf8.encode(
      jsonEncode({'data': List.filled(256, 'BBQr').join()}),
    );
    final split = await Split.tryFromData(
      bytes: payload,
      fileType: FileType.json,
      options: await SplitOptions.default_(),
    );
    expect(split.encoding, Encoding.zlib);
    final joined = await Joined.tryFromParts(parts: split.parts);
    expect(joined.encoding, Encoding.zlib);
    expect(joined.fileType, FileType.json);
    expect(joined.data, payload);
  });
}
