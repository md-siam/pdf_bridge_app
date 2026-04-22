class PdfGenerationState {
  final bool isLoading;
  final String? generatedPath;
  final String? message;
  final String? error;

  const PdfGenerationState({required this.isLoading, this.generatedPath, this.message, this.error});

  factory PdfGenerationState.initial() {
    return const PdfGenerationState(isLoading: false);
  }

  PdfGenerationState copyWith({bool? isLoading, String? generatedPath, String? message, String? error}) {
    return PdfGenerationState(isLoading: isLoading ?? this.isLoading, generatedPath: generatedPath ?? this.generatedPath, message: message ?? this.message, error: error);
  }
}
