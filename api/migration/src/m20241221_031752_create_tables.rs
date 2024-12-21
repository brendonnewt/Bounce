use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        create_user_table(manager).await?;
        create_club_table(manager).await?;
        create_athlete_table(manager).await?;
        create_coach_table(manager).await?;
        create_session_table(manager).await?;
        create_turn_table(manager).await?;
        create_skill_table(manager).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        drop_skill_table(manager).await?;
        drop_turn_table(manager).await?;
        drop_session_table(manager).await?;
        drop_coach_table(manager).await?;
        drop_athlete_table(manager).await?;
        drop_club_table(manager).await?;
        drop_user_table(manager).await
    }
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241221_031752_create_tables"
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
                .col(
                    ColumnDef::new(User::UserType)
                        .enumeration(UserType::Table, vec![UserType::Coach, UserType::Athlete])
                        .not_null(),
                )
                .to_owned(),
        )
        .await
}

async fn create_club_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(Club::Table)
                .if_not_exists()
                .col(pk_auto(Club::ClubId))
                .col(string(Club::Name))
                .to_owned(),
        )
        .await
}

async fn create_athlete_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(Athlete::Table)
                .if_not_exists()
                .col(pk_auto(Athlete::AthleteId))
                .col(integer(Athlete::ClubId).null())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-athlete-club_id")
                        .from(Athlete::Table, Athlete::ClubId)
                        .to(Club::Table, Club::ClubId),
                )
                .to_owned(),
        )
        .await
}

async fn create_coach_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(Coach::Table)
                .if_not_exists()
                .col(pk_auto(Coach::CoachId))
                .col(integer(Coach::ClubId).null())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-coach-club_id")
                        .from(Coach::Table, Coach::ClubId)
                        .to(Club::Table, Club::ClubId),
                )
                .to_owned(),
        )
        .await
}

async fn create_session_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(Session::Table)
                .if_not_exists()
                .col(pk_auto(Session::SessionId))
                .col(
                    ColumnDef::new(Session::EventId)
                        .enumeration(Event::Table, vec![Event::TRA, Event::DMT, Event::TUM])
                        .not_null(),
                )
                .col(date_time(Session::TimeStart))
                .col(date_time(Session::TimeEnd))
                .to_owned(),
        )
        .await
}

async fn create_turn_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(Turn::Table)
                .if_not_exists()
                .col(pk_auto(Turn::TurnId))
                .col(integer(Turn::SessionId))
                .col(integer(Turn::AthleteId))
                .col(
                    ColumnDef::new(Turn::EventId)
                        .enumeration(Event::Table, vec![Event::TRA, Event::DMT, Event::TUM])
                        .not_null(),
                )
                .col(float(Turn::TotalDifficulty))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-turn-session_id")
                        .from(Turn::Table, Turn::SessionId)
                        .to(Session::Table, Session::SessionId),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-turn-athlete_id")
                        .from(Turn::Table, Turn::AthleteId)
                        .to(Athlete::Table, Athlete::AthleteId),
                )
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
                .col(integer(Skill::TurnId))
                .col(
                    ColumnDef::new(Skill::EventId)
                        .enumeration(Event::Table, vec![Event::TRA, Event::DMT, Event::TUM])
                        .not_null(),
                )
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
                .col(float(Skill::Difficulty))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-skill-turn_id")
                        .from(Skill::Table, Skill::TurnId)
                        .to(Turn::Table, Turn::TurnId),
                )
                .to_owned(),
        )
        .await
}

async fn drop_user_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(Table::drop().table(User::Table).to_owned())
        .await
}

async fn drop_club_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(Table::drop().table(Club::Table).to_owned())
        .await
}

async fn drop_athlete_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(Table::drop().table(Athlete::Table).to_owned())
        .await
}

async fn drop_coach_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(Table::drop().table(Coach::Table).to_owned())
        .await
}

async fn drop_session_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(Table::drop().table(Session::Table).to_owned())
        .await
}

async fn drop_turn_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(Table::drop().table(Turn::Table).to_owned())
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
    UserType,
}

#[derive(DeriveIden)]
enum UserType {
    Table,
    Coach,
    Athlete,
}

#[derive(DeriveIden)]
enum Club {
    Table,
    ClubId,
    Name,
}

#[derive(DeriveIden)]
enum Athlete {
    Table,
    AthleteId,
    ClubId,
}

#[derive(DeriveIden)]
enum Coach {
    Table,
    CoachId,
    ClubId,
}

#[derive(DeriveIden)]
enum Session {
    Table,
    SessionId,
    EventId,
    TimeStart,
    TimeEnd,
}

#[derive(DeriveIden)]
enum Turn {
    Table,
    TurnId,
    SessionId,
    AthleteId,
    EventId,
    TotalDifficulty,
}

#[derive(DeriveIden)]
enum Skill {
    Table,
    SkillId,
    TurnId,
    EventId,
    FigRep,
    Position,
    Difficulty,
}

#[derive(DeriveIden)]
enum Position {
    Table,
    TUCK,
    PIKE,
    STRAIGHT,
    SPLIT,
    NONE,
}

#[derive(DeriveIden)]
enum Event {
    Table,
    TRA,
    DMT,
    TUM,
}
