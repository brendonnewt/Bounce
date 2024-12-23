use actix_web::{post, web};

use crate::{
    routes::services::auth_service,
    utils::{
        api_response::ApiResponse,
        app_state,
        request_models::auth_models::{LoginModel, RegisterModel},
    },
};

#[post("register")]
pub async fn register_athlete(
    app_state: web::Data<app_state::AppState>,
    json: web::Json<RegisterModel>,
) -> Result<ApiResponse, ApiResponse> {
    auth_service::register(app_state, json).await
}

#[post("login")]
pub async fn login(
    app_state: web::Data<app_state::AppState>,
    json: web::Json<LoginModel>,
) -> Result<ApiResponse, ApiResponse> {
    auth_service::login_user(app_state, json).await
}
