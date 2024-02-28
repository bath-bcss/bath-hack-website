use actix_web::{get, post, web};
use bhw_models::user;
use bhw_types::{
    models::group::GroupMember,
    requests::{
        create_group::{
            CreateGroupError, CreateGroupRequest, CreateGroupResponse, CreateGroupResult,
        },
        join_group::{JoinGroupError, JoinGroupRequest, JoinGroupResponse, JoinGroupResult},
        my_group::{MyGroupResponse, MyGroupResponseError, MyGroupResult},
    },
};
use log::warn;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    data::session::SessionUser,
    models::groups::{GetUserGroupError, GroupsHelper, JoinGroupByCodeError},
};

fn user_vec_to_members(user_vec: Vec<user::Model>) -> Vec<GroupMember> {
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

    Ok(res.map(|(group, members)| MyGroupResponse {
        id: group.id.to_string(),
        name: group.group_name,
        join_code: group.join_code,
        members: user_vec_to_members(members),
    }))
}

#[post("/groups")]
pub async fn create_group_route(
    user: SessionUser,
    db: web::Data<DatabaseConnection>,
    data: web::Json<CreateGroupRequest>,
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
                JoinGroupByCodeError::DBError(e) => {
                    warn!("joining group: {}", e);
                    JoinGroupError::DBError
                }
            })?;

    txn.commit().await?;

    Ok(JoinGroupResponse {
        group_id: group_details.id.to_string(),
        group_name: group_details.group_name,
        group_members: user_vec_to_members(group_members),
    })
}
