import 'package:api_client/api_client.dart';
import 'package:flutter/material.dart';

import 'screens/authors_screen.dart';
import 'screens/books_screen.dart';
import 'screens/series_screen.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Bouquinerie',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
      ),
      home: const HomeScreen(),
    );
  }
}

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  int _selectedIndex = 0;

  final _apiClient = ApiClient(basePathOverride: 'http://localhost:3000');

  @override
  Widget build(BuildContext context) {
    final screens = [
      BooksScreen(api: _apiClient.getHandlersBooksApi()),
      AuthorsScreen(api: _apiClient.getHandlersAuthorsApi()),
      SeriesScreen(api: _apiClient.getHandlersSeriesApi()),
    ];

    return Scaffold(
      appBar: AppBar(
        title: const Text('Bouquinerie'),
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
      ),
      body: IndexedStack(
        index: _selectedIndex,
        children: screens,
      ),
      bottomNavigationBar: NavigationBar(
        selectedIndex: _selectedIndex,
        onDestinationSelected: (index) {
          setState(() => _selectedIndex = index);
        },
        destinations: const [
          NavigationDestination(icon: Icon(Icons.book), label: 'Books'),
          NavigationDestination(icon: Icon(Icons.person), label: 'Authors'),
          NavigationDestination(icon: Icon(Icons.collections_bookmark), label: 'Series'),
        ],
      ),
    );
  }
}
