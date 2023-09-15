import 'package:flutter/material.dart';
import 'package:proportional_cost_splitter_app/main.dart';
import 'package:proportional_cost_splitter_app/messages/calculate.pb.dart'
    as calculate;
import 'package:proportional_cost_splitter_app/messages/result_signal.pb.dart'
    as result_signal;
import 'package:rust_in_flutter/rust_in_flutter.dart';

class Result extends StatelessWidget {
  final List<CostEntry> costEntries;
  final double finalCost;

  const Result({super.key, required this.costEntries, required this.finalCost});

  @override
  Widget build(BuildContext context) {
    var initialCostsInput = costEntries
        .map((e) => calculate.RustCostEntry(name: e.name, initialCost: e.cost));

    var calculateRequest = calculate.RustCalculateRequest(
        initialCosts: initialCostsInput, finalTotalCost: finalCost);

    final rustRequest = RustRequest(
      resource: calculate.ID,
      operation: RustOperation.Read,
      bytes: calculateRequest.writeToBuffer(),
    );

    requestToRust(rustRequest);

    return StreamBuilder<RustSignal>(
        stream: rustBroadcaster.stream.where((rustSignal) {
      return rustSignal.resource == result_signal.ID;
    }), builder: (context, snapshot) {
      if (!snapshot.hasData) {
        return const Center(
          child: CircularProgressIndicator(),
        );
      } else {
        var response =
            result_signal.ResultSignal.fromBuffer(snapshot.data!.bytes);
        return Scaffold(
          backgroundColor: Colors.white,
          body: Center(
            child: ListView(
              children: response.finalCosts
                  .map((entry) => Container(
                        margin: const EdgeInsets.symmetric(
                            vertical: 5, horizontal: 10),
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
              Navigator.pop(context);
            },
            tooltip: 'calculate',
            child: const Icon(Icons.arrow_back_rounded),
          ),
        );
      }
    });
  }
}
