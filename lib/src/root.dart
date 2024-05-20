import 'dart:io';
import 'package:bbqr_dart/src/utils/loader.dart';

Future<void> setCurrentDirectory() async {
  try {
    await Dylib.downloadUnitTestDylib(Directory.current.path);
  } catch (e) {
    print(e.toString());
  }
}
