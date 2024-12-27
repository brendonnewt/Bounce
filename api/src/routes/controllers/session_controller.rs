use actix_web::{get, web};
use sea_orm::EntityTrait;

use crate::{
    entities,
    utils::{api_response::ApiResponse, app_state, jwt::Claims},
};

/*

Requirements:
    - Get all sessions by athlete_id (only athlete and coaches)
    - Get a specific session for an athlete (only athlete and coaches)
    - Create a new session (only athlete)
    - End a session (Maybe after 30 minutes of inactivity??)
*/

#[get("/{athlete_id}")]
pub async fn get_sessions_by_athlete(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
    //path: web::Path<i32>,
) -> Result<ApiResponse, ApiResponse> {
    //let athlete_id = path.into_inner();
    let user = entities::user::Entity::find_by_id(claim_data.user_id)
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(404, err.to_string()))?
        .ok_or(ApiResponse::new(404, "User not found".to_string()))?;

    Ok(ApiResponse::new(
        200,
        format!(
            "{{ 'user_id': {}, 'user_type': {}, 'name_first': {}, 'name_last': {}, 'email': {} }}",
            user.user_id, user.user_type, user.name_first, user.name_last, user.email
        ),
    ))
}
