use actix_web::web;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

use crate::{
    entities,
    utils::{app_state, jwt::Claims},
};

pub async fn get_club_member(
    app_state: &web::Data<app_state::AppState>,
    claim_data: Claims,
    filters: Option<Condition>,
) -> Option<entities::club_member::Model> {
    let mut query = entities::club_member::Entity::find();

    if let Some(filter) = filters {
        query = query.filter(filter);
    } else {
        query = query.filter(entities::club_member::Column::UserId.eq(claim_data.user_id));
    }

    match query.one(&app_state.db).await {
        Ok(Some(user)) => Some(user),
        Ok(None) => None,
        Err(_) => None,
    }
}
