import 'dart:typed_data';

class PdfFormData {
  final String title;
  final String body;
  final String? author;
  final Uint8List? imageBytes;

  const PdfFormData({required this.title, required this.body, this.author, this.imageBytes});

  PdfFormData copyWith({String? title, String? body, String? author, Uint8List? imageBytes}) {
    return PdfFormData(title: title ?? this.title, body: body ?? this.body, author: author ?? this.author, imageBytes: imageBytes ?? this.imageBytes);
  }
}
