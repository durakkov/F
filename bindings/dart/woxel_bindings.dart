import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';

final class WoxelEntryNative extends Struct {
  @Array(1024)
  external Array<Uint8> path;

  @Array(256)
  external Array<Uint8> name;

  @Uint64()
  external int size;

  @Int64()
  external int modifiedMs;

  @Int32()
  external int isDir;

  @Int32()
  external int category;
}

final class WoxelBindings {
  late final DynamicLibrary _lib;
  late final int Function(Pointer<Utf8>, Pointer<WoxelEntryNative>, int) listDir;

  WoxelBindings() {
    _lib = Platform.isAndroid ? DynamicLibrary.open('libwoxel_core.so') : DynamicLibrary.process();
    listDir = _lib
        .lookupFunction<Int32 Function(Pointer<Utf8>, Pointer<WoxelEntryNative>, Int32), int Function(Pointer<Utf8>, Pointer<WoxelEntryNative>, int)>('woxel_list_dir');
  }
}
