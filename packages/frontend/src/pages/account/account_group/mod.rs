use bhw_types::requests::my_group::MyGroupResponse;
use gloo_console::error;
use yew::prelude::*;

use crate::{
    components::{
        loading_spinner::LoadingSpinner, page_container::PageContainer, page_title::PageTitle,
    },
    data::group::get_my_group,
    pages::account::account_group::{
        in_group::AccountGroupManage, joining_group::AccountGroupJoining, types::FrontendGroupState,
    },
};

mod in_group;
mod joining_group;
mod member_card;
mod types;

#[function_component(AccountGroupPage)]
pub fn account_group_page() -> Html {
    let get_group_loading_handle = use_state_eq(|| false);
    let get_group_loading = (*get_group_loading_handle).clone();

    let current_group_handle = use_state_eq(|| None::<FrontendGroupState>);
    let current_group = (*current_group_handle).clone();

    {
        let current_group_handle = current_group_handle.clone();
        let get_group_loading_handle = get_group_loading_handle.clone();
        use_effect_with((), |_| {
            wasm_bindgen_futures::spawn_local(async move {
                get_group_loading_handle.set(true);
                let group = get_my_group().await;
                get_group_loading_handle.set(false);

                match group {
                    Err(e) => error!("getting group: ", e.to_string()),
                    Ok(group) => match group {
                        MyGroupResponse::None => return,
                        MyGroupResponse::Data(group) => {
                            current_group_handle.set(Some(FrontendGroupState {
                                name: group.name,
                                join_code: group.join_code,
                                members: group.members,
                            }))
                        }
                    },
                }
            });
        });
    }

    html! {
    <PageContainer>
        <PageTitle page_description="Manage your group membership">
            {"Your group"}
        </PageTitle>

        if get_group_loading {
        <LoadingSpinner />
        } else {
        if let Some(_) = current_group {
        <AccountGroupManage group_handle={current_group_handle} />
        } else {
        <AccountGroupJoining group_handle={current_group_handle} />
        }
        }
    </PageContainer>
    }
}
