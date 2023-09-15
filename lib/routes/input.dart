import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:proportional_cost_splitter_app/messages/calculate_action.pb.dart'
    as calculate_action;
import 'package:rust_in_flutter/rust_in_flutter.dart';

class CostEntry {
  String name;
  double cost;
  CostEntry(this.name, this.cost);
}

InputDecoration textInputDecoration(String hint) {
  return InputDecoration(
    fillColor: Colors.white,
    hintText: hint,
    border: OutlineInputBorder(
      borderRadius: BorderRadius.circular(10),
    ),
  );
}

class InputScreen extends StatefulWidget {
  const InputScreen({
    super.key,
  });

  @override
  State<InputScreen> createState() => _InputScreenState();
}

class _InputScreenState extends State<InputScreen> {
  List<CostEntry> costEntries = [];

  TextEditingController currentNameController = TextEditingController();
  TextEditingController currentCostController = TextEditingController();
  TextEditingController finalCostController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.white,
      body: Center(
          // Center is a layout widget. It takes a single child and positions it
          // in the middle of the parent.
          child: Column(
        children: [
          Expanded(
            child: ListView.builder(
                itemCount: costEntries.length,
                itemBuilder: (context, index) {
                  return Container(
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
                          costEntries[index].name,
                          style: const TextStyle(
                              fontSize: 28, color: Colors.white),
                        ),
                        Text(
                          costEntries[index].cost.toString(),
                          style: const TextStyle(
                              fontSize: 28, color: Colors.white),
                        ),
                        CloseButton(
                          onPressed: () {
                            costEntries.removeAt(index);
                            setState(() {});
                          },
                          color: Colors.redAccent,
                        )
                      ],
                    ),
                  );
                }),
          ),
          Row(
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              Expanded(
                child: Container(
                  margin:
                      const EdgeInsets.symmetric(vertical: 2, horizontal: 4),
                  child: TextField(
                    decoration: textInputDecoration("Name"),
                    controller: currentNameController,
                  ),
                ),
              ),
              Expanded(
                child: Container(
                  margin:
                      const EdgeInsets.symmetric(vertical: 2, horizontal: 4),
                  child: TextField(
                    decoration: textInputDecoration("Cost"),
                    controller: currentCostController,
                    keyboardType: TextInputType.number,
                    inputFormatters: [FilteringTextInputFormatter.digitsOnly],
                  ),
                ),
              ),
              Container(
                margin: const EdgeInsets.symmetric(vertical: 2, horizontal: 4),
                child: TextButton(
                    style: TextButton.styleFrom(
                        backgroundColor: Colors.transparent,
                        shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(10),
                        )),
                    onPressed: () {
                      if (currentNameController.text.isNotEmpty &&
                          currentCostController.text.isNotEmpty) {
                        double? maybeCurrentCost =
                            double.tryParse(currentCostController.text);
                        if (maybeCurrentCost != null) {
                          costEntries.add(CostEntry(
                              currentNameController.text, maybeCurrentCost));
                          currentNameController.clear();
                          currentCostController.clear();
                          setState(() {});
                        }
                      }
                    },
                    child: const Text("Add")),
              )
            ],
          ),
          Container(
            margin: const EdgeInsets.symmetric(vertical: 2, horizontal: 4),
            child: TextField(
                decoration: textInputDecoration("Total"),
                controller: finalCostController,
                keyboardType: TextInputType.number,
                inputFormatters: [FilteringTextInputFormatter.digitsOnly]),
          )
        ],
      )),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          double? finalCost = double.tryParse(finalCostController.text);
          if (finalCost != null && costEntries.isNotEmpty) {
            var initialCostsInput = costEntries.map((e) =>
                calculate_action.CostEntry(name: e.name, initialCost: e.cost));

            currentCostController.clear();
            currentNameController.clear();
            finalCostController.clear();
            costEntries.clear();

            setState(() {});

            var calculateRequest = calculate_action.CalculateAction(
                initialCosts: initialCostsInput, finalTotalCost: finalCost);

            final rustRequest = RustRequest(
              resource: calculate_action.ID,
              operation: RustOperation.Read,
              message: calculateRequest.writeToBuffer(),
            );

            requestToRust(rustRequest);
          }
        },
        tooltip: 'calculate',
        child: const Icon(Icons.calculate),
      ), // This trailing comma makes auto-formatting nicer for build methods.
    );
  }
}
