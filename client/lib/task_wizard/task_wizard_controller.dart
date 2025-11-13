import 'package:flutter/material.dart';
import 'package:get/get.dart';

class TaskWizardController extends GetxController {
  final TextEditingController promptController = TextEditingController();
  final RxBool isRunning = false.obs;
  final RxBool hasPrompt = false.obs;
  final RxList<String> feed = <String>[].obs;

  void _handlePromptChanged() {
    hasPrompt.value = promptController.text.trim().isNotEmpty;
  }

  @override
  void onInit() {
    super.onInit();
    promptController.addListener(_handlePromptChanged);
  }

  void sendPrompt() {
    final prompt = promptController.text.trim();
    if (prompt.isEmpty) {
      Get.snackbar('Missing input', 'Enter instructions before sending.');
      return;
    }

    promptController.clear();
    hasPrompt.value = false;
    isRunning.value = true;
    feed.add('You: $prompt');
    feed.add('Wizard: (stubbed response)');
  }

  void cancelRun() {
    if (!isRunning.value) {
      return;
    }
    isRunning.value = false;
    feed.add('Wizard: Run cancelled.');
  }

  @override
  void onClose() {
    promptController.removeListener(_handlePromptChanged);
    promptController.dispose();
    super.onClose();
  }
}
