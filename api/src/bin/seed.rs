use floz::app::AppContext;

#[path = "../seed/mod.rs"]
mod seed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    floz::logger::init_tracing();
    
    let ctx = AppContext::init(std::collections::HashMap::new()).await;
    seed::run_all(&ctx).await?;
    
    Ok(())
}
