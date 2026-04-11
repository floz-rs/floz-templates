use ntex::web;

async fn handler1() -> String { "get".to_string() }
async fn handler2() -> String { "post".to_string() }

fn run1(cfg: &mut web::ServiceConfig) {
    cfg.route("/test", web::get().to(handler1));
}

fn run2(cfg: &mut web::ServiceConfig) {
    cfg.route("/test", web::post().to(handler2));
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    // Start server in background
    let srv = web::HttpServer::new(|| {
        web::App::new()
            .configure(run1)
            .configure(run2)
    })
    .bind("127.0.0.1:8081")?
    .run();
    
    // Spawn task
    ntex::rt::spawn(async move { srv.await });
    
    actix_rt::time::sleep(std::time::Duration::from_millis(500)).await;
    
    // Send post request
    let resp = reqwest::Client::new()
        .post("http://127.0.0.1:8081/test")
        .send()
        .await
        .unwrap();
    println!("Status: {}", resp.status());
    Ok(())
}
