use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

use crate::{
    components::{
        account::group::group_name::GroupName, button::Button, error::ErrorMessage,
        page_control_heading::PageControlHeading, page_control_paragraph::PageControlParagraph,
    },
    data::group::leave_group,
    pages::account::account_group::member_card::member_card,
};

use super::types::AccountGroupSubpageProps;

#[function_component(AccountGroupManage)]
pub fn account_group_manage(props: &AccountGroupSubpageProps) -> Html {
    let current_group = (*props.group_handle)
        .clone()
        .expect_throw("Cannot use AccountGroupManage with a None value for group_handle.");

    let leave_group_loading_handle = use_state_eq(|| false);
    let leave_group_loading = (*leave_group_loading_handle).clone();

    let leave_group_error_handle = use_state_eq(|| None::<String>);
    let leave_group_error = (*leave_group_error_handle).clone();

    let on_leave_group_click = use_callback(
        (
            props.group_handle.clone(),
            leave_group_loading_handle,
            leave_group_error_handle,
        ),
        |_, (group_handle, leave_group_loading_handle, leave_group_error_handle)| {
            let group_handle = group_handle.clone();
            let leave_group_error_handle = leave_group_error_handle.clone();
            let leave_group_loading_handle = leave_group_loading_handle.clone();
            wasm_bindgen_futures::spawn_local(async move {
                leave_group_error_handle.set(None);
                leave_group_loading_handle.set(true);
                let result = leave_group().await;
                leave_group_loading_handle.set(false);

                match result {
                    Err(e) => leave_group_error_handle.set(Some(e.to_string())),
                    Ok(_) => group_handle.set(None),
                }
            });
        },
    );

    html! {
        <>
            <PageControlHeading>{ "Current group" }</PageControlHeading>
            <PageControlParagraph>
                { "You're currently a member of a group, which you either created or joined with a code." }
            </PageControlParagraph>
            <div
                class="mt-4 py-6 px-8 bg-bcss-50 dark:bg-bcss-900 shadow-md shadow-bcss-200 dark:shadow-bcss-800 rounded-2xl block sm:inline-block"
            >
                <GroupName group={current_group.clone()} />
                <p class="mt-1 text-bcss-900 dark:text-bcss-200">
                    { "Join code: " }
                    <code>{ current_group.join_code }</code>
                </p>
                <div
                    class="mt-4 flex flex-col sm:flex-row items-start gap-y-4 sm:gap-y-0 sm:gap-x-4"
                >
                    { current_group.members.into_iter().map(|m| member_card(m)).collect::<Html>() }
                </div>
                <Button
                    class={classes!("mt-4")}
                    dark_mode=false
                    onclick={on_leave_group_click}
                    disabled={leave_group_loading}
                >
                    { "Leave group" }
                </Button>
                <ErrorMessage message={leave_group_error} />
            </div>
        </>
    }
}
