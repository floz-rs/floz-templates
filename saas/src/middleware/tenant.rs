use floz::prelude::*;

#[derive(Clone)]
pub struct RequireTenant;

impl RequireTenant {
    fn is_public(path: &str) -> bool {
        matches!(path, "/health" | "/ui" | "/users/register" | "/users/login")
            || path.starts_with("/api-docs")
    }
}

impl AsyncMiddleware for RequireTenant {
    async fn handle(&self, req: &Req) -> Option<Resp> {
        if Self::is_public(req.path()) {
            return None;
        }

        // In a real SaaS, this might be resolved from the subdomain (tenant.myapp.com)
        // or a specific HTTP header. This is async-ready: you can `.await` a DB query here.
        let tenant_id = req
            .headers()
            .get("X-Tenant-Id")
            .and_then(|v| v.to_str().ok());

        if tenant_id.is_none() {
            return Some(Resp::BadRequest().json(&serde_json::json!({
                "error": "missing_tenant",
                "message": "X-Tenant-Id header is required for this operation"
            })));
        }

        None
    }

    fn name(&self) -> &str {
        "require_tenant"
    }
}
