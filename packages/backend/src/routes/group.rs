use actix_web::{post, web};
use bhw_types::requests::{
    create_group::{CreateGroupError, CreateGroupRequest, CreateGroupResponse, CreateGroupResult},
    join_group::{JoinGroupError, JoinGroupRequest, JoinGroupResponse, JoinGroupResult},
};
use log::warn;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    data::session::SessionUser,
    models::groups::{GroupsHelper, JoinGroupByCodeError},
};

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

    let new_group_id = GroupsHelper::create(&txn, user.id, data.group_name.clone()).await?;

    txn.commit().await?;

    Ok(CreateGroupResponse {
        new_group_id: new_group_id.to_string(),
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

    let group_details = GroupsHelper::join_by_code(&txn, user.id, data.join_code.clone())
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
    })
}
