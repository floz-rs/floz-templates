//! Note model — ORM schema definition.
//!
//! Uses the `schema!` macro to define the Notes table.
//! Auto-generates: `Note::all()`, `Note::find()`, `.create()`,
//! `.update()`, `.delete()`, `Note::create_table()`, `Note::drop_table()`.

use floz::prelude::*;

schema! {
    model Note("notes") {
        id:      integer("id").auto_increment().primary(),
        title:   text("title"),
        content: text("content"),
    }
}

/// Input payload for creating a note.
#[derive(Debug, Deserialize)]
pub struct CreateNote {
    pub title: String,
    pub content: String,
}

/// Input payload for updating a note.
#[derive(Debug, Deserialize)]
pub struct UpdateNote {
    pub title: Option<String>,
    pub content: Option<String>,
}

/// Create the notes table if it doesn't exist.
pub async fn create_table(db: &Db) {
    if let Err(e) = Note::create_table(db).await {
        tracing::warn!("Table may already exist: {}", e);
    }
}
