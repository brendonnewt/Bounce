// use actix_web::{get, post, web};
// use sea_orm::EntityTrait;

// use crate::{
//     entities,
//     routes::services::user_service,
//     utils::{
//         api_response::ApiResponse,
//         app_state,
//         jwt::Claims,
//         request_models::user_models::{UpdatePasswordModel, UpdateUserModel},
//     },
// };

/*

Requirements:
    - Get all sessions by athlete_id (only athlete and coaches)
    - Get a specific session for an athlete (only athlete and coaches)
    - Create a new session (only athlete)
    - End a session (Maybe after 30 minutes of inactivity??)
*/
