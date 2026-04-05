//! Application modules.
//!
//! Each domain module (note, user, etc.) has its own `model.rs` and `routes.rs`.
//! Routes annotated with `#[route]` are auto-discovered — no manual registration.

pub mod note;
