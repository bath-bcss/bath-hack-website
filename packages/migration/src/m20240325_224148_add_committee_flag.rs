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
                        ColumnDef::new(WebsiteUser::IsCommittee)
                            .boolean()
                            .default(false)
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
                    .drop_column(WebsiteUser::IsCommittee)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum WebsiteUser {
    Table,
    IsCommittee,
}
