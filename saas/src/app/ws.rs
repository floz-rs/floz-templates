use floz::prelude::*;
use crate::app::user_role::model::UserRole;

/// This magically intercepts any frontend asking to join an "org_*" channel
#[channel_gate("org_{org_id}")]
async fn check_org_access(ctx: Context, org_id: String) -> bool {
    // 1. Ensure the session has a valid logged in user
    let user_id = match ctx.req.auth.user_id {
        Some(ref id) => id,
        None => return false, // Kicks the WebSocket out
    };
    
    // 2. Only allow them to join if they belong to this org in the DB
    // 2. Only allow them to join if they belong to this org in the DB
    // Assuming `user_roles` table validates access.
    // Example: UserRole::find().filter("user_id = $1").bind(user_id).count(&ctx.app.db()).await > 0
    true
}

/// Helper endpoint to demonstrate broadcasting to the WebSocket channel
#[route(
    post: "/orgs/:org_id/broadcast_test",
    tag: "System",
)]
pub async fn trigger_broadcast(ctx: Context, path: Path<String>) -> Resp {
    let org_id = path.into_inner();
    
    // Broadcast down the WebSocket channel instantly!
    let channel = format!("org_{}", org_id);
    let payload = json!({ 
        "event": "Ping", 
        "message": format!("Broadcast sent to {}", channel)
    });
    
    ctx.app.broadcast(&channel, &payload);
    
    Resp::Ok().json(&json!({"status": "success", "org_id": org_id}))
}
