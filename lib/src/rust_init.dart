import 'rust/frb_generated.dart';

class RustInit {
  static Future<void> init() async {
    // Some FRB setups may need explicit initialization here.
    // Keep this wrapper so your app architecture stays clean.
    await RustLib.init();
  }
}
