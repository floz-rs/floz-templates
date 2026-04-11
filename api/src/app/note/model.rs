//! Note model — ORM schema definition.
//!
//! Uses the `#[model]` attribute macro to define the Notes table.
//! Auto-generates: `Note::all()`, `Note::find()`, `.create()`,
//! `.update()`, `.delete()`, `Note::create_table()`, `Note::drop_table()`.

use floz::prelude::*;

#[model("notes")]
pub struct Note {
    #[col(key, auto)]
    pub id: i32,
    pub title: Varchar,
    pub content: Text,
}

/// Create the notes table if it doesn't exist.
pub async fn create_table(db: &Db) {
    if let Err(e) = Note::create_table(db).await {
        tracing::warn!("Table may already exist: {}", e);
    }
}
