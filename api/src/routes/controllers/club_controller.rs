use actix_web::{get, post, web};

use crate::{
    routes::services::{club_member_service, club_service},
    utils::{
        api_response::ApiResponse, app_state, jwt::Claims, request_models::club_models::ClubModel,
    },
};

#[get("")]
pub async fn get_user_club(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    let search_result = club_service::get_user_club(&app_state, claim_data.clone(), None).await;

    if search_result.is_err() {
        return Err(search_result.unwrap_err());
    }
    let club = search_result.unwrap();

    Ok(ApiResponse::new(
        200,
        format!(
            "{{ 'user_id': {}, 'club_id': {}, 'name': {} }}",
            claim_data.user_id, club.club_id, club.name
        ),
    ))
}

#[post("")]
pub async fn create_club(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
    json: web::Json<ClubModel>,
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

#[post("/join")]
pub async fn join_club(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
    json: web::Json<ClubModel>,
) -> Result<ApiResponse, ApiResponse> {
    // Get the club
    let club_result = club_service::get_club_by_name(&app_state, json.name.clone()).await;

    // Error handling and formatting
    if club_result.is_err() {
        return Err(club_result.unwrap_err());
    }
    let club = club_result.unwrap();

    // Create the membership
    let join_result =
        club_member_service::create_membership(&app_state, claim_data, club.club_id).await;

    // Error handling and formatting
    if join_result.is_err() {
        return Err(join_result.unwrap_err());
    }
    let membership = join_result.unwrap();

    return Ok(ApiResponse::new(
        200,
        format!(
            "{{ 'club_member_id': {}, 'user_id': {}, 'club_id': {}",
            membership.club_member_id, membership.user_id, membership.club_id
        ),
    ));
}
