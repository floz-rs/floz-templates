use floz::prelude::*;
use super::model::Organization;

#[derive(Deserialize)]
pub struct CreateOrg {
    pub name: String,
    pub slug: String,
}

#[route(
    post: "/orgs",
    tag: "Organizations",
    resps: [(201, "Organization Created", Json<Organization>)]
)]
pub async fn create_org(state: State, body: Json<CreateOrg>) -> Resp {
    let org = Organization {
        name: body.name.clone(),
        slug: body.slug.clone(),
        tier: "free".to_string(),
        ..Default::default()
    }.create(&state.db()).await;

    match org {
        Ok(o) => JsonResponse::created(&o),
        Err(e) => JsonResponse::error(&e.to_string()),
    }
}

#[route(
    get: "/orgs",
    tag: "Organizations",
)]
pub async fn list_orgs(state: State) -> Resp {
    match Organization::all(&state.db()).await {
        Ok(o) => JsonResponse::ok(&o),
        Err(e) => JsonResponse::error(&e.to_string()),
    }
}
