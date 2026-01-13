# Rust Todo API

A simple REST API for managing todo tasks, built with Rocket and CSV file storage.

## Project Structure

```
rust_todo/
├── src/
│   ├── main.rs      # Rocket server and API endpoints
│   └── task.rs      # Task struct and CSV I/O operations
├── tasks.csv        # Task storage (auto-created)
├── run.sh           # Helper script to kill port 8000 and run server
├── curl_examples.sh # Example curl commands
└── Cargo.toml       # Project dependencies
```

## Dependencies

### External Crates
- **rocket** (0.5.1) - Web framework with JSON support
- **csv** (1.4.0) - CSV file reading/writing
- **serde** (1.0.228) - Serialization/deserialization

### Standard Library
- `std::fs::File` - File operations
- `std::io::Read` - Reading file contents

## Running the Server

```bash
cargo run
```

Server runs on `http://localhost:8000`

## API Endpoints

| Method | Endpoint | Description | Status Codes |
|--------|----------|-------------|--------------|
| `GET` | `/tasks` | Fetch all tasks | 200 OK |
| `POST` | `/create-task` | Create a new task | 201 Created, 409 Conflict |
| `PUT` | `/update-task` | Update existing task | 204 No Content, 404 Not Found |
| `DELETE` | `/delete-task` | Delete a task | 204 No Content, 404 Not Found |

## Task Schema

```json
{
  "task_name": "string",
  "task_description": "string",
  "task_complete": "string"  // "true" or "false"
}
```

## Example Requests

```bash
# Get all tasks
curl http://localhost:8000/tasks

# Create a task
curl -X POST http://localhost:8000/create-task \
  -H "Content-Type: application/json" \
  -d '{"task_name":"My Task","task_description":"Description","task_complete":"false"}'

# Update a task
curl -X PUT http://localhost:8000/update-task \
  -H "Content-Type: application/json" \
  -d '{"task_name":"My Task","task_description":"Updated","task_complete":"true"}'

# Delete a task
curl -X DELETE http://localhost:8000/delete-task \
  -H "Content-Type: application/json" \
  -d '{"task_name":"My Task","task_description":"","task_complete":""}'
```

