use floz::prelude::*;

#[floz::main]
async fn main() -> std::io::Result<()> {
    App::new()
        .server(
            ServerConfig::new()
                .with_default_port(3030)
                .with_middleware(SecurityHeaders::default())
                .with_middleware(Cors::permissive())
                .with_middleware(RequestTrace::default())
                .with_middleware(SessionMiddleware::new())
        )
        .run()
        .await
}

#[route(
    get: "/health",
    tag: "System",
    desc: "Minimal health check"
)]
async fn health() -> Resp {
    Resp::Ok().json(&json!({"status": "ok"}))
}
