import 'package:api_client/api_client.dart';
import 'package:flutter/material.dart';

class SeriesScreen extends StatefulWidget {
  final HandlersSeriesApi api;

  const SeriesScreen({super.key, required this.api});

  @override
  State<SeriesScreen> createState() => _SeriesScreenState();
}

class _SeriesScreenState extends State<SeriesScreen> {
  late Future<SeriesListResponse> _future;

  @override
  void initState() {
    super.initState();
    _future = widget.api.listSeries().then((r) => r.data!);
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<SeriesListResponse>(
      future: _future,
      builder: (context, snapshot) {
        if (snapshot.connectionState == ConnectionState.waiting) {
          return const Center(child: CircularProgressIndicator());
        }
        if (snapshot.hasError) {
          return Center(child: Text('Error: ${snapshot.error}'));
        }
        final series = snapshot.data!.data;
        return ListView.builder(
          itemCount: series.length,
          itemBuilder: (context, index) {
            final s = series[index];
            return ListTile(
              title: Text(s.name),
              subtitle: s.description.isNotEmpty
                  ? Text(s.description, maxLines: 1, overflow: TextOverflow.ellipsis)
                  : null,
            );
          },
        );
      },
    );
  }
}
