use actix_web::{get, post, put, web};
use sea_orm::EntityTrait;

use crate::{
    entities,
    routes::services::{club_service, user_service},
    utils::{
        api_response::ApiResponse, app_state, jwt::Claims,
        request_models::user_models::UpdateUserModel,
    },
};

#[get("")]
pub async fn user(
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
            "{{ 'nameFirst': {}, 'nameLast': {}, 'email': {} }}",
            user.name_first, user.name_last, user.email
        ),
    ))
}

#[post("update")]
pub async fn update(
    app_state: web::Data<app_state::AppState>,
    user_data: web::Json<UpdateUserModel>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    user_service::update_user(app_state, user_data, claim_data).await
}

#[get("club")]
pub async fn get_user_club(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    club_service::get_user_club(app_state, claim_data).await
}

#[put("club")]
pub async fn update_user_club(
    app_state: web::Data<app_state::AppState>,
    user_data: web::Json<UpdateUserModel>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    user_service::update_user(app_state, user_data, claim_data).await
}
