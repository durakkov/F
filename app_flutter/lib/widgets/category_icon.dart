import 'package:flutter/material.dart';

class CategoryIcon extends StatelessWidget {
  final int category;
  const CategoryIcon({super.key, required this.category});

  @override
  Widget build(BuildContext context) {
    final icons = <int, IconData>{
      0: Icons.image,
      1: Icons.video_file,
      2: Icons.audio_file,
      3: Icons.description,
      4: Icons.archive,
      5: Icons.android,
      6: Icons.code,
    };
    return Icon(icons[category] ?? Icons.insert_drive_file);
  }
}
