use bhw_models::{group, prelude::*, user};
use rand::{rngs::OsRng, RngCore};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, Set,
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

#[derive(Debug, Error)]
pub enum GetGroupError {
    #[error("database error: {0}")]
    DBError(#[from] DbErr),
    #[error("User does not exist")]
    UserNotFound,
}

pub struct GroupsHelper;
impl GroupsHelper {
    pub async fn check_is_in_group<T: ConnectionTrait>(
        conn: &T,
        user_id: uuid::Uuid,
    ) -> Result<bool, DbErr> {
        let res = User::find()
            .filter(user::Column::Id.eq(user_id))
            .filter(user::Column::GroupId.is_not_null())
            .count(conn)
            .await?;

        Ok(res > 0)
    }

    pub async fn get_current_group<T: ConnectionTrait>(
        conn: &T,
        user_id: uuid::Uuid,
    ) -> Result<Option<group::Model>, GetGroupError> {
        let (_, user_group) = User::find()
            .find_also_related(Group)
            .filter(user::Column::Id.eq(user_id))
            .one(conn)
            .await?
            .ok_or(GetGroupError::UserNotFound)?;

        Ok(user_group)
    }

    fn generate_join_code() -> String {
        let mut rnd_data = [0u8; 3];
        OsRng.fill_bytes(&mut rnd_data);
        hex::encode(rnd_data)
    }

    pub async fn create<T: ConnectionTrait>(
        conn: &T,
        initial_user_id: uuid::Uuid,
        group_name: String,
    ) -> Result<group::Model, DbErr> {
        let new_group = group::ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            join_code: Set(Self::generate_join_code()),
            group_name: Set(group_name),
            ..Default::default()
        };

        let new_group: group::Model = new_group.insert(conn).await?;
        UserHelper::set_group_id(conn, initial_user_id, new_group.id).await?;

        Ok(new_group)
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
