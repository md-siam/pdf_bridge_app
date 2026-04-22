import 'package:pdf_bridge_app/src/rust/api/pdf_api.dart';
import 'package:pdf_bridge_app/src/rust/domain/models.dart';

import '../../pdf/models/pdf_form_data.dart';
import 'file_path_service.dart';

class PdfService {
  PdfService({
    required FilePathService filePathService,
  }) : _filePathService = filePathService;

  final FilePathService _filePathService;

  Future<CreatePdfResponse> generatePdf(PdfFormData formData) async {
    final String outputPath = await _filePathService.buildPdfOutputPath();

    final request = CreatePdfRequest(
      outputPath: outputPath,
      title: formData.title,
      body: formData.body,
      author: formData.author,
    );

    final response = await createSimplePdf(request: request);
    return response;
  }
}