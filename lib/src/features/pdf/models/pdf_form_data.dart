class PdfFormData {
  final String title;
  final String body;
  final String? author;

  const PdfFormData({required this.title, required this.body, this.author});

  PdfFormData copyWith({String? title, String? body, String? author}) {
    return PdfFormData(title: title ?? this.title, body: body ?? this.body, author: author ?? this.author);
  }
}
