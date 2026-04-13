use super::model::User;
use floz::prelude::*;

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
    }
    .create(&state.db())
    .await;

    // Feature #7: Background Workers - Trigger background job
    if let Ok(ref created_user) = user {
        let _ = crate::app::tasks::sync_tenant_metrics
            .dispatch(&*state, created_user.id.to_string())
            .await;
    }

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
    let users = match User::all(&state.db()).await {
        Ok(u) => u,
        Err(_) => {
            return Resp::Unauthorized().json(&serde_json::json!({
                "error": "database_error",
                "message": "Failed to query users"
            }))
        }
    };

    let user_opt = users.into_iter().find(|u| u.email == body.email);

    match user_opt {
        Some(u) => {
            // mock verification
            JsonResponse::ok(&serde_json::json!({
                "token": format!("Bearer {}-mock-token", u.id),
                "user": u
            }))
        }
        _ => Resp::Unauthorized().json(&serde_json::json!({
            "error": "unauthorized",
            "message": "Invalid credentials"
        })),
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Cached routes — declarative caching examples
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// List all users — cached for 5 minutes.
///
/// When any row in the `users` table is inserted, updated, or deleted,
/// the outbox sweeper automatically purges this cache entry.
#[route(
    get: "/users/cached",
    tag: "Users",
    desc: "List all users (cached 5 min)",
    cache(ttl = 300, watch = ["users"]),
)]
pub async fn list_users_cached(state: State) -> Resp {
    match User::all(&state.db()).await {
        Ok(users) => JsonResponse::ok(&users),
        Err(e) => JsonResponse::error(&e.to_string()),
    }
}

/// Get a single user by ID — cached for 10 minutes.
///
/// The `watch` tag `"users:{id}"` means this cache entry is only
/// invalidated when that specific user row changes, not the entire table.
#[route(
    get: "/users/cached/:id",
    tag: "Users",
    desc: "Get user by ID (cached 10 min)",
    cache(ttl = 600, watch = ["users", "users:{id}"]),
)]
pub async fn get_user_cached(state: State, id: Path<i32>) -> Resp {
    match User::find(id.into_inner(), &state.db()).await {
        Ok(Some(user)) => JsonResponse::ok(&user),
        Ok(None) => Resp::NotFound().json(&serde_json::json!({
            "error": "not_found",
            "message": "User not found"
        })),
        Err(e) => JsonResponse::error(&e.to_string()),
    }
}

/// Feature #9: File Uploads
/// Extremely ergonomic native handling of multipart form-data for robust local storage.
#[route(
    post: "/users/:id/avatar",
    tag: "Users",
    desc: "Upload user avatar via multipart form data"
)]
pub async fn upload_avatar(
    id: Path<i32>,
    mut payload: floz::web::upload::multipart::Multipart,
) -> Resp {
    use futures_util::StreamExt;
    let user_id = id.into_inner();

    // Process incoming file chunks
    while let Some(item) = payload.next().await {
        if let Ok(field) = item {
            // Note: In real app, validate field.content_type() and size
            let filename = format!("/tmp/avatar_{}.png", user_id);
            let _ = floz::web::upload::save_field(field, filename).await;
        }
    }

    Resp::Ok().json(&serde_json::json!({
        "status": "success",
        "message": "Avatar uploaded successfully"
    }))
}
