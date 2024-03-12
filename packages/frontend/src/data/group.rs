use bhw_types::{
    nothing::Nothing,
    requests::{
        create_group::{CreateGroupError, CreateGroupRequest, CreateGroupResponse},
        join_group::{JoinGroupError, JoinGroupRequest, JoinGroupResponse},
        leave_group::LeaveGroupResponseError,
        my_group::{MyGroupResponse, MyGroupResponseError},
        rename_group::{RenameGroupRequest, RenameGroupResponseError},
    },
};

use super::api::{send_get, send_post, FrontendRequestError};

pub async fn get_my_group() -> Result<MyGroupResponse, FrontendRequestError<MyGroupResponseError>> {
    send_get("/groups/me".to_string()).await
}

pub async fn create_group(
    group_name: String,
) -> Result<CreateGroupResponse, FrontendRequestError<CreateGroupError>> {
    let request = CreateGroupRequest { group_name };
    send_post("/groups".to_string(), &request).await
}

pub async fn change_my_group_name(
    new_name: String,
) -> Result<Nothing, FrontendRequestError<RenameGroupResponseError>> {
    let request = RenameGroupRequest { new_name };
    send_post("/groups/rename".to_string(), &request).await
}

pub async fn join_group(
    join_code: String,
) -> Result<JoinGroupResponse, FrontendRequestError<JoinGroupError>> {
    let request = JoinGroupRequest { join_code };
    send_post("/groups/join".to_string(), &request).await
}

pub async fn leave_group() -> Result<Nothing, FrontendRequestError<LeaveGroupResponseError>> {
    send_post("/groups/leave".to_string(), &Nothing {}).await
}
