use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::{
    components::{button::Button, error::ErrorMessage, form::input::Input},
    data::group::change_my_group_name,
    pages::account::account_group::types::FrontendGroupState,
};

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Props {
    pub group: FrontendGroupState,
}

#[function_component(GroupName)]
pub fn group_name_component(props: &Props) -> Html {
    let is_editing_handle = use_state_eq(|| false);
    let is_editing = (*is_editing_handle).clone();
    let on_edit_click = {
        let is_editing_handle = is_editing_handle.clone();
        use_callback((), move |_, _| {
            is_editing_handle.set(true);
        })
    };

    let group_name_handle = use_state_eq(|| props.group.name.clone());
    let group_name = (*group_name_handle).clone();

    let loading_handle = use_state_eq(|| false);
    let loading = (*loading_handle).clone();
    let error_handle = use_state_eq(|| None::<String>);
    let error = (*error_handle).clone();

    let on_name_submit = use_callback(
        (
            group_name.clone(),
            loading_handle,
            error_handle,
            is_editing_handle,
        ),
        |e: SubmitEvent, (new_group_name, loading_handle, error_handle, is_editing_handle)| {
            e.prevent_default();

            let new_group_name = new_group_name.clone();
            let loading_handle = loading_handle.clone();
            let error_handle = error_handle.clone();
            let is_editing_handle = is_editing_handle.clone();
            wasm_bindgen_futures::spawn_local(async move {
                error_handle.set(None);
                loading_handle.set(true);
                let res = change_my_group_name(new_group_name).await;
                loading_handle.set(false);

                match res {
                    Err(e) => error_handle.set(Some(e.to_string())),
                    Ok(_) => is_editing_handle.set(false),
                }
            });
        },
    );

    html! {
    if !is_editing {
    <div class="flex items-center justify-between gap-x-4 mb-2">
        <p class="text-xl font-bold text-bcss-800 dark:text-bcss-200">
            {group_name}
        </p>
        <Button dark_mode={false} onclick={on_edit_click}>
            <Icon icon_id={IconId::FontAwesomeSolidPencil} class={classes!("h-4", "w-4" )} />
        </Button>
    </div>
    } else {
    <form class="mb-2" onsubmit={on_name_submit}>
        <Input handle={group_name_handle} disabled={loading.clone()} />
        <Button class={classes!("mt-2")} button_type={"submit"} dark_mode={false} disabled={loading.clone()}>
            {"Save"}
        </Button>
        <ErrorMessage message={error} />
    </form>
    }
    }
}
