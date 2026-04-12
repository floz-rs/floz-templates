use floz::prelude::*;

mod app;
mod middleware;
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
                .with_middleware(Cors::permissive())
                .with_middleware(RequestTrace::default())
                .with_middleware(Compression::gzip())
                .with_async_middleware(CacheMiddleware)
                // .with_async_middleware(middleware::auth::RequireAuth)
                // .with_async_middleware(middleware::tenant::RequireTenant)
        )
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
