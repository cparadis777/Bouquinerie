import 'package:api_client/api_client.dart';
import 'package:flutter/material.dart';

class BooksScreen extends StatefulWidget {
  final HandlersBooksApi api;

  const BooksScreen({super.key, required this.api});

  @override
  State<BooksScreen> createState() => _BooksScreenState();
}

class _BooksScreenState extends State<BooksScreen> {
  late Future<BookListResponse> _future;

  @override
  void initState() {
    super.initState();
    _future = widget.api.listBooks().then((r) => r.data!);
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<BookListResponse>(
      future: _future,
      builder: (context, snapshot) {
        if (snapshot.connectionState == ConnectionState.waiting) {
          return const Center(child: CircularProgressIndicator());
        }
        if (snapshot.hasError) {
          return Center(child: Text('Error: ${snapshot.error}'));
        }
        final books = snapshot.data!.data;
        return ListView.builder(
          itemCount: books.length,
          itemBuilder: (context, index) {
            final book = books[index];
            return ListTile(
              title: Text(book.title),
              subtitle: Text(book.language),
            );
          },
        );
      },
    );
  }
}
