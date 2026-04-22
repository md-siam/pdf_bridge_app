import 'package:flutter/material.dart';

import '../controller/pdf_controller.dart';
import '../models/pdf_form_data.dart';
import '../service/file_open_service.dart';
import '../service/file_path_service.dart';
import '../service/pdf_service.dart';

class PdfHomePage extends StatefulWidget {
  const PdfHomePage({super.key});

  @override
  State<PdfHomePage> createState() => _PdfHomePageState();
}

class _PdfHomePageState extends State<PdfHomePage> {
  late final PdfController _controller;
  final _titleController = TextEditingController(text: 'Hello from Rust');
  final _bodyController = TextEditingController(text: 'This PDF was generated in Rust and opened from Flutter.');
  final _authorController = TextEditingController(text: 'Md. Siam');

  @override
  void initState() {
    super.initState();

    _controller =
        PdfController(
          pdfService: PdfService(filePathService: FilePathService()),
          fileOpenService: FileOpenService(),
        )..addListener(() {
          if (mounted) setState(() {});
        });
  }

  @override
  void dispose() {
    _controller.dispose();
    _titleController.dispose();
    _bodyController.dispose();
    _authorController.dispose();
    super.dispose();
  }

  Future<void> _onGeneratePressed() async {
    final formData = PdfFormData(title: _titleController.text.trim(), body: _bodyController.text.trim(), author: _authorController.text.trim().isEmpty ? null : _authorController.text.trim());

    await _controller.generateAndOpenPdf(formData);
  }

  @override
  Widget build(BuildContext context) {
    final state = _controller.state;

    return Scaffold(
      appBar: AppBar(title: const Text('Flutter + Rust PDF')),
      body: SingleChildScrollView(
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            children: [
              TextField(
                controller: _titleController,
                decoration: const InputDecoration(labelText: 'PDF title', border: OutlineInputBorder()),
              ),
              const SizedBox(height: 12),
              TextField(
                controller: _authorController,
                decoration: const InputDecoration(labelText: 'Author', border: OutlineInputBorder()),
              ),
              const SizedBox(height: 12),
              TextField(
                controller: _bodyController,
                maxLines: 6,
                decoration: const InputDecoration(labelText: 'PDF body', border: OutlineInputBorder()),
              ),
              const SizedBox(height: 16),
              SizedBox(
                width: double.infinity,
                child: FilledButton(
                  onPressed: state.isLoading ? null : _onGeneratePressed,
                  child: state.isLoading ? const SizedBox(height: 20, width: 20, child: CircularProgressIndicator(strokeWidth: 2)) : const Text('Generate PDF'),
                ),
              ),
              const SizedBox(height: 16),
              if (state.message != null) Text(state.message!, style: const TextStyle(color: Colors.green)),
              if (state.generatedPath != null) ...[const SizedBox(height: 8), SelectableText('Saved: ${state.generatedPath}')],
              if (state.error != null) ...[const SizedBox(height: 8), Text(state.error!, style: const TextStyle(color: Colors.red))],
            ],
          ),
        ),
      ),
    );
  }
}
