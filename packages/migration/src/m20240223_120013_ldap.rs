use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(WebsiteUser::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(WebsiteUser::LdapCheckStatus)
                            .small_integer()
                            .default(0)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(SignupRequest::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(SignupRequest::LdapCheckStatus)
                            .small_integer()
                            .default(0)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(WebsiteUser::Table)
                    .drop_column(WebsiteUser::LdapCheckStatus)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(SignupRequest::Table)
                    .drop_column(SignupRequest::LdapCheckStatus)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum WebsiteUser {
    Table,
    LdapCheckStatus,
}

#[derive(DeriveIden)]
enum SignupRequest {
    Table,
    LdapCheckStatus,
}
