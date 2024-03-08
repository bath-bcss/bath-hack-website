use actix_web::{get, post, web};
use bhw_models::website_user;
use bhw_types::{
    models::group::GroupMember,
    nothing::Nothing,
    requests::{
        create_group::{
            CreateGroupError, CreateGroupRequest, CreateGroupResponse, CreateGroupResult,
        },
        join_group::{JoinGroupError, JoinGroupRequest, JoinGroupResponse, JoinGroupResult},
        leave_group::{LeaveGroupResponseError, LeaveGroupResult},
        my_group::{MyGroupData, MyGroupResponse, MyGroupResponseError, MyGroupResult},
        rename_group::{RenameGroupRequest, RenameGroupResponseError, RenameGroupResult},
    },
};
use log::error;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    data::session::SessionUser,
    models::groups::{
        GetUserGroupError, GroupsHelper, JoinGroupByCodeError, RemoveUserFromGroupError,
        RenameMemberGroupError,
    },
};

fn user_vec_to_members(user_vec: Vec<website_user::Model>) -> Vec<GroupMember> {
    user_vec
        .iter()
        .map(|m| GroupMember {
            bath_username: m.bath_username.clone(),
            display_name: m.display_name.clone(),
        })
        .collect()
}

#[get("/groups/me")]
pub async fn get_my_group_route(
    user: SessionUser,
    db: web::Data<DatabaseConnection>,
) -> MyGroupResult {
    let res = GroupsHelper::get_current_group(db.get_ref(), user.id)
        .await
        .map_err(|e| match e {
            GetUserGroupError::UserNotFound => MyGroupResponseError::UserNotFound,
            GetUserGroupError::DBError(e) => e.into(),
        })?;

    let res = res.map(|(group, members)| MyGroupData {
        id: group.id.to_string(),
        name: group.group_name,
        join_code: group.join_code,
        members: user_vec_to_members(members),
    });

    if let Some(res) = res {
        Ok(MyGroupResponse::Data(res))
    } else {
        Ok(MyGroupResponse::None)
    }
}

#[post("/groups")]
pub async fn create_group_route(
    user: SessionUser,
    db: web::Data<DatabaseConnection>,
    data: bhw_types::actix_web_validator::Json<CreateGroupRequest>,
) -> CreateGroupResult {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await?;

    let is_already_in_group = GroupsHelper::check_is_in_group(&txn, user.id).await?;
    if is_already_in_group {
        return Err(CreateGroupError::AlreadyInGroup);
    }

    let new_group = GroupsHelper::create(&txn, user.id, data.group_name.clone()).await?;

    txn.commit().await?;

    Ok(CreateGroupResponse {
        new_group_id: new_group.id.to_string(),
        new_group_join_code: new_group.join_code,
    })
}

#[post("/groups/rename")]
pub async fn rename_my_group_route(
    user: SessionUser,
    db: web::Data<DatabaseConnection>,
    data: bhw_types::actix_web_validator::Json<RenameGroupRequest>,
) -> RenameGroupResult {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await?;

    GroupsHelper::rename_member_group(&txn, user.id, data.new_name.clone())
        .await
        .map_err(|e| match e {
            RenameMemberGroupError::DBError(e) => {
                error!("rename group: {}", e.to_string());
                RenameGroupResponseError::DBError
            }
            RenameMemberGroupError::UserNotFound => RenameGroupResponseError::UserNotFound,
            RenameMemberGroupError::UserNotInGroup => RenameGroupResponseError::UserNotInGroup,
        })?;

    txn.commit().await?;

    Ok(Nothing)
}

#[post("/groups/join")]
pub async fn join_group_route(
    user: SessionUser,
    db: web::Data<DatabaseConnection>,
    data: web::Json<JoinGroupRequest>,
) -> JoinGroupResult {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await?;

    let is_already_in_group = GroupsHelper::check_is_in_group(&txn, user.id).await?;
    if is_already_in_group {
        return Err(JoinGroupError::AlreadyInGroup);
    }

    let (group_details, group_members) =
        GroupsHelper::join_by_code(&txn, user.id, data.join_code.clone())
            .await
            .map_err(|e| match e {
                JoinGroupByCodeError::NoJoinCode => JoinGroupError::CodeNotFound,
                JoinGroupByCodeError::MaxCapacity => JoinGroupError::MaxCapacity,
                JoinGroupByCodeError::DBError(e) => e.into(),
            })?;

    txn.commit().await?;

    Ok(JoinGroupResponse {
        group_id: group_details.id.to_string(),
        group_name: group_details.group_name,
        group_members: user_vec_to_members(group_members),
    })
}

#[post("/groups/leave")]
pub async fn leave_group_route(
    user: SessionUser,
    db: web::Data<DatabaseConnection>,
) -> LeaveGroupResult {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::Serializable),
            Some(AccessMode::ReadWrite),
        )
        .await?;

    GroupsHelper::remove_user_from_group(&txn, user.id)
        .await
        .map_err(|e| match e {
            RemoveUserFromGroupError::DBError(e) => e.into(),
            RemoveUserFromGroupError::UserNotFound => LeaveGroupResponseError::UserNotFound,
            RemoveUserFromGroupError::UserNotInGroup => LeaveGroupResponseError::NotInGroup,
        })?;

    txn.commit().await?;

    Ok(Nothing)
}
