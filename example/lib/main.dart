import 'dart:convert';
import 'dart:developer';

import 'package:bbqr_dart/bbqr.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

void main() {
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
  int _counter = 0;

  void _incrementCounter() {
    setState(() {
      _counter++;
    });
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
          children: <Widget>[
            const Text(
              'You have pushed the button this many times:',
            ),
            Text(
              '$_counter',
              style: Theme.of(context).textTheme.headlineMedium,
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _incrementCounter,
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}

void testPackage() async {
  try {
    await LibBbqr.init();

    const template = "bacon bacon bacon bacon bacon bacon bacon bacon bacon";

    String large = template * 100;
    List<int> bytes = utf8.encode(large);

    // examples of different options
    SplitOptions options = defaultSplitOptions();

    const options2 = SplitOptions(
        encoding: Encoding.zlib,
        minSplitNumber: 1,
        maxSplitNumber: 1000,
        minVersion: Version.v01,
        maxVersion: Version.v40);

    Split split = await Split.tryFromData(
        data: bytes, fileType: FileType.unicodeText, options: options);

    Split split2 = await Split.tryFromData(
        data: bytes, fileType: FileType.unicodeText, options: options2);

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

    // continuous joiner with multiple parts
    ContinuousJoiner joiner2 = ContinuousJoiner();

    const options3 = SplitOptions(
        encoding: Encoding.hex,
        minSplitNumber: 1,
        maxSplitNumber: 1000,
        minVersion: Version.v01,
        maxVersion: Version.v10);

    Split split3 = await Split.tryFromData(
        data: bytes, fileType: FileType.unicodeText, options: options3);

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
  } catch (e) {
    log(e.toString());
  }
}
