import 'dart:io';

import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';

class FilePathService {
  Future<String> buildPdfOutputPath({String fileName = 'simple_demo.pdf'}) async {
    final Directory baseDir = await getApplicationDocumentsDirectory();
    final Directory pdfDir = Directory(p.join(baseDir.path, 'generated_pdfs'));

    if (!await pdfDir.exists()) {
      await pdfDir.create(recursive: true);
    }

    return p.join(pdfDir.path, fileName);
  }
}
