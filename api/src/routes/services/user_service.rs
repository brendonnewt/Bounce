use actix_web::web;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};

use crate::{
    entities,
    utils::{
        api_response::ApiResponse, app_state, jwt::Claims,
        request_models::user_models::UpdateUserModel,
    },
};

pub async fn update_user(
    app_state: web::Data<app_state::AppState>,
    user_data: web::Json<UpdateUserModel>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    let mut user_model = entities::user::Entity::find_by_id(claim_data.user_id)
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(404, "User not found".to_string()))?
        .into_active_model();

    if let Some(name_first) = &user_data.name_first {
        user_model.name_first = Set(name_first.clone());
    }

    if let Some(name_last) = &user_data.name_last {
        user_model.name_last = Set(name_last.clone());
    }

    if let Some(email) = &user_data.email {
        user_model.email = Set(email.clone());
    }

    user_model
        .update(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, "User updated!".to_string()))
}
