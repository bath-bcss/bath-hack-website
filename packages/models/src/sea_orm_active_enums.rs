//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "registration_stage_enum"
)]
pub enum RegistrationStageEnum {
    #[sea_orm(string_value = "attendance")]
    Attendance,
    #[sea_orm(string_value = "pizza")]
    Pizza,
    #[sea_orm(string_value = "t_shirt")]
    TShirt,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "t_shirt_size_enum")]
pub enum TShirtSizeEnum {
    #[sea_orm(string_value = "l")]
    L,
    #[sea_orm(string_value = "m")]
    M,
    #[sea_orm(string_value = "s")]
    S,
    #[sea_orm(string_value = "xl")]
    Xl,
    #[sea_orm(string_value = "xxl")]
    Xxl,
    #[sea_orm(string_value = "xxxl")]
    Xxxl,
    #[sea_orm(string_value = "xxxxl")]
    Xxxxl,
}
