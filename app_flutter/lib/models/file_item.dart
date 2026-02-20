class FileItem {
  final String path;
  final String name;
  final int size;
  final bool isDir;
  final int category;

  const FileItem({required this.path, required this.name, required this.size, required this.isDir, required this.category});
}
