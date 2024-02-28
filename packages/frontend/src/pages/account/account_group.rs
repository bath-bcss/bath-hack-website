use gloo_console::error;
use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    components::{
        button::Button, error::ErrorMessage, input::Input, loading_spinner::LoadingSpinner,
        page_container::PageContainer, page_control_heading::PageControlHeading,
        page_control_paragraph::PageControlParagraph, page_title::PageTitle,
    },
    data::group::{create_group, get_my_group, join_group},
    redirect_if_not_authed,
};

#[derive(Debug, Clone, PartialEq)]
struct FrontendGroupState {
    pub name: String,
    pub join_code: String,
}

#[function_component(AccountGroupPage)]
pub fn account_group_page() -> Html {
    let navigator = use_navigator().expect_throw("Navigator not found");
    use_effect_with((), move |_| {
        let navigator = navigator.clone();
        redirect_if_not_authed!(navigator);
    });

    let loading_handle = use_state_eq(|| false);
    let loading = (*loading_handle).clone();

    let current_group_handle = use_state_eq(|| None::<FrontendGroupState>);
    let current_group = (*current_group_handle).clone();

    {
        let current_group_handle = current_group_handle.clone();
        let loading_handle = loading_handle.clone();
        use_effect_with((), |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading_handle.set(true);
                let group = get_my_group().await;
                loading_handle.set(false);

                match group {
                    Err(e) => error!("getting group: ", e.to_string()),
                    Ok(group) => match group {
                        None => return,
                        Some(group) => current_group_handle.set(Some(FrontendGroupState {
                            name: group.name,
                            join_code: group.join_code,
                        })),
                    },
                }
            });
        });
    }

    let new_group_name_handle = use_state_eq(|| String::default());
    let new_group_name = (*new_group_name_handle).clone();

    let create_group_error_handle = use_state_eq(|| None::<String>);
    let create_group_error = (*create_group_error_handle).clone();

    let on_create_group_click = use_callback(
        (
        new_group_name.clone(),
        create_group_error_handle.clone(),
        current_group_handle.clone(),
        loading_handle.clone(),
        ),
        |e: SubmitEvent, (
        new_group_name,
        create_group_error_handle,
        current_group_handle,
        loading_handle
        )
        | {
        e.prevent_default();

        let new_group_name = new_group_name.clone();

        let create_group_error_handle = create_group_error_handle.clone();
        let current_group_handle = current_group_handle.clone();
        let loading_handle = loading_handle.clone();
        wasm_bindgen_futures::spawn_local(async move {
        create_group_error_handle.set(None);
        loading_handle.set(true);
        let new_group = create_group(new_group_name.clone()).await;
        loading_handle.set(false);

        match new_group {
        Err(e) => create_group_error_handle.set(Some(e.to_string())),
        Ok(new_group) => {
        current_group_handle.set(Some(FrontendGroupState {
        name: new_group_name,
        join_code: new_group.new_group_join_code,
        }));
        }
        }
        });
        });

    let join_code_handle = use_state_eq(|| String::default());
    let join_code = (*join_code_handle).clone();

    let join_group_error_handle = use_state_eq(|| None::<String>);
    let join_group_error = (*join_group_error_handle).clone();

    let on_join_group_click = use_callback(
        (
            loading_handle.clone(),
            join_code.clone(),
            join_group_error_handle.clone(),
            current_group_handle.clone(),
        ),
        |e: SubmitEvent,
         (loading_handle, join_code, join_group_error_handle, current_group_handle)| {
            e.prevent_default();

            let loading_handle = loading_handle.clone();
            let join_group_error_handle = join_group_error_handle.clone();
            let current_group_handle = current_group_handle.clone();
            let join_code = join_code.clone();
            wasm_bindgen_futures::spawn_local(async move {
                join_group_error_handle.set(None);
                loading_handle.set(true);
                let group = join_group(join_code.clone()).await;
                loading_handle.set(false);

                match group {
                    Err(e) => join_group_error_handle.set(Some(e.to_string())),
                    Ok(group) => current_group_handle.set(Some(FrontendGroupState {
                        name: group.group_name,
                        join_code,
                    })),
                }
            });
        },
    );

    html! {
    <PageContainer>
        <PageTitle page_description="Manage your group membership">
            {"Your group"}
        </PageTitle>

        if loading {
        <LoadingSpinner />
        } else {
        if let Some(current_group) = current_group {

        <PageControlHeading>
            {"Current group"}
        </PageControlHeading>
        <PageControlParagraph>
            {"You're currently a member of a group, which you either created or joined with a code."}
        </PageControlParagraph>

        <div class="mt-4 py-6 px-8 bg-bcss-100 shadow-md shadow-bcss-200 rounded-2xl inline-block">
            <p class="text-xl font-bold text-bcss-800">
                {current_group.name}
            </p>
            <p class="mt-1 text-bcss-900">
                {"Join code: "}
                <code>{current_group.join_code}</code>
            </p>
        </div>

        } else {

        <PageControlHeading>
            {"Create a new group"}
        </PageControlHeading>
        <PageControlParagraph>
            {"You'll need a group to enter Bath Hack. Your group can have between 1 and 4 members. When you
                    create a
                    group,
                    you'll be given a join code that you can share with your other teammates."}
        </PageControlParagraph>
        <form onsubmit={on_create_group_click}>
            <Input input_label="Group name" placeholder="E.g. The Cool Kids" handle={new_group_name_handle}
                disabled={loading} required={true} />
            <Button button_type="submit" dark_mode={false} class={classes!("mt-4")} disabled={loading}>
                {"Create!"}
            </Button>

            <ErrorMessage message={create_group_error} />
        </form>

        <PageControlHeading>
            {"Join a group"}
        </PageControlHeading>
        <PageControlParagraph>
            {"If someone's given you a join code, enter it here to join their group!"}
        </PageControlParagraph>
        <form onsubmit={on_join_group_click}>
            <Input input_label="Join code" placeholder="E.g. ab24be" handle={join_code_handle}
                disabled={loading} required={true} />
            <Button button_type="submit" dark_mode={false} class={classes!("mt-4")} disabled={loading}>
                {"Join!"}
            </Button>

            <ErrorMessage message={join_group_error} />
        </form>
        }
        }
    </PageContainer>
    }
}
