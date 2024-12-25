use actix_web::web;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter, Set,
};

use crate::{
    entities,
    utils::{api_response::ApiResponse, app_state, jwt::Claims},
};

use super::club_service::get_user_club;

pub async fn get_club_member(
    app_state: &web::Data<app_state::AppState>,
    claim_data: Claims,
    filters: Option<Condition>,
) -> Result<entities::club_member::Model, ApiResponse> {
    let mut query = entities::club_member::Entity::find();

    if let Some(filter) = filters {
        query = query.filter(filter);
    } else {
        query = query.filter(entities::club_member::Column::UserId.eq(claim_data.user_id));
    }

    let result = query
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(
            404,
            "No club membership found".to_string(),
        ))?;

    return Ok(result);
}

pub async fn create_membership(
    app_state: &web::Data<app_state::AppState>,
    claim_data: Claims,
    club_id: i32,
) -> Result<entities::club_member::Model, ApiResponse> {
    // Check if the user is a part of another club
    if let Ok(_) = get_club_member(app_state, claim_data.clone(), None).await {
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
    let member_result = get_club_member(app_state, claim_data.clone(), None).await;

    if member_result.is_err() {
        return Err(member_result.unwrap_err());
    }

    let membership = member_result.unwrap();

    // See if the user is the owner of the club
    let filters = Condition::all()
        .add(entities::club_member::Column::ClubId.eq(membership.club_id))
        .add(entities::club_member::Column::UserId.eq(membership.user_id));

    let search_result = get_user_club(app_state, claim_data.clone(), Some(filters)).await;

    // Error handling/formatting
    if search_result.is_err() {
        return Err(search_result.unwrap_err());
    }
    if search_result.unwrap().owner_id == claim_data.user_id {
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
