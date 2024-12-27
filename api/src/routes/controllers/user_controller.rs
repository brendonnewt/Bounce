use actix_web::{get, post, web};
use sea_orm::EntityTrait;

use crate::{
    entities,
    routes::services::{club_member_service, club_service, user_service},
    utils::{
        api_response::ApiResponse,
        app_state,
        jwt::Claims,
        request_models::user_models::{UpdatePasswordModel, UpdateUserModel},
    },
};

#[get("/{user_id}")]
pub async fn get_user(
    app_state: web::Data<app_state::AppState>,
    path: web::Path<i32>,
) -> Result<ApiResponse, ApiResponse> {
    let user_id = path.into_inner();
    let user = entities::user::Entity::find_by_id(user_id)
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(404, err.to_string()))?
        .ok_or(ApiResponse::new(404, "User not found".to_string()))?;

    Ok(ApiResponse::new(
        200,
        format!(
            "{{ 'user_id': {}, 'user_type': {}, 'name_first': {}, 'name_last': {}, 'email': {} }}",
            user.user_id, user.user_type, user.name_first, user.name_last, user.email
        ),
    ))
}

#[get("/{user_id}/club")]
pub async fn get_user_club(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
    path: web::Path<i32>,
) -> Result<ApiResponse, ApiResponse> {
    let user_id = path.into_inner();
    // Get the membership
    let membership = club_member_service::get_member_by_user_id(&app_state, user_id).await;

    if membership.is_err() {
        return Err(membership.unwrap_err());
    }
    let membership = membership.unwrap();

    // Get the club the user is a part of
    let search_result = club_service::get_club_by_id(&app_state, membership.club_id).await;

    if search_result.is_err() {
        return Err(search_result.unwrap_err());
    }
    let club = search_result.unwrap();

    Ok(ApiResponse::new(
        200,
        format!(
            "{{ 'user_id': {}, 'club_id': {}, 'name': {} }}",
            claim_data.user_id, club.club_id, club.name
        ),
    ))
}

#[post("/reset-password")]
pub async fn reset_password(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
    json: web::Json<UpdatePasswordModel>,
) -> Result<ApiResponse, ApiResponse> {
    let old_pass = json.old_password.clone();
    let new_pass = json.new_password.clone();
    user_service::reset_password(&app_state, claim_data, old_pass, new_pass).await
}

#[post("/update")]
pub async fn update(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
    user_data: web::Json<UpdateUserModel>,
) -> Result<ApiResponse, ApiResponse> {
    user_service::update_user(&app_state, user_data, claim_data).await
}
