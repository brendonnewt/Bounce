use actix_web::web;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};

use crate::{
    entities,
    utils::{api_response::ApiResponse, app_state, jwt::Claims},
};

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

// pub async fn join_club(
//     app_state: &web::Data<app_state::AppState>,
//     claim_data: Claims,
//     club_id: i32,
// ) -> Option<entities::club_member::Model> {
// }
