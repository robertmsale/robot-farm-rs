import 'package:my_api_client/api.dart' as robot_farm_api;

class TaskGroupEditPayload {
  const TaskGroupEditPayload({
    required this.slug,
    required this.title,
    required this.description,
  });

  final String slug;
  final String title;
  final String description;
}

class TaskEditPayload {
  const TaskEditPayload({
    required this.slug,
    required this.title,
    this.commitHash,
    required this.status,
    required this.owner,
    required this.description,
  });

  final String slug;
  final String title;
  final String? commitHash;
  final robot_farm_api.TaskStatus status;
  final String owner;
  final String description;

  String toStringForLog() {
    return '{slug: $slug, title: $title, commit: ${commitHash ?? '—'}, '
        'status: ${status.value}, owner: $owner, '
        'descriptionLength: ${description.length}}';
  }
}
