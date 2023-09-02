import 'package:flutter/material.dart';
import 'package:proportional_cost_splitter_app/main.dart';

class Result extends StatelessWidget {
  final List<CostEntry> costEntries;
  final double finalCost;

  Result({super.key, required this.costEntries, required this.finalCost});

  @override
  Widget build(BuildContext context) {
    double total =
      costEntries
        .map((e) => e.cost)
        .toList()
        .reduce((value, element) => value + element);

    double scaling = finalCost / total;
    return Scaffold(
      backgroundColor: Colors.white,
      body: Center(
        child: ListView(
          children: costEntries.map((entry) =>
            Container(
              margin: const EdgeInsets.symmetric(vertical: 5, horizontal: 10),
              padding: const EdgeInsets.symmetric(vertical: 15, horizontal: 15),

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
                  Text(entry.name, style: const TextStyle(fontSize: 28, color: Colors.white),),
                  Text(entry.cost.toStringAsFixed(2), style: const TextStyle(fontSize: 28, color: Colors.white),),
                  Text((entry.cost * scaling ).toStringAsFixed(2), style: const TextStyle(fontSize: 28, color: Colors.white),),
                ],
              ),
            )
          ).toList(),
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: (){
          Navigator.pop(context);
        },
        tooltip: 'calculate',
        child: const Icon(Icons.arrow_back_rounded),
      ),
    );
  }
}
