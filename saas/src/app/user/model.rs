use floz::prelude::*;

#[model("users", crud(tag = "Users", path = "/users"))]
pub struct User {
    #[col(key, auto)]
    pub id: i32,
    pub email: Text,
    pub password_hash: Text,

    #[rel(has_many(model = "crate::app::user_role::UserRole", foreign_key = "user_id"))]
    pub user_roles: Vec<crate::app::user_role::UserRole>,

    #[col(now)]
    #[schema(value_type = String, format = DateTime)]
    pub created_at: TimestampTz,
}

pub async fn create_table(db: &Db) {
    if let Err(e) = User::create_table(db).await {
        tracing::warn!("Table users may already exist: {}", e);
    }
}
