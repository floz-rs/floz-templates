use floz::prelude::*;

schema! {
    model User("users") {
        id: integer("id").auto_increment().primary(),
        org_id: integer("org_id").nullable(),
        email: text("email"),
        password_hash: text("password_hash"),
        created_at: datetime("created_at").tz().now(),
    }
}

pub async fn create_table(db: &Db) {
    if let Err(e) = User::create_table(db).await {
        tracing::warn!("Table users may already exist: {}", e);
    }
}
