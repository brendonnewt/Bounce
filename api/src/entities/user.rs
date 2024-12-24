//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub name_first: String,
    pub name_last: String,
    pub email: String,
    pub password: String,
    #[sea_orm(column_type = "custom(\"enum_text\")")]
    pub user_type: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::club_member::Entity")]
    ClubMember,
    #[sea_orm(has_many = "super::session::Entity")]
    Session,
    #[sea_orm(has_many = "super::turn::Entity")]
    Turn,
}

impl Related<super::club_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ClubMember.def()
    }
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl Related<super::turn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Turn.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}