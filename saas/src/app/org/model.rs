use floz::prelude::*;

schema! {
    model Organization("organizations") {
        id: integer("id").auto_increment().primary(),
        name: text("name"),
        slug: text("slug"),
        tier: text("tier"),
        created_at: datetime("created_at").tz().now(),
    }
}

pub async fn create_table(db: &Db) {
    if let Err(e) = Organization::create_table(db).await {
        tracing::warn!("Table organizations may already exist: {}", e);
    }
}
