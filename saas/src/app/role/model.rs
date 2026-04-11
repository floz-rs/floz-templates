use floz::prelude::*;

#[model("roles", crud(tag = "Roles", path = "/roles"))]
pub struct Role {
    #[col(key, auto)]
    pub id: i32,
    pub name: Text,
    pub permissions: Option<Jsonb>,
}

pub async fn create_table(db: &Db) {
    if let Err(e) = Role::create_table(db).await {
        tracing::warn!("Table roles may already exist: {}", e);
    }
}
