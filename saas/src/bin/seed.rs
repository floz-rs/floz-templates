use floz::app::AppContext;

#[path = "../seed/mod.rs"]
mod seed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    
    // Initialize standard logging
    floz::logger::init_tracing();
    
    // Connect to the database and initialize AppContext
    let ctx = AppContext::init(std::collections::HashMap::new()).await;
    
    // Run the seed sequence
    seed::run_all(&ctx).await?;
    
    Ok(())
}
