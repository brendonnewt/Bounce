use actix_web::{post, web};
use serde::{Deserialize, Serialize};

use crate::{
    routes::services::auth_service,
    utils::{api_response::ApiResponse, app_state},
};

#[derive(Serialize, Deserialize)]
pub struct RegisterModel {
    pub user_type: String,
    pub name_first: String,
    pub name_last: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginModel {
    pub user_type: String,
    pub email: String,
    pub password: String,
}

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
