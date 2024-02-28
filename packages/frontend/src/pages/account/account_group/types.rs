use bhw_types::models::group::GroupMember;
use yew::{Properties, UseStateHandle};

#[derive(Debug, Clone, PartialEq)]
pub struct FrontendGroupState {
    pub name: String,
    pub join_code: String,
    pub members: Vec<GroupMember>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AccountGroupSubpageProps {
    pub group_handle: UseStateHandle<Option<FrontendGroupState>>,
}
