import 'package:flutter/material.dart';
import 'package:proportional_cost_splitter_app/messages/state.pbserver.dart';
import 'package:proportional_cost_splitter_app/messages/reset_action.pb.dart'
    as reset_action;
import 'package:rust_in_flutter/rust_in_flutter.dart';

class ResultScreen extends StatelessWidget {
  final CalculatedStateDto state;

  const ResultScreen({super.key, required this.state});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.white,
      body: Center(
        child: ListView(
          children: state.finalCosts
              .map((entry) => Container(
                    margin:
                        const EdgeInsets.symmetric(vertical: 5, horizontal: 10),
                    padding: const EdgeInsets.symmetric(
                        vertical: 15, horizontal: 15),
                    decoration: BoxDecoration(
                      color: Colors.deepPurple,
                      border: Border.all(
                        width: 2,
                      ),
                      borderRadius: BorderRadius.circular(15),
                    ),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                      children: [
                        Text(
                          entry.name,
                          style: const TextStyle(
                              fontSize: 28, color: Colors.white),
                        ),
                        Text(
                          (entry.finalCost).toStringAsFixed(2),
                          style: const TextStyle(
                              fontSize: 28, color: Colors.white),
                        ),
                      ],
                    ),
                  ))
              .toList(),
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          final rustRequest = RustRequest(
            resource: reset_action.ID,
            operation: RustOperation.Read,
            message: reset_action.ResetActionDto().writeToBuffer(),
          );

          requestToRust(rustRequest);
        },
        tooltip: 'calculate',
        child: const Icon(Icons.arrow_back_rounded),
      ),
    );
  }
}
