use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Group::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Group::JoinCode).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(User::DisplayName).string().null())
                    .col(
                        ColumnDef::new(User::BathUsername)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::PasswordHash).string().not_null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(User::DietaryRequirements).text().null())
                    .col(
                        ColumnDef::new(User::AccessibilityRequirements)
                            .text()
                            .null(),
                    )
                    .col(ColumnDef::new(User::GroupId).uuid().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_group")
                            .from(User::Table, User::GroupId)
                            .to(Group::Table, Group::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SignupRequest::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SignupRequest::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SignupRequest::BathUsername)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(SignupRequest::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(SignupRequest::ExpiresAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SignupRequest::SecretHash)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SignupRequest::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Group {
    Table,
    Id,
    JoinCode,
}
/**
*
CREATE TABLE groups (
id uuid primary key not null,
join_code char(8) unique
);

CREATE TABLE users (
id uuid primary key not null,
display_name text not null,
bath_username text unique not null,
password_hash text not null,
created_at timestamp not null default now(),
dietary_requirements text not null,
accessibility_requirements text not null,
group_id uuid,
constraint fk_group
foreign key(group_id)
references groups(id)
);

CREATE TABLE signup_request (
id uuid primary key not null,
bath_username text unique not null,
created_at Timestamp not null default now(),
expires Timestamp not null,
secret_hash text not null
);
*/

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    DisplayName,
    BathUsername,
    PasswordHash,
    CreatedAt,
    DietaryRequirements,
    AccessibilityRequirements,
    GroupId,
}

#[derive(DeriveIden)]
enum SignupRequest {
    Table,
    Id,
    BathUsername,
    CreatedAt,
    ExpiresAt,
    SecretHash,
}
