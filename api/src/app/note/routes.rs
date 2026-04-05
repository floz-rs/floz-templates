//! Note routes — full CRUD with OpenAPI metadata.
//!
//! All handlers use `#[route]` for auto-discovery (no manual registration).
//! Each demonstrates different floz features:
//!
//! | Route            | Features                                        |
//! |------------------|-------------------------------------------------|
//! | GET /health      | Public endpoint, no auth                        |
//! | GET /notes       | State, DbPool, pagination, JsonResponse         |
//! | GET /notes/{id}  | Path extraction, ApiError, error handling        |
//! | POST /notes      | Json body, JsonResponse::created()               |
//! | PUT /notes/{id}  | Path + Json, ORM update                         |
//! | DELETE /notes/{id}| Path, JsonResponse::no_content()                |

use floz::prelude::*;
use super::model::{Note, CreateNote, UpdateNote};
use crate::AppState;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Public
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Health check — public, no auth needed.
#[route(
    get: "/health",
    tag: "System",
    desc: "Health check",
    resps: [(200, "Service is healthy")],
)]
async fn health(state: State) -> Resp {
    let app = state.ext::<AppState>();
    Resp::Ok().json(&json!({
        "status": "ok",
        "app": app.app_name,
    }))
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Notes CRUD (protected by auth middleware)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// List all notes with pagination.
///
/// Query params: `?limit=10&offset=0&order_by=id`
#[route(
    get: "/notes",
    tag: "Notes",
    desc: "List all notes",
    resps: [(200, "List of notes", Json<Vec<Note>>)],
)]
async fn list_notes(state: State) -> Resp {
    match Note::all(&state.db()).await {
        Ok(notes) => JsonResponse::ok(&notes),
        Err(e) => {
            error!("Failed to list notes: {}", e);
            JsonResponse::error(&e.to_string())
        }
    }
}

/// Get a single note by ID.
#[route(
    get: "/notes/{id}",
    tag: "Notes",
    desc: "Get a note by ID",
    resps: [(200, "Note found", Json<Note>), (404, "Note not found")],
)]
async fn get_note(state: State, id: Path<i32>) -> Resp {
    match Note::find(*id, &state.db()).await {
        Ok(Some(note)) => JsonResponse::ok(&note),
        Ok(None) => JsonResponse::not_found(&format!("Note {} not found", *id)),
        Err(e) => JsonResponse::error(&e.to_string()),
    }
}

/// Create a new note.
#[route(
    post: "/notes",
    tag: "Notes",
    desc: "Create a new note",
    resps: [(201, "Note created", Json<Note>)],
)]
async fn create_note(state: State, body: Json<CreateNote>) -> Resp {
    let input = body.into_inner();
    let note = Note {
        title: input.title,
        content: input.content,
        ..Default::default()
    };

    match note.create(&state.db()).await {
        Ok(created) => JsonResponse::created(&created),
        Err(e) => {
            error!("Failed to create note: {}", e);
            JsonResponse::error(&e.to_string())
        }
    }
}

/// Update an existing note.
#[route(
    put: "/notes/{id}",
    tag: "Notes",
    desc: "Update a note",
    resps: [(200, "Note updated", Json<Note>), (404, "Note not found")],
)]
async fn update_note(state: State, id: Path<i32>, body: Json<UpdateNote>) -> Resp {
    let input = body.into_inner();

    // Find existing note
    let mut existing = match Note::get(*id, &state.db()).await {
        Ok(n) => n,
        Err(_) => return JsonResponse::not_found(&format!("Note {} not found", *id)),
    };

    // Apply partial update
    if let Some(t) = input.title {
        existing.set_title(t);
    }
    if let Some(c) = input.content {
        existing.set_content(c);
    }

    match existing.save(&state.db()).await {
        Ok(_) => JsonResponse::ok(&existing),
        Err(e) => {
            error!("Failed to update note {}: {}", *id, e);
            JsonResponse::error(&e.to_string())
        }
    }
}

/// Delete a note.
#[route(
    delete: "/notes/{id}",
    tag: "Notes",
    desc: "Delete a note",
    resps: [(204, "Note deleted"), (404, "Note not found")],
)]
async fn delete_note(state: State, id: Path<i32>) -> Resp {
    // Verify it exists first
    let to_delete = match Note::get(*id, &state.db()).await {
        Ok(n) => n,
        Err(_) => return JsonResponse::not_found(&format!("Note {} not found", *id)),
    };

    match to_delete.delete(&state.db()).await {
        Ok(_) => JsonResponse::no_content(),
        Err(e) => {
            error!("Failed to delete note {}: {}", *id, e);
            JsonResponse::error(&e.to_string())
        }
    }
}
