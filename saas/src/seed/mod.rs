pub mod initial_demo;

pub async fn run_all(ctx: &floz::app::AppContext) -> Result<(), Box<dyn std::error::Error>> {
    println!("🌱 Running database seeds...");

    // Register your seeds here in the correct execution order.
    initial_demo::run(ctx).await?;

    println!("✅ All seeds completed!");
    Ok(())
}
