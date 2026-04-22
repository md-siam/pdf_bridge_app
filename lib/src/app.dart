import 'package:flutter/material.dart';
import 'features/pdf/ui/pdf_home_page.dart';

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(title: 'PDF Bridge App', debugShowCheckedModeBanner: false, theme: ThemeData(useMaterial3: true), home: const PdfHomePage());
  }
}
