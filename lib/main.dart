import 'package:flutter/material.dart';
import 'package:proportional_cost_splitter_app/messages/state.pb.dart' as state;
import 'package:proportional_cost_splitter_app/messages/reset_action.pb.dart'
    as reset_action;
import 'package:proportional_cost_splitter_app/routes/input.dart';
import 'package:proportional_cost_splitter_app/routes/result.dart';
import 'package:rust_in_flutter/rust_in_flutter.dart';

const double verticalPadding = 5;
const double horizontalPadding = 15;
const Color rowColor = Colors.black38;

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
  @override
  Widget build(BuildContext context) {
    final rustRequest = RustRequest(
      resource: reset_action.ID,
      operation: RustOperation.Update,
      message: reset_action.ResetActionDto().writeToBuffer(),
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
        var currentState =
            state.AppStateDto.fromBuffer(snapshot.data!.message!);

        switch (currentState.whichState()) {
          case state.AppStateDto_State.calculated:
            return ResultScreen(state: currentState.calculated);
          case state.AppStateDto_State.readingInput:
            return InputScreen(
              state: currentState.readingInput,
            );
          case state.AppStateDto_State.notSet:
          default:
            return const Scaffold(
              body: Center(child: Text("Fatal Error")),
            );
        }
      }
    });
  }
}
