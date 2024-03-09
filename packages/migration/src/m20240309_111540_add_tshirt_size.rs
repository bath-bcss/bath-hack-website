use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(TShirtSizeEnum)
                    .values(TShirtSizeVariants::iter())
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(WebsiteUser::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(WebsiteUser::TShirtSize)
                            .enumeration(TShirtSizeEnum, TShirtSizeVariants::iter())
                            .null(),
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
                    .drop_column(WebsiteUser::TShirtSize)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_type(Type::drop().name(TShirtSizeEnum).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum WebsiteUser {
    Table,
    TShirtSize,
}

#[derive(DeriveIden)]
struct TShirtSizeEnum;
#[derive(DeriveIden, EnumIter)]
enum TShirtSizeVariants {
    S,
    M,
    L,
    XL,
    XXL,
    XXXL,
    XXXXL,
}
