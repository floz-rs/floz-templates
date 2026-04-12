# Floz API Starter

A full-featured REST API template built with [floz](https://github.com/user/floz).

## Features

- ✅ **Auto-discovered routes** — `#[route]` macro, zero manual registration
- ✅ **Middleware pipeline** — CORS, tracing, gzip compression, bearer auth
- ✅ **ORM** — `schema!` macro with auto-CRUD (create, find, all, update, delete)
- ✅ **Swagger UI** — auto-generated at `/ui`
- ✅ **Shared state** — custom `AppState` via `state.ext::<T>()`
- ✅ **Structured errors** — `ApiError` + `ErrorCode` with JSON responses
- ✅ **Pagination** — `PaginationParams` query extractor
- ✅ **Startup hooks** — `on_start` for migrations / seeding

## Quick Start

```bash
# 1. Copy environment config
cp .env.example .env

# 2. Start PostgreSQL (update DATABASE_URL in .env)

# 3. Run
cargo run
```

Server starts at `http://localhost:3030`.

## Endpoints

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/health` | ❌ | Health check |
| GET | `/ui` | ❌ | Swagger UI |
| GET | `/notes` | ✅ | List all notes |
| GET | `/notes/{id}` | ✅ | Get note by ID |
| POST | `/notes` | ✅ | Create a note |
| PUT | `/notes/{id}` | ✅ | Update a note |
| DELETE | `/notes/{id}` | ✅ | Delete a note |

## Testing with curl

```bash
# Health (public)
curl http://localhost:3030/health

# Create a note (auth required)
curl -X POST http://localhost:3030/notes \
  -H "Authorization: Bearer dev-secret-change-me" \
  -H "Content-Type: application/json" \
  -d '{"title": "Hello", "content": "World"}'

# List notes
curl http://localhost:3030/notes \
  -H "Authorization: Bearer dev-secret-change-me"

# Get note by ID
curl http://localhost:3030/notes/1 \
  -H "Authorization: Bearer dev-secret-change-me"

# Update a note
curl -X PUT http://localhost:3030/notes/1 \
  -H "Authorization: Bearer dev-secret-change-me" \
  -H "Content-Type: application/json" \
  -d '{"title": "Updated Title"}'

# Delete a note
curl -X DELETE http://localhost:3030/notes/1 \
  -H "Authorization: Bearer dev-secret-change-me"

# Without auth (rejected)
curl http://localhost:3030/notes
# → {"error":"missing_token","message":"Authorization header is required"}
```

## Project Structure

```
src/
├── main.rs              # Boot: ServerConfig, middleware, state, on_start
├── middleware/
│   └── mod.rs           # Custom auth middleware (Bearer token)
└── app/
    ├── mod.rs           # Module barrel
    └── note/
        ├── mod.rs       # Note module barrel
        ├── model.rs     # schema! + ORM model + DTOs
        └── routes.rs    # CRUD handlers with #[route]
```

## Adding a New Resource

1. Create `src/app/todo/mod.rs`, `model.rs`, `routes.rs`
2. Add `pub mod todo;` to `src/app/mod.rs`
3. Define your model with `schema!`
4. Add route handlers with `#[route]`
5. That's it — routes are auto-discovered, no wiring needed
