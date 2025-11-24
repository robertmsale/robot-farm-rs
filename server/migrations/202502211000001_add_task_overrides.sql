-- Add model and reasoning overrides to tasks
ALTER TABLE task ADD COLUMN model_override TEXT NULL;
ALTER TABLE task ADD COLUMN reasoning_override TEXT NULL;
