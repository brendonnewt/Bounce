use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        create_user_table(manager).await?;
        create_skill_table(manager).await?;
        create_routine_table(manager).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        drop_user_table(manager).await?;
        drop_skill_table(manager).await?;
        drop_routine_table(manager).await
    }
}

async fn create_user_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(pk_auto(User::UserId))
                .col(string(User::Username))
                .col(string(User::Password))
                .to_owned(),
        )
        .await
}

async fn create_routine_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(Routine::Table)
                .if_not_exists()
                .col(pk_auto(Routine::RoutineId))
                .col(
                    ColumnDef::new(Routine::EventId)
                        .enumeration(
                            EventId::Table,
                            vec![EventId::TRA, EventId::DMT, EventId::TUM],
                        )
                        .not_null(),
                )
                .col(integer(Routine::UserId))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-routine-user_id")
                        .from(Routine::Table, Routine::UserId)
                        .to(User::Table, User::UserId),
                )
                .col(string(Routine::Name))
                .to_owned(),
        )
        .await
}

async fn create_skill_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(Skill::Table)
                .if_not_exists()
                .col(pk_auto(Skill::SkillId))
                .col(integer(Skill::FigRep))
                .col(
                    ColumnDef::new(Skill::Position)
                        .enumeration(
                            Position::Table,
                            vec![
                                Position::TUCK,
                                Position::PIKE,
                                Position::STRAIGHT,
                                Position::SPLIT,
                                Position::NONE,
                            ],
                        )
                        .not_null(),
                )
                .col(float(Skill::DD))
                .to_owned(),
        )
        .await
}

async fn drop_user_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(Table::drop().table(User::Table).to_owned())
        .await
}

async fn drop_routine_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(Table::drop().table(Routine::Table).to_owned())
        .await
}

async fn drop_skill_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(Table::drop().table(Skill::Table).to_owned())
        .await
}

#[derive(DeriveIden)]
enum User {
    Table,
    UserId,
    Username,
    Password,
}

#[derive(DeriveIden)]
enum Routine {
    Table,
    RoutineId,
    EventId,
    UserId,
    Name,
}

#[derive(Iden)]
enum EventId {
    Table,
    TRA,
    DMT,
    TUM,
}

#[derive(Iden)]
enum Skill {
    Table,
    SkillId,
    FigRep,
    Position,
    DD,
}

#[derive(Iden)]
enum Position {
    Table,
    TUCK,
    PIKE,
    STRAIGHT,
    SPLIT,
    NONE,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}
