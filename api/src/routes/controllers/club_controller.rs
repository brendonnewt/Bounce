use actix_web::{get, post, web};

use crate::{
    routes::services::{club_member_service, club_service},
    utils::{
        api_response::ApiResponse, app_state, jwt::Claims,
        request_models::club_models::CreateClubModel,
    },
};

#[get("")]
pub async fn get_user_club(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    club_service::get_club(&app_state, claim_data, None).await
}

#[post("")]
pub async fn create_club(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
    json: web::Json<CreateClubModel>,
) -> Result<ApiResponse, ApiResponse> {
    club_service::create_club(&app_state, claim_data, json).await
}

#[post("/leave")]
pub async fn leave_club(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    club_member_service::leave_club(&app_state, claim_data).await
}

// TODO

// #[post("/join")]
// pub async fn join_club(
//     app_state: web::Data<app_state::AppState>,
//     user_data: web::Json<UpdateUserModel>,
//     claim_data: Claims,
// ) -> Result<ApiResponse, ApiResponse> {
//     user_service::update_user(&app_state, user_data, claim_data).await
// }
