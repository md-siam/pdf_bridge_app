import 'package:open_filex/open_filex.dart';

class FileOpenService {
  Future<void> openFile(String path) async {
    final result = await OpenFilex.open(path);

    if (result.type != ResultType.done) {
      throw Exception('Failed to open file: ${result.message}');
    }
  }
}
