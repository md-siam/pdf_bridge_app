import 'package:flutter/foundation.dart';

import '../models/pdf_form_data.dart';
import '../models/pdf_generation_state.dart';
import '../service/file_open_service.dart';
import '../service/pdf_service.dart';

class PdfController extends ChangeNotifier {
  PdfController({required PdfService pdfService, required FileOpenService fileOpenService}) : _pdfService = pdfService, _fileOpenService = fileOpenService;

  final PdfService _pdfService;
  final FileOpenService _fileOpenService;

  PdfGenerationState _state = PdfGenerationState.initial();
  PdfGenerationState get state => _state;

  Future<void> generateAndOpenPdf(PdfFormData formData) async {
    _state = _state.copyWith(isLoading: true, error: null, message: null);
    notifyListeners();

    try {
      final response = await _pdfService.generatePdf(formData);
      await _fileOpenService.openFile(response.filePath);

      _state = _state.copyWith(isLoading: false, generatedPath: response.filePath, message: 'PDF created and opened successfully.', error: null);
    } catch (e) {
      _state = _state.copyWith(isLoading: false, error: e.toString());
    }

    notifyListeners();
  }
}
