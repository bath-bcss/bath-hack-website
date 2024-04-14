use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(CompetitionGroup::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(CompetitionGroup::GroupNumber)
                            .auto_increment()
                            .integer()
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
                    .table(CompetitionGroup::Table)
                    .drop_column(CompetitionGroup::GroupNumber)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum CompetitionGroup {
    Table,
    GroupNumber,
}
