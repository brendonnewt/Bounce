use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        create_user_table(manager).await?;
        create_club_table(manager).await?;
        create_club_member_table(manager).await?;
        create_session_table(manager).await?;
        create_turn_table(manager).await?;
        create_skill_table(manager).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        drop_skill_table(manager).await?;
        drop_turn_table(manager).await?;
        drop_session_table(manager).await?;
        drop_club_member_table(manager).await?;
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
                .col(string(User::NameFirst))
                .col(string(User::NameLast))
                .col(string(User::Email))
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
                .col(integer(Club::OwnerId))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-club-user_id")
                        .from(Club::Table, Club::OwnerId)
                        .to(User::Table, User::UserId)
                        .on_delete(ForeignKeyAction::SetNull),
                )
                .to_owned(),
        )
        .await
}

async fn create_club_member_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(ClubMember::Table)
                .if_not_exists()
                .col(pk_auto(ClubMember::ClubMemberId))
                .col(integer(ClubMember::UserId))
                .col(integer(ClubMember::ClubId))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-club_member-user_id")
                        .from(ClubMember::Table, ClubMember::UserId)
                        .to(User::Table, User::UserId)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-club_member-club_id")
                        .from(ClubMember::Table, ClubMember::ClubId)
                        .to(Club::Table, Club::ClubId)
                        .on_delete(ForeignKeyAction::Cascade),
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
                .col(integer(Session::UserId))
                .col(
                    ColumnDef::new(Session::EventId)
                        .enumeration(Event::Table, vec![Event::TRA, Event::DMT, Event::TUM])
                        .not_null(),
                )
                .col(date_time(Session::TimeStart))
                .col(date_time(Session::TimeEnd))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-turn-user_id")
                        .from(Session::Table, Session::UserId)
                        .to(User::Table, User::UserId),
                )
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
                .col(integer(Turn::UserId))
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
                        .name("fk-turn-user_id")
                        .from(Turn::Table, Turn::UserId)
                        .to(User::Table, User::UserId),
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

async fn drop_club_member_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(Table::drop().table(ClubMember::Table).to_owned())
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
    NameFirst,
    NameLast,
    Email,
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
    OwnerId,
    Name,
}

#[derive(DeriveIden)]
enum ClubMember {
    Table,
    ClubMemberId,
    UserId,
    ClubId,
}

#[derive(DeriveIden)]
enum Session {
    Table,
    SessionId,
    UserId,
    EventId,
    TimeStart,
    TimeEnd,
}

#[derive(DeriveIden)]
enum Turn {
    Table,
    TurnId,
    SessionId,
    UserId,
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
