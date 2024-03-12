use yew::prelude::*;

use crate::{
    components::{
        button::Button, error::ErrorMessage, form::input::Input,
        page_control_heading::PageControlHeading, page_control_paragraph::PageControlParagraph,
    },
    data::group::join_group,
    pages::account::account_group::types::{AccountGroupSubpageProps, FrontendGroupState},
};

#[function_component(JoinGroup)]
pub fn join_group_component(props: &AccountGroupSubpageProps) -> Html {
    let form_loading_handle = use_state_eq(|| false);
    let form_loading = (*form_loading_handle).clone();

    let join_code_handle = use_state_eq(|| String::default());
    let join_code = (*join_code_handle).clone();

    let join_group_error_handle = use_state_eq(|| None::<String>);
    let join_group_error = (*join_group_error_handle).clone();

    let on_join_group_click = use_callback(
        (
            form_loading_handle.clone(),
            join_code.clone(),
            join_group_error_handle.clone(),
            props.group_handle.clone(),
        ),
        |e: SubmitEvent, (loading_handle, join_code, join_group_error_handle, update_group)| {
            e.prevent_default();

            let loading_handle = loading_handle.clone();
            let join_group_error_handle = join_group_error_handle.clone();
            let update_group = update_group.clone();
            let join_code = join_code.clone();
            wasm_bindgen_futures::spawn_local(async move {
                join_group_error_handle.set(None);
                loading_handle.set(true);
                let group = join_group(join_code.clone()).await;
                loading_handle.set(false);

                match group {
                    Err(e) => join_group_error_handle.set(Some(e.to_string())),
                    Ok(group) => update_group.set(Some(FrontendGroupState {
                        name: group.group_name,
                        join_code,
                        members: group.group_members,
                    })),
                }
            });
        },
    );

    html! {
        <>
            <PageControlHeading>{ "Join a group" }</PageControlHeading>
            <PageControlParagraph>
                { "If someone's given you a join code, enter it here to join their group!" }
            </PageControlParagraph>
            <form onsubmit={on_join_group_click}>
                <Input
                    input_label="Join code"
                    placeholder="E.g. ab24be"
                    handle={join_code_handle}
                    disabled={form_loading}
                    required=true
                />
                <Button
                    button_type="submit"
                    dark_mode=false
                    class={classes!("mt-4")}
                    disabled={form_loading}
                >
                    { "Join!" }
                </Button>
                <ErrorMessage message={join_group_error} />
            </form>
        </>
    }
}
