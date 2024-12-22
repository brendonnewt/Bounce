use actix_web::web;
use sha256::digest;

use crate::entities;
use crate::routes::controllers::auth_controller::{LoginModel, RegisterModel};
use crate::utils::jwt::encode_jwt;
use crate::utils::{api_response::ApiResponse, app_state};

use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};

pub async fn register_athlete(
    app_state: web::Data<app_state::AppState>,
    json: web::Json<RegisterModel>,
) -> Result<ApiResponse, ApiResponse> {
    let user_model = entities::user::ActiveModel {
        name_first: Set(json.name_first.clone()),
        name_last: Set(json.name_last.clone()),
        email: Set(json.email.clone()),
        password: Set(digest(&json.password)),
        user_type: Set("A".to_string()),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, format!("{}", user_model.user_id)))
}

pub async fn login_user(
    app_state: web::Data<app_state::AppState>,
    json: web::Json<LoginModel>,
) -> Result<ApiResponse, ApiResponse> {
    // Get the user
    let user = entities::user::Entity::find()
        .filter(
            Condition::all()
                .add(entities::user::Column::Email.eq(&json.email))
                .add(entities::user::Column::Password.eq(digest(&json.password))),
        )
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(404, "User not found".to_string()))?;

    // Create the jwt token
    let token = encode_jwt(user.email, user.user_id)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, format!("{{ 'token': {}}}", token)))
}
