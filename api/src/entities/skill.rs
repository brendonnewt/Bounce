//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "skill")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub skill_id: i32,
    pub turn_id: i32,
    #[sea_orm(column_type = "custom(\"enum_text\")")]
    pub event_id: String,
    pub fig_rep: i32,
    #[sea_orm(column_type = "custom(\"enum_text\")")]
    pub position: String,
    #[sea_orm(column_type = "Float")]
    pub difficulty: f32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::turn::Entity",
        from = "Column::TurnId",
        to = "super::turn::Column::TurnId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Turn,
}

impl Related<super::turn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Turn.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}