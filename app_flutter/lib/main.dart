import 'package:flutter/material.dart';
import 'screens/home_screen.dart';

void main() {
  runApp(const WoxelApp());
}

class WoxelApp extends StatelessWidget {
  const WoxelApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Woxel',
      theme: ThemeData.light(useMaterial3: true),
      darkTheme: ThemeData.dark(useMaterial3: true),
      home: const HomeScreen(),
    );
  }
}
