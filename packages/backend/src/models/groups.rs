use bhw_models::{group, prelude::*, user};
use rand::{rngs::OsRng, RngCore};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, Set,
};
use thiserror::Error;

use super::users::UserHelper;

#[derive(Debug, Error)]
pub enum JoinGroupByCodeError {
    #[error("database error: {0}")]
    DBError(#[from] DbErr),
    #[error("Join code not found")]
    NoJoinCode,
}

pub struct GroupsHelper;
impl GroupsHelper {
    pub async fn check_is_in_group<T: ConnectionTrait>(
        conn: &T,
        user_id: uuid::Uuid,
    ) -> Result<bool, DbErr> {
        let res = Group::find()
            .find_with_related(User)
            .filter(user::Column::Id.eq(user_id))
            .all(conn)
            .await?;

        Ok(res.len() > 0)
    }

    fn generate_join_code() -> String {
        let mut rnd_data = [0u8; 6];
        OsRng.fill_bytes(&mut rnd_data);
        hex::encode(rnd_data)
    }

    pub async fn create<T: ConnectionTrait>(
        conn: &T,
        initial_user_id: uuid::Uuid,
        group_name: String,
    ) -> Result<uuid::Uuid, DbErr> {
        let new_group = group::ActiveModel {
            join_code: Set(Self::generate_join_code()),
            group_name: Set(group_name),
            ..Default::default()
        };

        let new_group: group::Model = new_group.insert(conn).await?;
        UserHelper::set_group_id(conn, initial_user_id, new_group.id).await?;

        Ok(new_group.id)
    }

    pub async fn join_by_code<T: ConnectionTrait>(
        conn: &T,
        user_id: uuid::Uuid,
        join_code: String,
    ) -> Result<group::Model, JoinGroupByCodeError> {
        let group_with_code = Group::find()
            .filter(group::Column::JoinCode.eq(join_code))
            .one(conn)
            .await?
            .ok_or(JoinGroupByCodeError::NoJoinCode)?;

        UserHelper::set_group_id(conn, user_id, group_with_code.id).await?;

        Ok(group_with_code)
    }
}
