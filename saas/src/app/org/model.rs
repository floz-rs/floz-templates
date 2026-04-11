use floz::prelude::*;

#[model("organizations", crud(tag = "Organizations", path = "/orgs"))]
pub struct Organization {
    #[col(key, auto)]
    pub id: i32,
    pub name: Text,
    pub slug: Text,
    pub tier: Text,

    #[col(now)]
    #[schema(value_type = String, format = DateTime)]
    pub created_at: TimestampTz,
}

pub async fn create_table(db: &Db) {
    if let Err(e) = Organization::create_table(db).await {
        tracing::warn!("Table organizations may already exist: {}", e);
    }
}
