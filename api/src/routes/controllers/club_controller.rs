use actix_web::{delete, get, post, put, web};

use crate::{
    routes::services::{club_member_service, club_service},
    utils::{
        api_response::ApiResponse,
        app_state,
        jwt::Claims,
        request_models::club_models::{ClubModel, TransferOwnerModel},
    },
};

#[get("/{club_id}")]
pub async fn get_club(
    app_state: web::Data<app_state::AppState>,
    path: web::Path<i32>,
) -> Result<ApiResponse, ApiResponse> {
    let club_id = path.into_inner();
    let club = club_service::get_club_by_id(&app_state, club_id).await;

    if club.is_err() {
        return Err(club.unwrap_err());
    }
    let club = club.unwrap();

    Ok(ApiResponse::new(
        200,
        format!(
            "{{ 'club_id': {}, 'name': {}, 'owner_id': {} }}",
            club.club_id, club.name, club.owner_id
        ),
    ))
}

#[post("/create")]
pub async fn create_club(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
    json: web::Json<ClubModel>,
) -> Result<ApiResponse, ApiResponse> {
    club_service::create_club(&app_state, claim_data, json.name.clone()).await
}

#[post("/{club_id}/leave")]
pub async fn leave_club(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    club_member_service::leave_club(&app_state, claim_data).await
}

#[post("/{club_id}/join")]
pub async fn join_club(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
    path: web::Path<i32>,
) -> Result<ApiResponse, ApiResponse> {
    let club_id = path.into_inner();
    // Get the club
    let club_result = club_service::get_club_by_id(&app_state, club_id).await;

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

#[delete("/delete")]
pub async fn delete_club(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    club_service::delete_club(&app_state, claim_data).await
}

#[put("/transfer")]
pub async fn transfer_ownership(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
    json: web::Json<TransferOwnerModel>,
) -> Result<ApiResponse, ApiResponse> {
    let new_owner_id = json.new_owner_id;
    club_service::transfer_ownership(&app_state, claim_data, new_owner_id).await
}

// TODO: Get a list of all athletes in the users club
// #[get("athletes")]
// pub async fn get_club_athletes(
//     app_state: web::Data<app_state::AppState>,
//     claim_data: Claims,
// ) -> Result<ApiResponse, ApiResponse> {
//     let new_owner_id = json.new_owner_id;
//     club_service::get_club_athletes(&app_state, claim_data).await
// }

// TODO: Get a list of all members in the users club
// #[get("club-members")]
// pub async fn get_club_members(
//     app_state: web::Data<app_state::AppState>,
//     claim_data: Claims,
// ) -> Result<ApiResponse, ApiResponse> {
//     club_service::get_club_members(&app_state, claim_data).await
// }
