use bhw_models::{group, prelude::*, user};
use rand::{rngs::OsRng, RngCore};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QuerySelect, SelectColumns, Set,
};
use thiserror::Error;

use super::users::UserHelper;

const MAX_MEMBERS: u64 = 4;

#[derive(Debug, Error)]
pub enum JoinGroupByCodeError {
    #[error("database error: {0}")]
    DBError(#[from] DbErr),
    #[error("Join code not found")]
    NoJoinCode,
    #[error("MaxCapacity")]
    MaxCapacity,
}

#[derive(Debug, Error)]
pub enum GetUserGroupError {
    #[error("database error: {0}")]
    DBError(#[from] DbErr),
    #[error("User does not exist")]
    UserNotFound,
}

#[derive(Debug, Error)]
enum GetGroupByIdError {
    #[error("database error: {0}")]
    DBError(#[from] DbErr),
    #[error("Group does not exist")]
    GroupNotFound,
}

#[derive(Debug, Error)]
pub enum RemoveUserFromGroupError {
    #[error("database error: {0}")]
    DBError(#[from] DbErr),
    #[error("User does not exist")]
    UserNotFound,
    #[error("User is not in a group")]
    UserNotInGroup,
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

    async fn get_group_by_id_with_members<T: ConnectionTrait>(
        conn: &T,
        group_id: uuid::Uuid,
    ) -> Result<(group::Model, Vec<user::Model>), GetGroupByIdError> {
        let group_with_code_res = Group::find_by_id(group_id)
            .find_with_related(User)
            .all(conn)
            .await?;

        let (group_with_code, group_members) = group_with_code_res
            .first()
            .ok_or(GetGroupByIdError::GroupNotFound)?;

        Ok((group_with_code.clone(), group_members.clone()))
    }

    pub async fn get_current_group<T: ConnectionTrait>(
        conn: &T,
        user_id: uuid::Uuid,
    ) -> Result<Option<(group::Model, Vec<user::Model>)>, GetUserGroupError> {
        let (user_group_id,): (Option<uuid::Uuid>,) = User::find_by_id(user_id)
            .select_only()
            .select_column(user::Column::GroupId)
            .into_tuple()
            .one(conn)
            .await?
            .ok_or(GetUserGroupError::UserNotFound)?;

        if let Some(user_group_id) = user_group_id {
            let res = Self::get_group_by_id_with_members(conn, user_group_id)
                .await
                .map_err(|e| match e {
                    GetGroupByIdError::GroupNotFound => GetUserGroupError::UserNotFound,
                    GetGroupByIdError::DBError(e) => GetUserGroupError::DBError(e),
                })?;

            Ok(Some(res))
        } else {
            Ok(None)
        }
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
        UserHelper::set_group_id(conn, initial_user_id, Some(new_group.id)).await?;

        Ok(new_group)
    }

    pub async fn join_by_code<T: ConnectionTrait>(
        conn: &T,
        user_id: uuid::Uuid,
        join_code: String,
    ) -> Result<(group::Model, Vec<user::Model>), JoinGroupByCodeError> {
        let (group_id,): (uuid::Uuid,) = Group::find()
            .select_only()
            .select_column(group::Column::Id)
            .filter(group::Column::JoinCode.eq(join_code))
            .into_tuple()
            .one(conn)
            .await?
            .ok_or(JoinGroupByCodeError::NoJoinCode)?;

        let num_members = User::find()
            .filter(user::Column::GroupId.eq(group_id))
            .count(conn)
            .await?;

        if num_members == MAX_MEMBERS {
            return Err(JoinGroupByCodeError::MaxCapacity);
        }

        UserHelper::set_group_id(conn, user_id, Some(group_id)).await?;

        Self::get_group_by_id_with_members(conn, group_id)
            .await
            .map_err(|e| match e {
                GetGroupByIdError::GroupNotFound => JoinGroupByCodeError::NoJoinCode,
                GetGroupByIdError::DBError(e) => JoinGroupByCodeError::DBError(e),
            })
    }

    pub async fn remove_user_from_group<T: ConnectionTrait>(
        conn: &T,
        user_id: uuid::Uuid,
    ) -> Result<(), RemoveUserFromGroupError> {
        let (group, group_members) = Self::get_current_group(conn, user_id)
            .await
            .map_err(|e| match e {
                GetUserGroupError::DBError(e) => RemoveUserFromGroupError::DBError(e),
                GetUserGroupError::UserNotFound => RemoveUserFromGroupError::UserNotFound,
            })?
            .ok_or(RemoveUserFromGroupError::UserNotInGroup)?;

        UserHelper::set_group_id(conn, user_id, None).await?;

        // The last person in the group just left, so we should delete it
        // to avoid orphaned groups
        if group_members.len() == 1 {
            Group::delete_by_id(group.id).exec(conn).await?;
        }

        Ok(())
    }
}
