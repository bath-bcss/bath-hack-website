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
            .alter_table(
                Table::alter()
                    .table(WebsiteUser::Table)
                    .drop_column(WebsiteUser::AttendedAt)
                    .drop_column(WebsiteUser::AttendedBy)
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(RegistrationStageEnum)
                    .values(RegistrationStageVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ParticipantRegistration::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ParticipantRegistration::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ParticipantRegistration::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ParticipantRegistration::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ParticipantRegistration::CreatedBy)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ParticipantRegistration::Stage)
                            .enumeration(RegistrationStageEnum, RegistrationStageVariants::iter())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_registration_user")
                            .from(
                                ParticipantRegistration::Table,
                                ParticipantRegistration::UserId,
                            )
                            .to(WebsiteUser::Table, WebsiteUser::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ParticipantRegistration::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(Type::drop().name(RegistrationStageEnum).to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(WebsiteUser::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(WebsiteUser::AttendedAt).date_time().null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(WebsiteUser::AttendedBy).string().null(),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum WebsiteUser {
    Table,
    Id,
    AttendedAt,
    AttendedBy,
}

#[derive(DeriveIden)]
enum ParticipantRegistration {
    Table,
    Id,
    UserId,
    CreatedAt,
    CreatedBy,
    Stage,
}

#[derive(DeriveIden)]
struct RegistrationStageEnum;
#[derive(DeriveIden, EnumIter)]
enum RegistrationStageVariants {
    Attendance,
    TShirt,
    Pizza,
}
