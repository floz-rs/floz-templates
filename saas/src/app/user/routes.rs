use floz::prelude::*;
use super::model::User;

#[derive(Deserialize)]
pub struct AuthInput {
    pub email: String,
    pub password: String,
}

#[route(
    post: "/users/register",
    tag: "Auth",
    resps: [(201, "Registered")]
)]
pub async fn register(state: State, body: Json<AuthInput>) -> Resp {
    let input = body.into_inner();
    let hashed = input.password.chars().rev().collect::<String>(); // mock hash
    
    let user = User {
        email: input.email,
        password_hash: hashed,
        ..Default::default()
    }.create(&state.db()).await;

    match user {
        Ok(u) => JsonResponse::created(&u),
        Err(e) => JsonResponse::error(&e.to_string()),
    }
}

#[route(
    post: "/users/login",
    tag: "Auth",
    resps: [(200, "Logged In")]
)]
pub async fn login(state: State, body: Json<AuthInput>) -> Resp {
    // Fetch all users and filter (in a real app, use a targeted query or custom ORM finder method)
    let users = match User::all(&state.db()).await {
        Ok(u) => u,
        Err(_) => return Resp::Unauthorized().json(&serde_json::json!({
            "error": "database_error",
            "message": "Failed to query users"
        })),
    };

    let user_opt = users.into_iter().find(|u| u.email == body.email);

    match user_opt {
        Some(u) => {
            // mock verification
            JsonResponse::ok(&serde_json::json!({
                "token": format!("Bearer {}-mock-token", u.id),
                "user": u
            }))
        },
        _ => Resp::Unauthorized().json(&serde_json::json!({
            "error": "unauthorized",
            "message": "Invalid credentials"
        })),
    }
}
