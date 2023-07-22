import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:proportional_cost_splitter_app/routes/result.dart';

void main() {
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

class CostEntry {
  String name;
  double cost;
  CostEntry(this.name, this.cost);
}

InputDecoration textInputDecoration(String hint) {
  return
    InputDecoration(
      fillColor: Colors.white,
      hintText: hint,
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(10),
      ),
    );
}

class _MyHomePageState extends State<MyHomePage> {
  List<CostEntry> costEntries = [];

  TextEditingController currentNameController = TextEditingController();
  TextEditingController currentCostController = TextEditingController();
  TextEditingController finalCostController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    // This method is rerun every time setState is called, for instance as done
    // by the _incrementCounter method above.
    //
    // The Flutter framework has been optimized to make rerunning build methods
    // fast, so that you can just rebuild anything that needs updating rather
    // than having to individually change instances of widgets.
    return Scaffold(
      backgroundColor: Colors.white,
      body: Center(
        // Center is a layout widget. It takes a single child and positions it
        // in the middle of the parent.
        child: 
          Column(
            children: [
              Expanded(
                child: ListView.builder(
                  itemCount: costEntries.length,
                  itemBuilder: (context, index) {
                    return
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
                            Text(costEntries[index].name, style: const TextStyle(fontSize: 28, color: Colors.white),),
                            Text(costEntries[index].cost.toString(), style: const TextStyle(fontSize: 28, color: Colors.white),),
                            CloseButton(onPressed: () {costEntries.removeAt(index); setState(() {});}, color: Colors.redAccent,)
                          ],
                        ),
                      );
                  }
                ),
              ),
              Row(
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Expanded(
                    child: Container(
                      margin: const EdgeInsets.symmetric(vertical: 2, horizontal: 4),
                      child: TextField(
                        decoration: textInputDecoration("Name"),
                        controller: currentNameController,
                      ),
                    ),
                  ),
                  Expanded(
                    child: Container(
                      margin: const EdgeInsets.symmetric(vertical: 2, horizontal: 4),
                      child: TextField(
                        decoration: textInputDecoration("Cost"),
                        controller: currentCostController,
                        keyboardType: TextInputType.number,
                        inputFormatters: [ FilteringTextInputFormatter.digitsOnly ],
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
                      onPressed: (){
                        if (currentNameController.text.isNotEmpty && currentCostController.text.isNotEmpty) {
                          double? maybeCurrentCost = double.tryParse(currentCostController.text);
                          if (maybeCurrentCost != null) {
                            costEntries.add(CostEntry(currentNameController.text, maybeCurrentCost));
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
                  inputFormatters: [ FilteringTextInputFormatter.digitsOnly ]
                ),
              )
            ],
          )
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: (){
          double? finalCost = double.tryParse(finalCostController.text);
          if(finalCost != null && costEntries.isNotEmpty) {
            var costEntriesCopy = List.of(costEntries);
            currentCostController.clear();
            currentNameController.clear();
            finalCostController.clear();
            costEntries.clear();
            setState(() {});
            Navigator.push(
              context,
              MaterialPageRoute(builder: (context) => Result(finalCost: finalCost, costEntries: costEntriesCopy,)),
            );
          }
        },
        tooltip: 'calculate',
        child: const Icon(Icons.calculate),
      ), // This trailing comma makes auto-formatting nicer for build methods.
    );
  }
}
