import 'dart:ffi';
import 'package:ffi/ffi.dart';
import '../models/file_item.dart';
import '../../../bindings/dart/woxel_bindings.dart';

class WoxelClient {
  final _bindings = WoxelBindings();

  List<FileItem> listDir(String path) {
    final nativePath = path.toNativeUtf8();
    final entries = calloc<WoxelEntryNative>(512);
    final count = _bindings.listDir(nativePath, entries, 512);
    calloc.free(nativePath);
    if (count < 0) {
      calloc.free(entries);
      return const [];
    }
    final out = <FileItem>[];
    for (var i = 0; i < count; i++) {
      out.add(FileItem(path: path, name: 'item_$i', size: entries[i].size, isDir: entries[i].isDir == 1, category: entries[i].category));
    }
    calloc.free(entries);
    return out;
  }
}
