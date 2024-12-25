use actix_web::web;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter, Set,
};

use crate::{
    entities,
    utils::{
        api_response::ApiResponse, app_state, jwt::Claims,
        request_models::user_models::UpdateUserModel,
    },
};

pub async fn update_user(
    app_state: &web::Data<app_state::AppState>,
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

pub async fn get_user(
    app_state: &web::Data<app_state::AppState>,
    claim_data: Claims,
    filters: Option<Condition>,
) -> Result<entities::user::Model, ApiResponse> {
    // Init query
    let mut query = entities::user::Entity::find();

    // Apply filters, or default to finding by id
    if let Some(filter) = filters {
        query = query.filter(filter);
    } else {
        query = query.filter(entities::user::Column::UserId.eq(claim_data.user_id));
    }

    // Find the user
    let user = query
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(404, "User not found".to_string()))?;

    Ok(user)
}

pub async fn get_user_by_id(
    app_state: &web::Data<app_state::AppState>,
    user_id: i32,
) -> Result<entities::user::Model, ApiResponse> {
    let user_model = entities::user::Entity::find_by_id(user_id)
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    match user_model {
        Some(user) => Ok(user),
        None => Err(ApiResponse::new(404, "User not found".to_string())),
    }
}
