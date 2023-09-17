import 'package:flutter/material.dart';
import 'package:proportional_cost_splitter_app/messages/state.pb.dart' as state;
import 'package:proportional_cost_splitter_app/messages/reset_action.pb.dart'
    as reset_action;
import 'package:proportional_cost_splitter_app/routes/input.dart';
import 'package:proportional_cost_splitter_app/routes/result.dart';
import 'package:rust_in_flutter/rust_in_flutter.dart';

void main() async {
  await RustInFlutter.ensureInitialized();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Split the VAT',
      theme: ThemeData(
        // This is the theme of your application.
        //
        // TRY THIS: Try running your application with "flutter run". You'll see
        // the application has a blue toolbar. Then, without quitting the app,
        // try changing the seedColor in the colorScheme below to Colors.green
        // and then invoke "hot reload" (save your changes or press the "hot
        // reload" button in a Flutter-supported IDE, or press "r" if you used
        // the command line to start the app).
        //
        // Notice that the counter didn't reset back to zero; the application
        // state is not lost during the reload. To reset the state, use hot
        // restart instead.
        //
        // This works for code too, not just values: Most code changes can be
        // tested with just a hot reload.
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  // This widget is the home page of your application. It is stateful, meaning
  // that it has a State object (defined below) that contains fields that affect
  // how it looks.

  // This class is the configuration for the state. It holds the values (in this
  // case the title) provided by the parent (in this case the App widget) and
  // used by the build method of the State. Fields in a Widget subclass are
  // always marked "final".

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  @override
  Widget build(BuildContext context) {
    // This method is rerun every time setState is called, for instance as done
    // by the _incrementCounter method above.
    //
    // The Flutter framework has been optimized to make rerunning build methods
    // fast, so that you can just rebuild anything that needs updating rather
    // than having to individually change instances of widgets.

    final rustRequest = RustRequest(
      resource: reset_action.ID,
      operation: RustOperation.Read,
      message: reset_action.ResetAction().writeToBuffer(),
    );

    requestToRust(rustRequest);

    return StreamBuilder<RustSignal>(
        stream: rustBroadcaster.stream.where((rustSignal) {
      return rustSignal.resource == state.ID;
    }), builder: (context, snapshot) {
      if (!snapshot.hasData) {
        return const Center(
          child: CircularProgressIndicator(),
        );
      } else {
        var currentState = state.AppState.fromBuffer(snapshot.data!.message!);

        switch (currentState.whichState()) {
          case state.AppState_State.calculated:
            return ResultScreen(state: currentState.calculated);
          case state.AppState_State.readingInput:
          case state.AppState_State.notSet:
          default:
            return const InputScreen();
        }
      }
    });
  }
}
