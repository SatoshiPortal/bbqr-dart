import 'dart:convert';
import 'dart:developer';

import 'package:dart_bbqr/bbqr.dart' as bbqr;
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await bbqr.LibBbqr.init();
  testPackage();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  String part = '';

  void _processQr() async {
    part = await qrCode();
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[Text(part)],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _processQr,
        tooltip: 'Create QR',
        child: const Icon(Icons.add),
      ),
    );
  }
}

void testPackage() async {
  try {
    await bbqr.LibBbqr.init();

    const template = "bacon bacon bacon bacon bacon bacon bacon bacon bacon";

    final large = template * 100;
    final bytes = utf8.encode(large);

    // examples of different options
    final options = await bbqr.SplitOptions.default_();

    final options2 = bbqr.SplitOptions(
      encoding: bbqr.Encoding.zlib,
      minSplitNumber: BigInt.from(1),
      maxSplitNumber: BigInt.from(1000),
      minVersion: bbqr.Version.v01,
      maxVersion: bbqr.Version.v40,
    );

    final split = await bbqr.Split.tryFromData(
      bytes: bytes,
      fileType: bbqr.FileType.unicodeText,
      options: options,
    );

    final split2 = await bbqr.Split.tryFromData(
      bytes: bytes,
      fileType: bbqr.FileType.unicodeText,
      options: options2,
    );

    // options2 is the same as defaultSplitOptions
    assert(split.parts.length == split2.parts.length);
    assert(listEquals(split.parts, split2.parts));

    // // continuous joiner with single part
    // final joiner = bbqr.ContinuousJoiner();

    // assert(split.parts.length == 1);
    // final part = split.parts.first;

    final result = await bbqr.Joined.tryFromParts(parts: split.parts);
    print(result);

    // if (result case bbqr.JoinResult_Complete(:final joined)) {
    //   assert(listEquals(joined.data, bytes));
    // }

    // // continuous joiner with multiple parts
    // final joiner2 = bbqr.ContinuousJoiner();

    final options3 = bbqr.SplitOptions(
      encoding: bbqr.Encoding.hex,
      minSplitNumber: BigInt.from(1),
      maxSplitNumber: BigInt.from(1000),
      minVersion: bbqr.Version.v01,
      maxVersion: bbqr.Version.v10,
    );

    final split3 = await bbqr.Split.tryFromData(
      bytes: bytes,
      fileType: bbqr.FileType.unicodeText,
      options: options3,
    );

    assert(split3.parts.length > 1);

    // bool isComplete = false;
    // for (final part in split3.parts) {
    //   final result = joiner2.addPart(part_: part);

    //   if (result case bbqr.JoinResult_Complete(:final joined)) {
    //     isComplete = true;
    //     assert(listEquals(joined.data, bytes));
    //   }
    // }
    // assert(isComplete);

    // join all at once
    final joined = await bbqr.Joined.tryFromParts(parts: split3.parts);
    assert(listEquals(joined.data, bytes));
  } catch (e) {
    log(e.toString());
  }
}

Future<String> qrCode() async {
  const template = "bacon bacon bacon bacon bacon bacon bacon bacon bacon";
  final large = template * 100;
  final bytes = utf8.encode(large);

  final options = await bbqr.SplitOptions.default_();

  final split = await bbqr.Split.tryFromData(
    bytes: bytes,
    fileType: bbqr.FileType.unicodeText,
    options: options,
  );

  return split.parts.first;
}
