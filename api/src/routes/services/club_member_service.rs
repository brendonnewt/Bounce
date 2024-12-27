use actix_web::web;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter, Set,
};

use crate::{
    entities,
    utils::{api_response::ApiResponse, app_state, jwt::Claims},
};

use super::club_service;

pub async fn get_member_by_user_id(
    app_state: &web::Data<app_state::AppState>,
    user_id: i32,
) -> Result<entities::club_member::Model, ApiResponse> {
    // Get membership
    let membership = entities::club_member::Entity::find()
        .filter(Condition::all().add(entities::club_member::Column::UserId.eq(user_id)))
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(404, "No club found for user".to_string()))?;

    Ok(membership)
}

// pub async fn get_members_by_club_id(
//     app_state: &web::Data<app_state::AppState>,
//     club_id: i32,
// ) -> Result<Vec<entities::club_member::Model>, ApiResponse> {
//     // Get membership
//     let memberships = entities::club_member::Entity::find()
//         .filter(Condition::all().add(entities::club_member::Column::ClubId.eq(club_id)))
//         .all(&app_state.db)
//         .await
//         .map_err(|err| ApiResponse::new(500, err.to_string()))?;

//     Ok(memberships)
// }

pub async fn create_membership(
    app_state: &web::Data<app_state::AppState>,
    claim_data: Claims,
    club_id: i32,
) -> Result<entities::club_member::Model, ApiResponse> {
    // Check if the user is a part of another club
    if let Ok(_) = get_member_by_user_id(app_state, claim_data.user_id).await {
        return Err(ApiResponse::new(
            409,
            "User is already a member of a club".to_string(),
        ));
    }

    // Create the membership
    match (entities::club_member::ActiveModel {
        user_id: Set(claim_data.user_id),
        club_id: Set(club_id),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await)
    {
        Ok(member) => Ok(member),
        Err(err) => Err(ApiResponse::new(500, err.to_string())),
    }
}

pub async fn leave_club(
    app_state: &web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    // Ensure user is a member of a club
    let member_result = get_member_by_user_id(app_state, claim_data.user_id).await;

    if member_result.is_err() {
        return Err(member_result.unwrap_err());
    }

    let membership = member_result.unwrap();

    // Check if user is the owner
    let result = club_service::is_owner(&app_state, claim_data.user_id, membership.club_id).await;
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Reject if they are the owner
    if result.unwrap() {
        return Err(ApiResponse::new(
            409,
            "User cannot leave the club if they are the owner".to_string(),
        ));
    }

    let membership = membership.into_active_model();

    // Delete the membership
    let delete_result = membership
        .delete(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    // Evaluate the result
    if delete_result.rows_affected == 1 {
        return Ok(ApiResponse::new(200, "Successfully left club".to_string()));
    } else {
        return Err(ApiResponse::new(500, "Could not leave club".to_string()));
    }
}
