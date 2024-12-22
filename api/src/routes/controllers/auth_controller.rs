use actix_web::{post, web, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use sha256::digest;

use crate::{
    entities,
    utils::{api_response, app_state, jwt::encode_jwt},
};

#[derive(Serialize, Deserialize)]
struct RegisterModel {
    pub name_first: String,
    pub name_last: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct LoginModel {
    pub email: String,
    pub password: String,
}

#[post("register")]
pub async fn register(
    app_state: web::Data<app_state::AppState>,
    json: web::Json<RegisterModel>,
) -> impl Responder {
    let user_model = entities::user::ActiveModel {
        name_first: Set(json.name_first.clone()),
        name_last: Set(json.name_last.clone()),
        email: Set(json.email.clone()),
        password: Set(digest(&json.password)),
        user_type: Set("A".to_string()),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .unwrap();

    api_response::ApiResponse::new(200, format!("{}", user_model.user_id))
}

#[post("login")]
pub async fn login(
    app_state: web::Data<app_state::AppState>,
    json: web::Json<LoginModel>,
) -> impl Responder {
    let user = entities::user::Entity::find()
        .filter(
            Condition::all()
                .add(entities::user::Column::Email.eq(&json.email))
                .add(entities::user::Column::Password.eq(digest(&json.password))),
        )
        .one(&app_state.db)
        .await
        .unwrap();

    if user.is_none() {
        return api_response::ApiResponse::new(401, "User not found".to_string());
    }

    let user_data = user.unwrap();

    let token = encode_jwt(user_data.email, user_data.user_id).unwrap();

    api_response::ApiResponse::new(200, format!("{{ 'token': {}}}", token))
}
