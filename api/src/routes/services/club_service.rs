use actix_web::web;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};

use crate::{
    entities,
    routes::services::user_service,
    utils::{
        api_response::ApiResponse, app_state, jwt::Claims,
        request_models::club_models::CreateClubModel,
    },
};

pub async fn get_user_club(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    let membership = entities::club_member::Entity::find()
        .filter(Condition::all().add(entities::club_member::Column::UserId.eq(claim_data.user_id)))
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
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
    json: web::Json<CreateClubModel>,
) -> Result<ApiResponse, ApiResponse> {
    // Ensure user trying to make club is a coach
    let filters = Some(
        Condition::all()
            .add(entities::user::Column::Email.eq(claim_data.email.clone()))
            .add(entities::user::Column::UserType.eq("C".to_string())),
    );

    let coach;

    if let Some(found_coach) =
        user_service::get_user(app_state.clone(), claim_data.clone(), filters).await
    {
        coach = found_coach;
    } else {
        return Err(ApiResponse::new(
            404,
            "Coach account could not be found for that email".to_string(),
        ));
    }

    // Check if the coach is already a member of a club
    if let Some(_) = entities::club_member::Entity::find()
        .filter(entities::club_member::Column::UserId.eq(coach.user_id))
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
    {
        return Err(ApiResponse::new(409, "Users cannot be part of two clubs at once. Please leave the club the account is registered for before creating a new one".to_string()));
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
        name: Set(json.name.clone()),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    // Associate the coach with the club
    let coach_membership = entities::club_member::ActiveModel {
        user_id: Set(coach.user_id),
        club_id: Set(club_model.club_id),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(
        200,
        format!(
            "{{ 'member_id': {}, 'club_id': {}, 'name': {} }}",
            coach_membership.club_member_id, club_model.club_id, club_model.name
        ),
    ))
}
