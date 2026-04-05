//! Floz API Starter
//!
//! A full-featured API template demonstrating every floz capability:
//! - Auto-discovered routes via `#[route]`
//! - Middleware pipeline (CORS, tracing, compression, auth)
//! - ORM with `schema!` macro
//! - Shared state via `AppContext` + extensions
//! - Swagger UI at `/docs`
//! - Structured error handling
//!
//! Run: `cargo run`
//! Docs: http://localhost:3030/docs

use floz::prelude::*;

mod app;
mod middleware;

/// Custom application state — available via `state.ext::<AppState>()`
#[derive(Clone)]
pub struct AppState {
    pub app_name: String,
    pub jwt_secret: String,
}

#[floz::main]
async fn main() -> std::io::Result<()> {
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "dev-secret-change-me".to_string());

    App::new()
        // Custom shared state — accessible in handlers and middleware
        .with(AppState {
            app_name: "Floz API".to_string(),
            jwt_secret: jwt_secret.clone(),
        })
        // Server config with middleware pipeline
        .server(
            ServerConfig::new()
                .with_default_port(3030)
                // 1. CORS — handle preflight OPTIONS, allow all origins
                .with_middleware(Cors::permissive())
                // 2. Tracing — structured request/response logging
                .with_middleware(RequestTrace::default())
                // 3. Compression — gzip responses > 1KB
                .with_middleware(Compression::gzip())
                // 4. Auth — bearer token validation (skips /health, /docs)
                .with_middleware(middleware::Auth::new(&jwt_secret)),
        )
        // Run migrations / seed data before accepting requests
        .on_start(|ctx| async move {
            let db = ctx.db();
            // Create table if not exists
            app::note::model::create_table(&db).await;
            info!("📝 Notes table ready");
        })
        .run()
        .await
}
