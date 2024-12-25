use actix_web::web;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};

use crate::{
    entities,
    routes::services::{club_member_service, user_service},
    utils::{
        api_response::ApiResponse, app_state, jwt::Claims,
        request_models::club_models::CreateClubModel,
    },
};

pub async fn get_club(
    app_state: &web::Data<app_state::AppState>,
    claim_data: Claims,
    filters: Option<Condition>,
) -> Result<ApiResponse, ApiResponse> {
    let mut query = entities::club_member::Entity::find();

    if let Some(filters) = filters {
        query = query.filter(filters);
    } else {
        query = query.filter(
            Condition::all().add(entities::club_member::Column::UserId.eq(claim_data.user_id)),
        );
    }

    let membership = query
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(404, "No club found for user".to_string()))?;

    let club = entities::club::Entity::find_by_id(membership.club_id)
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(404, "Club not found".to_string()))?;

    Ok(ApiResponse::new(
        200,
        format!(
            "{{ 'user_id': {}, 'club_id': {}, 'name': {} }}",
            claim_data.user_id, club.club_id, club.name
        ),
    ))
}

pub async fn create_club(
    app_state: &web::Data<app_state::AppState>,
    claim_data: Claims,
    json: web::Json<CreateClubModel>,
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
    if let Ok(_) = club_member_service::get_club_member(&app_state, claim_data.clone(), None).await
    {
        return Err(ApiResponse::new(
            409,
            "Users cannot be part of two clubs at once".to_string(),
        ));
    }

    // Check if the club already exists
    if let Some(_) = entities::club::Entity::find()
        .filter(entities::club::Column::Name.eq(&json.name.to_lowercase()))
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
        name: Set(json.name.clone().to_lowercase()),
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
