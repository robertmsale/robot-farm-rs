# robot-farm-rs

Robot Farm is a containerized agent system for building software at light speed 🚀

## Architecture

1. All Codex-CLI instances run inside a docker container that is built on server startup. 
    - It is a concatenation of buildpack-deps with a base environment containing Python 3 and Deno. 
    - You provide a Dockerfile in `<WORKSPACE_DIR>/.robot-farm/Dockerfile` as an intermediate build step to form the final container.
2. Each Codex instance runs in exec mode with `--json` and `--output-schema`
    - This allows intermediate steps to be captured and presented in the Flutter UI
    - The output schema is how the workers and orchestrator communicate. Communication is turn-based, and the system uses strategies to manage worker activity.
3. 