use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CompetitionGroup::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CompetitionGroup::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CompetitionGroup::JoinCode)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(WebsiteUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WebsiteUser::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WebsiteUser::DisplayName).string().null())
                    .col(
                        ColumnDef::new(WebsiteUser::BathUsername)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(WebsiteUser::PasswordHash)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WebsiteUser::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(WebsiteUser::DietaryRequirements)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(WebsiteUser::AccessibilityRequirements)
                            .text()
                            .null(),
                    )
                    .col(ColumnDef::new(WebsiteUser::GroupId).uuid().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_competition_group")
                            .from(WebsiteUser::Table, WebsiteUser::GroupId)
                            .to(CompetitionGroup::Table, CompetitionGroup::Id)
                            .on_delete(ForeignKeyAction::SetNull),
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
            .drop_table(Table::drop().table(CompetitionGroup::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(WebsiteUser::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SignupRequest::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CompetitionGroup {
    Table,
    Id,
    JoinCode,
}

#[derive(DeriveIden)]
enum WebsiteUser {
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
