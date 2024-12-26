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
    - Get all turns for a session
    - Create a new turn for a session
        -- DD needs to be calculated on creation
        -- DD needs to be calculated according to the event the session is being created for
    - Edit a turn in a session (DD needs to be recalculated via DB trigger most likely)
        -- When a turn is editted, all of the associated skills will need to be deleted and reuploaded
        -- If the turn is 10 skills long, routine specific DD will need to be applied
    - Delete a turn in a session
*/
