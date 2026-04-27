import 'package:flutter/material.dart';

import 'src/rust_init.dart';
import 'src/features/pdf/ui/pdf_home_page.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustInit.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      //
      title: 'PDF Bridge App',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(useMaterial3: true),
      home: const PdfHomePage(),
    );
  }
}
