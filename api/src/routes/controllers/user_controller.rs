use actix_web::{get, post, web};
use sea_orm::EntityTrait;

use crate::{
    entities,
    routes::services::user_service,
    utils::{
        api_response::ApiResponse,
        app_state,
        jwt::Claims,
        request_models::user_models::{UpdatePasswordModel, UpdateUserModel},
    },
};

#[get("")]
pub async fn get_user(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    let user = entities::user::Entity::find_by_id(claim_data.user_id)
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

#[post("reset-password")]
pub async fn reset_password(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
    json: web::Json<UpdatePasswordModel>,
) -> Result<ApiResponse, ApiResponse> {
    let old_pass = json.old_password.clone();
    let new_pass = json.new_password.clone();
    user_service::reset_password(&app_state, claim_data, old_pass, new_pass).await
}

#[post("update")]
pub async fn update(
    app_state: web::Data<app_state::AppState>,
    user_data: web::Json<UpdateUserModel>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    user_service::update_user(&app_state, user_data, claim_data).await
}
