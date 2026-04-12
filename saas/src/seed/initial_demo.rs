use floz::app::AppContext;

/// Database seeder for InitialDemo
pub async fn run(_ctx: &AppContext) -> Result<(), Box<dyn std::error::Error>> {
    println!("🌱 Seeding InitialDemo...");

    // TODO: Write your seeding logic here.
    // Example:
    // let item = User {
    //     // ... properties
    // };
    // item.insert(&ctx.db()).await?;

    Ok(())
}
