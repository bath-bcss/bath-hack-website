use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PasswordReset::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PasswordReset::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PasswordReset::UserId)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_password_reset")
                            .from(PasswordReset::Table, PasswordReset::UserId)
                            .to(WebsiteUser::Table, WebsiteUser::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(PasswordReset::PIN)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PasswordReset::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(PasswordReset::ExpiresAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PasswordReset::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum WebsiteUser {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum PasswordReset {
    Table,
    Id,
    UserId,
    PIN,
    CreatedAt,
    ExpiresAt,
}
