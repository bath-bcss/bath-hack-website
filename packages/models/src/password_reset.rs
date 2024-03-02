//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "password_reset")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub user_id: Uuid,
    #[sea_orm(unique)]
    pub pin: String,
    pub created_at: DateTime,
    pub expires_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::website_user::Entity",
        from = "Column::UserId",
        to = "super::website_user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    WebsiteUser,
}

impl Related<super::website_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WebsiteUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
