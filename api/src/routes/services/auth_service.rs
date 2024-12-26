use actix_web::web;
use sha256::digest;

use crate::entities;
use crate::utils::{
    api_response::ApiResponse,
    app_state,
    jwt::encode_jwt,
    request_models::auth_models::{LoginModel, RegisterModel},
};

use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};

pub async fn register(
    app_state: &web::Data<app_state::AppState>,
    json: web::Json<RegisterModel>,
) -> Result<ApiResponse, ApiResponse> {
    // Make sure user type is valid
    if json.user_type != "A" && json.user_type != "C" {
        return Err(ApiResponse::new(
            500,
            "Invalid user type, must be A or C".to_string(),
        ));
    }

    let user = entities::user::Entity::find()
        .filter(
            Condition::all()
                .add(entities::user::Column::Email.eq(&json.email))
                .add(entities::user::Column::UserType.eq(&json.user_type)),
        )
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    // If a user with that email and type already exists, reject it
    if user.is_some() {
        return Err(ApiResponse::new(
            409,
            "User with that email and type already exists".to_string(),
        ));
    }

    // Create the user
    let user_model = entities::user::ActiveModel {
        name_first: Set(json.name_first.clone()),
        name_last: Set(json.name_last.clone()),
        email: Set(json.email.clone()),
        password: Set(digest(&json.password)),
        user_type: Set(json.user_type.clone()),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, format!("{}", user_model.user_id)))
}

pub async fn login_user(
    app_state: &web::Data<app_state::AppState>,
    json: web::Json<LoginModel>,
) -> Result<ApiResponse, ApiResponse> {
    // Get the user
    let user = entities::user::Entity::find()
        .filter(
            Condition::all()
                .add(entities::user::Column::Email.eq(&json.email))
                .add(entities::user::Column::Password.eq(digest(&json.password)))
                .add(entities::user::Column::UserType.eq(&json.user_type)),
        )
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(
            404,
            "No user found for that email and password".to_string(),
        ))?;

    // Create the jwt token
    let token = encode_jwt(user.email, user.user_id)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, format!("{{ 'token': {}}}", token)))
}
