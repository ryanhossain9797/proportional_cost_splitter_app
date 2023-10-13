import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:proportional_cost_splitter_app/main.dart';
import 'package:proportional_cost_splitter_app/messages/add_cost_entry_action.pb.dart'
    as add_cost_entry_action;
import 'package:proportional_cost_splitter_app/messages/calculate_action.pb.dart'
    as calculate_action;
import 'package:proportional_cost_splitter_app/messages/remove_cost_entry_action.pb.dart'
    as remove_cost_entry_action;
import 'package:proportional_cost_splitter_app/messages/state.pb.dart';
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
  final ReadingInputStateDto state;

  const InputScreen({super.key, required this.state});

  @override
  State<InputScreen> createState() => _InputScreenState();
}

class _InputScreenState extends State<InputScreen> {
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
                itemCount: widget.state.currentCostEntries.length,
                itemBuilder: (context, index) {
                  return Container(
                    margin: const EdgeInsets.symmetric(
                        vertical: verticalPadding,
                        horizontal: horizontalPadding),
                    padding: const EdgeInsets.symmetric(
                        vertical: verticalPadding,
                        horizontal: horizontalPadding),
                    decoration: BoxDecoration(
                      color: rowColor,
                      border: Border.all(
                        width: 2,
                      ),
                      borderRadius: BorderRadius.circular(15),
                    ),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                      children: [
                        Text(
                          widget.state.currentCostEntries[index].name,
                          style: const TextStyle(
                              fontSize: 28, color: Colors.white),
                        ),
                        Text(
                          widget.state.currentCostEntries[index].cost
                              .toString(),
                          style: const TextStyle(
                              fontSize: 28, color: Colors.white),
                        ),
                        CloseButton(
                          onPressed: () {
                            final removeCostEntryAction =
                                remove_cost_entry_action
                                    .RemoveCostEntryActionDto(
                                        name: widget.state
                                            .currentCostEntries[index].name);
                            final rustRequest = RustRequest(
                              resource: remove_cost_entry_action.ID,
                              operation: RustOperation.Update,
                              message: removeCostEntryAction.writeToBuffer(),
                            );

                            setState(() {});

                            requestToRust(rustRequest);
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
                          final addCostEntryAction =
                              add_cost_entry_action.AddCostEntryActionDto(
                                  name: currentNameController.text,
                                  initialCost: maybeCurrentCost);
                          final rustRequest = RustRequest(
                            resource: add_cost_entry_action.ID,
                            operation: RustOperation.Update,
                            message: addCostEntryAction.writeToBuffer(),
                          );

                          currentNameController.clear();
                          currentCostController.clear();
                          setState(() {});

                          requestToRust(rustRequest);
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
          finalCostController.clear();
          if (finalCost != null && widget.state.currentCostEntries.isNotEmpty) {
            var calculateAction =
                calculate_action.CalculateActionDto(finalTotalCost: finalCost);

            final rustRequest = RustRequest(
              resource: calculate_action.ID,
              operation: RustOperation.Update,
              message: calculateAction.writeToBuffer(),
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
