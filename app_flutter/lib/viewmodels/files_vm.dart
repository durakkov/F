import 'package:flutter/foundation.dart';
import '../ffi/woxel_client.dart';
import '../models/file_item.dart';

class FilesVm extends ChangeNotifier {
  final WoxelClient _client = WoxelClient();
  List<FileItem> items = const [];
  String currentPath = '/storage/emulated/0';

  void load([String? path]) {
    if (path != null) currentPath = path;
    items = _client.listDir(currentPath);
    notifyListeners();
  }

  void searchLocal(String q) {
    items = items.where((e) => e.name.toLowerCase().contains(q.toLowerCase())).toList();
    notifyListeners();
  }
}
