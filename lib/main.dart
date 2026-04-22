import 'package:flutter/material.dart';
import 'package:pdf_bridge_app/src/rust/rust_init.dart';
import 'src/app.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustInit.init();
  runApp(const MyApp());
}
