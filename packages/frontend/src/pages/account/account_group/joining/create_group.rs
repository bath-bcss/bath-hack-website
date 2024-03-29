use bhw_types::models::group::GroupMember;
use yew::prelude::*;

use crate::{
    components::{
        button::Button, error::ErrorMessage, form::input::Input,
        page_control_heading::PageControlHeading, page_control_paragraph::PageControlParagraph,
    },
    data::group::create_group,
    pages::account::account_group::types::{AccountGroupSubpageProps, FrontendGroupState},
};

#[function_component(CreateGroup)]
pub fn create_group_component(props: &AccountGroupSubpageProps) -> Html {
    let form_loading_handle = use_state_eq(|| false);
    let form_loading = (*form_loading_handle).clone();

    let new_group_name_handle = use_state_eq(|| String::default());
    let new_group_name = (*new_group_name_handle).clone();

    let create_group_error_handle = use_state_eq(|| None::<String>);
    let create_group_error = (*create_group_error_handle).clone();

    let on_create_group_click = use_callback(
        (
            new_group_name.clone(),
            create_group_error_handle.clone(),
            form_loading_handle.clone(),
            props.group_handle.clone(),
        ),
        |e: SubmitEvent,
         (new_group_name, create_group_error_handle, loading_handle, update_group)| {
            e.prevent_default();

            let new_group_name = new_group_name.clone();

            let create_group_error_handle = create_group_error_handle.clone();
            let update_group = update_group.clone();
            let loading_handle = loading_handle.clone();
            wasm_bindgen_futures::spawn_local(async move {
                create_group_error_handle.set(None);
                loading_handle.set(true);
                let new_group = create_group(new_group_name.clone()).await;
                loading_handle.set(false);

                match new_group {
                    Err(e) => create_group_error_handle.set(Some(e.to_string())),
                    Ok(new_group) => {
                        update_group.set(Some(FrontendGroupState {
                            name: new_group_name,
                            join_code: new_group.new_group_join_code,
                            members: vec![GroupMember {
                                bath_username: "You!".to_string(),
                                display_name: None,
                            }],
                        }));
                    }
                }
            });
        },
    );

    html! {
        <>
            <PageControlHeading>{ "Create a new group" }</PageControlHeading>
            <PageControlParagraph>
                { "You'll need a group to enter Bath Hack. Your group can have between 1 and 4 members. When you create a
            group, you'll be given a join code that you can share with your other teammates." }
            </PageControlParagraph>
            <PageControlParagraph>
                { "Don't worry too much about your group name! You'll be able to change it any time before the competition
            starts." }
            </PageControlParagraph>
            <form onsubmit={on_create_group_click}>
                <Input
                    input_label="Group name"
                    placeholder="E.g. The Cool Kids"
                    handle={new_group_name_handle}
                    disabled={form_loading}
                    required=true
                />
                <Button
                    button_type="submit"
                    dark_mode=false
                    class={classes!("mt-4")}
                    disabled={form_loading}
                >
                    { "Create!" }
                </Button>
                <ErrorMessage message={create_group_error} />
            </form>
        </>
    }
}
