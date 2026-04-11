use floz::prelude::*;

#[model("user_roles", crud(tag = "User Roles", path = "/user-roles"))]
pub struct UserRole {
    #[col(key, auto)]
    pub id: i32,
    pub user_id: i32,
    pub org_id: i32,
    pub role_id: i32,
}

pub async fn create_table(db: &Db) {
    if let Err(e) = UserRole::create_table(db).await {
        tracing::warn!("Table user_roles may already exist: {}", e);
    }
}
