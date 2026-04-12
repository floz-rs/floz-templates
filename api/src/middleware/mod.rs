//! Custom middleware for the API starter.

use floz::prelude::*;

/// Bearer token authentication middleware.
///
/// Validates `Authorization: Bearer <token>` on protected routes.
/// Skips authentication for public paths: `/health`, `/ui`, `/api-docs`.
///
/// # Usage
/// ```ignore
/// ServerConfig::new()
///     .with_middleware(Auth::new("my-secret"))
/// ```
///
/// # How it accesses state
/// The middleware can also access `AppContext` via `req.app_state()`:
/// ```ignore
/// let ctx = req.app_state::<AppContext>().unwrap();
/// let db = ctx.db();  // access the database pool
/// let config = ctx.ext::<MyConfig>(); // access custom state
/// ```
#[derive(Clone)]
pub struct Auth {
    token: String,
}

impl Auth {
    pub fn new(secret: &str) -> Self {
        Self {
            token: format!("Bearer {}", secret),
        }
    }

    /// Check if a path is public (no auth required).
    fn is_public(path: &str) -> bool {
        matches!(path, "/health" | "/ui")
            || path.starts_with("/api-docs")
    }
}

impl Middleware for Auth {
    fn handle(&self, req: &Req) -> Option<Resp> {
        // Skip auth for public routes
        if Self::is_public(req.path()) {
            return None;
        }

        // Validate bearer token
        let header = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok());

        match header {
            Some(t) if t == self.token => None, // valid — continue
            Some(_) => Some(
                Resp::Unauthorized().json(&serde_json::json!({
                    "error": "invalid_token",
                    "message": "The provided authorization token is invalid"
                })),
            ),
            None => Some(
                Resp::Unauthorized().json(&serde_json::json!({
                    "error": "missing_token",
                    "message": "Authorization header is required"
                })),
            ),
        }
    }

    fn name(&self) -> &str {
        "auth"
    }
}
