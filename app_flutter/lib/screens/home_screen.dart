import 'package:flutter/material.dart';
import '../viewmodels/files_vm.dart';
import '../widgets/category_icon.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  final vm = FilesVm();
  int tab = 0;

  @override
  void initState() {
    super.initState();
    vm.load();
    vm.addListener(() => setState(() {}));
  }

  @override
  Widget build(BuildContext context) {
    final pages = [
      _filesPage(),
      const Center(child: Text('Search/Index tasks via C++ core')),
      const Center(child: Text('Task queue progress + notifications')),
      const Center(child: Text('Settings / permissions / SAF grants')),
    ];
    return Scaffold(
      appBar: AppBar(title: const Text('Woxel')),
      body: pages[tab],
      bottomNavigationBar: NavigationBar(
        selectedIndex: tab,
        onDestinationSelected: (i) => setState(() => tab = i),
        destinations: const [
          NavigationDestination(icon: Icon(Icons.folder), label: 'Files'),
          NavigationDestination(icon: Icon(Icons.search), label: 'Search'),
          NavigationDestination(icon: Icon(Icons.task), label: 'Tasks'),
          NavigationDestination(icon: Icon(Icons.settings), label: 'Settings'),
        ],
      ),
    );
  }

  Widget _filesPage() {
    return Column(children: [
      Padding(
        padding: const EdgeInsets.all(8),
        child: TextField(
          decoration: const InputDecoration(prefixIcon: Icon(Icons.search), hintText: 'Search in folder'),
          onChanged: vm.searchLocal,
        ),
      ),
      Expanded(
        child: ListView.builder(
          itemCount: vm.items.length,
          itemBuilder: (_, i) {
            final e = vm.items[i];
            return ListTile(
              leading: CategoryIcon(category: e.category),
              title: Text(e.name),
              subtitle: Text(e.path),
              trailing: Text(e.isDir ? 'DIR' : '${e.size} B'),
            );
          },
        ),
      ),
    ]);
  }
}
