use actix_web::{get, post, web};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};
use serde::{Deserialize, Serialize};

use crate::{
    entities,
    utils::{
        api_response::{self, ApiResponse},
        app_state,
        jwt::Claims,
    },
};

#[derive(Serialize, Deserialize)]
struct UpdateUserModel {
    pub name_first: String,
    pub name_last: String,
}

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

    Ok(api_response::ApiResponse::new(
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
    let mut user_model = entities::user::Entity::find_by_id(claim_data.user_id)
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(404, "User not found".to_string()))?
        .into_active_model();

    user_model.name_first = Set(user_data.name_first.clone());
    user_model.name_last = Set(user_data.name_last.clone());

    user_model
        .update(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, "User updated!".to_string()))
}
