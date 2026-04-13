use floz::prelude::*;

use starter_saas::{app, middleware};
#[derive(Clone)]
pub struct AppState {
    pub default_tenant_id: String,
}

#[floz::main]
async fn main() -> std::io::Result<()> {
    App::new()
        .with(AppState {
            default_tenant_id: "system".to_string(),
        })
        .with_worker(2)
        .server(
            ServerConfig::new()
                .with_default_port(3030)
                .with_middleware(SecurityHeaders::default())
                .with_middleware(Cors::permissive())
                .with_middleware(RequestTrace::default())
                .with_middleware(SessionMiddleware::new())
                .with_async_middleware(CacheMiddleware)
                .with_async_middleware(AuthMiddleware)
                .with_async_middleware(RateLimitMiddleware::new()), // .with_async_middleware(middleware::tenant::RequireTenant)
        )
        // Demonstrate background recurring tasks alongside the web server (Feature #8)
        .schedule(3600, |_ctx| async move {
            info!("🕰️  Hourly cron task: Re-syncing caches or generating reports...");
        })
        .on_start(|ctx: AppContext| async move {
            info!("🔐 Auth module enabled");
            info!("🏢 Multi-tenant architecture enabled");

            // Setup SQLite DB
            app::role::model::create_table(&ctx.db()).await;
            app::user::model::create_table(&ctx.db()).await;
            app::org::model::create_table(&ctx.db()).await;
            app::user_role::model::create_table(&ctx.db()).await;
        })
        .run()
        .await
}

#[route(
    get: "/health",
    tag: "System",
    desc: "Health check",
    cache(ttl = 30, watch = ["system"]),
)]
async fn health() -> Resp {
    Resp::Ok().json(&json!({"status": "ok"}))
}

#[route(
    get: "/session-demo",
    tag: "System",
    desc: "Demonstrate Redis Session Middleware",
    rate: "10/min", // Demonstrating declarative rate limiting (Feature #5)
)]
async fn session_demo(ctx: Context) -> Result<Resp, floz::errors::ApiError> {
    let mut count: i32 = ctx.session().get("visits").await?.unwrap_or(0);
    count += 1;
    ctx.session().set("visits", &count).await?;

    Ok(Resp::Ok().json(&json!({
        "message": "Session tracking is active!",
        "session_id": ctx.req.session_id,
        "visits": count
    })))
}

/// Feature 17: PG LISTEN/NOTIFY Live Events Demonstration
/// (Only compiles if the postgres feature is active)
#[cfg(feature = "postgres")]
#[route(
    get: "/live-events",
    tag: "System",
    desc: "Demonstrate native DB pub/sub via Postgres Listener",
)]
async fn live_events(ctx: Context) -> Result<Resp, floz::errors::ApiError> {
    use sqlx::postgres::PgListener;

    let mut listener = PgListener::connect_with(&*ctx.app.db_pool)
        .await
        .map_err(|_| floz::errors::ApiError::internal("Failed to connect PgListener"))?;

    listener
        .listen("system_events")
        .await
        .map_err(|_| floz::errors::ApiError::internal("Failed to listen"))?;

    // In a real app, this would bridge to an SSE stream or Ntex websocket.
    // For demonstration, we just show how it blocks/awaits events securely.
    // while let Some(notification) = listener.recv().await.unwrap() { ... }

    Ok(Resp::Ok().json(&json!({
        "status": "Listening for system_events payload on postgres backend",
    })))
}
