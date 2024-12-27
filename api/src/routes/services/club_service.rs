use actix_web::web;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter, Set,
};

use crate::{
    entities,
    routes::services::{club_member_service, user_service},
    utils::{api_response::ApiResponse, app_state, jwt::Claims},
};

use super::{club_member_service::get_member_by_user_id, user_service::get_user_by_id};

// pub async fn get_club_by_name(
//     app_state: &web::Data<app_state::AppState>,
//     name: String,
// ) -> Result<entities::club::Model, ApiResponse> {
//     // Search for clubs matching the input name
//     let query = entities::club::Entity::find()
//         .filter(Condition::all().add(entities::club::Column::Name.eq(name)));

//     // Get the club
//     let club = query
//         .one(&app_state.db)
//         .await
//         .map_err(|err| ApiResponse::new(500, err.to_string()))?
//         .ok_or(ApiResponse::new(
//             404,
//             "No club found with that name".to_string(),
//         ))?;

//     Ok(club)
// }

pub async fn get_club_by_id(
    app_state: &web::Data<app_state::AppState>,
    club_id: i32,
) -> Result<entities::club::Model, ApiResponse> {
    // Search for clubs matching the input name
    let query = entities::club::Entity::find()
        .filter(Condition::all().add(entities::club::Column::ClubId.eq(club_id)));

    // Get the club
    let club = query
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(
            404,
            "No club found with that club_id".to_string(),
        ))?;

    Ok(club)
}

pub async fn create_club(
    app_state: &web::Data<app_state::AppState>,
    claim_data: Claims,
    club_name: String,
) -> Result<ApiResponse, ApiResponse> {
    // Ensure user trying to make club is a coach
    let filters = Some(
        Condition::all()
            .add(entities::user::Column::UserId.eq(claim_data.user_id))
            .add(entities::user::Column::UserType.eq("C".to_string())),
    );

    // Search for a coach result with the current user_id
    let coach_result = user_service::get_user(&app_state, claim_data.clone(), filters).await;

    // Error handling/formatting result
    if coach_result.is_err() {
        if coach_result.as_ref().unwrap_err().status_code == 404 {
            return Err(ApiResponse::new(
                404,
                "Coach account not found for that email".to_string(),
            ));
        } else {
            return Err(coach_result.unwrap_err());
        }
    }
    let coach = coach_result.unwrap();

    // Check if the coach is already a member of a club
    if let Ok(_) = club_member_service::get_member_by_user_id(&app_state, claim_data.user_id).await
    {
        return Err(ApiResponse::new(
            409,
            "Users cannot be part of two clubs at once".to_string(),
        ));
    }

    // Check if the club already exists
    if let Some(_) = entities::club::Entity::find()
        .filter(entities::club::Column::Name.eq(club_name.clone().to_lowercase()))
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
    {
        return Err(ApiResponse::new(
            409,
            "A club with that name already exists. Please try a different name".to_string(),
        ));
    }

    // Create and insert the club into the database
    let club_model = entities::club::ActiveModel {
        name: Set(club_name.clone().to_lowercase()),
        owner_id: Set(coach.user_id),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    // Create the membership
    let membership_result =
        club_member_service::create_membership(&app_state, claim_data.clone(), club_model.club_id)
            .await;

    // Return the created membership, or the error if something went wrong
    if membership_result.is_ok() {
        return Ok(ApiResponse::new(
            200,
            format!(
                "{{ 'member_id': {}, 'club_id': {}, 'name': {} }}",
                membership_result.unwrap().club_member_id,
                club_model.club_id,
                club_model.name
            ),
        ));
    } else {
        return Err(membership_result.unwrap_err());
    }
}

pub async fn delete_club(
    app_state: &web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    // Check if user deleting is the club owner
    let club = get_club_if_owner(&app_state, claim_data.user_id).await;

    if club.is_err() {
        return Err(club.unwrap_err());
    }
    let club = club.unwrap();

    // Delete the club
    let deleted_rows = club
        .into_active_model()
        .delete(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    // Validate deletion
    if deleted_rows.rows_affected == 1 {
        return Ok(ApiResponse::new(
            200,
            "Club deleted successfully".to_string(),
        ));
    } else {
        return Err(ApiResponse::new(
            500,
            "Internal server error: Club could not be deleted".to_string(),
        ));
    }
}

pub async fn transfer_ownership(
    app_state: &web::Data<app_state::AppState>,
    claim_data: Claims,
    new_owner_id: i32,
) -> Result<ApiResponse, ApiResponse> {
    // Get new owners user information
    let user_result = get_user_by_id(&app_state, new_owner_id).await;

    // Check that the new owner is a coach
    if user_result.is_err() {
        return Err(user_result.unwrap_err());
    } else if user_result.unwrap().user_type != "C" {
        return Err(ApiResponse::new(
            422,
            "New owner must be a coach".to_string(),
        ));
    }

    // Check that user owns the club
    let club = get_club_if_owner(&app_state, claim_data.user_id).await;

    // Error handling/formatting
    if club.is_err() {
        return Err(club.unwrap_err());
    }
    let club = club.unwrap();

    // Ensure the new owner is a coach in the club
    let new_owner_membership = get_member_by_user_id(&app_state, new_owner_id).await;

    // Handle if they are not a member of any club
    if new_owner_membership.is_err() {
        return Err(new_owner_membership.unwrap_err());
    }

    // Handle if they are not a member of the club
    if new_owner_membership.unwrap().club_id != club.club_id {
        return Err(ApiResponse::new(
            403,
            "The new owner is not a member of the club".to_string(),
        ));
    }

    // Retrieve the model and set the owner_id to the new_owner
    let mut club_model = club.into_active_model();
    club_model.owner_id = Set(new_owner_id);

    // Update the club owner
    club_model
        .update(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    return Ok(ApiResponse::new(
        200,
        "Club owner updated successfully".to_string(),
    ));
}

pub async fn is_owner(
    app_state: &web::Data<app_state::AppState>,
    user_id: i32,
    club_id: i32,
) -> Result<bool, ApiResponse> {
    let club = get_club_by_id(&app_state, club_id).await;

    // Error handling/formatting
    if club.is_err() {
        return Err(club.unwrap_err());
    }
    let club = club.unwrap();

    // Make sure the user deleting is the owner
    Ok(club.owner_id == user_id)
}

pub async fn get_club_if_owner(
    app_state: &web::Data<app_state::AppState>,
    user_id: i32,
) -> Result<entities::club::Model, ApiResponse> {
    // Get the users membership
    let membership = get_member_by_user_id(&app_state, user_id).await;

    // Error handling/formatting
    if membership.is_err() {
        return Err(membership.unwrap_err());
    }
    let membership = membership.unwrap();

    let club = get_club_by_id(&app_state, membership.club_id).await;

    // Error handling/formatting
    if club.is_err() {
        return Err(club.unwrap_err());
    }
    let club = club.unwrap();

    // Make sure the user deleting is the owner
    if club.owner_id == user_id {
        return Ok(club);
    } else {
        return Err(ApiResponse::new(
            401,
            "User is not the owner of this club".to_string(),
        ));
    }
}
