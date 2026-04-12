use floz::prelude::*;

#[derive(Clone)]
pub struct RequireAuth;

impl RequireAuth {
    fn is_public(path: &str) -> bool {
        matches!(path, "/health" | "/ui" | "/users/register" | "/users/login")
            || path.starts_with("/api-docs")
    }
}

impl AsyncMiddleware for RequireAuth {
    async fn handle(&self, req: &Req) -> Option<Resp> {
        if Self::is_public(req.path()) {
            return None;
        }

        let header = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok());

        match header {
            Some(t) if t.starts_with("Bearer ") => {
                // In a real SaaS, decode the JWT here and validate against the DB.
                // This is async-ready: you can `.await` a DB query here.
                None 
            },
            _ => Some(
                Resp::Unauthorized().json(&serde_json::json!({
                    "error": "unauthorized",
                    "message": "Authorization Bearer token is required"
                })),
            ),
        }
    }

    fn name(&self) -> &str {
        "require_auth"
    }
}
