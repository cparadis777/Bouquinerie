import 'package:api_client/api_client.dart';
import 'package:flutter/material.dart';

class AuthorsScreen extends StatefulWidget {
  final HandlersAuthorsApi api;

  const AuthorsScreen({super.key, required this.api});

  @override
  State<AuthorsScreen> createState() => _AuthorsScreenState();
}

class _AuthorsScreenState extends State<AuthorsScreen> {
  late Future<AuthorListResponse> _future;

  @override
  void initState() {
    super.initState();
    _future = widget.api.listAuthors().then((r) => r.data!);
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<AuthorListResponse>(
      future: _future,
      builder: (context, snapshot) {
        if (snapshot.connectionState == ConnectionState.waiting) {
          return const Center(child: CircularProgressIndicator());
        }
        if (snapshot.hasError) {
          return Center(child: Text('Error: ${snapshot.error}'));
        }
        final authors = snapshot.data!.data;
        return ListView.builder(
          itemCount: authors.length,
          itemBuilder: (context, index) {
            final author = authors[index];
            return ListTile(
              title: Text(author.name),
            );
          },
        );
      },
    );
  }
}
