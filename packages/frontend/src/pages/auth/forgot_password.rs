use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    components::{
        button::Button,
        error::ErrorMessage,
        glass_container::{
            heading::GlassContainerHeading, paragraph::GlassContainerParagraph, GlassContainer,
        },
        hero::center::HeroCenterContainer,
        input::Input,
    },
    data::auth::forgot_password,
    router::Route,
};

#[function_component(ForgotPasswordPage)]
pub fn forgot_password_page() -> Html {
    let username_handle = use_state_eq(|| String::default());
    let error_handle = use_state_eq(|| None::<String>);
    let loading_handle = use_state_eq(|| false);
    let loading = (*loading_handle).clone();
    let navigator = use_navigator().expect_throw("Navigator not found");

    let on_submit = use_callback(
        (
            username_handle.clone(),
            error_handle.clone(),
            loading_handle.clone(),
            navigator,
        ),
        |e: SubmitEvent, (username_handle, error_handle, loading_handle, navigator)| {
            e.prevent_default();

            let username = username_handle.to_string();
            let error_handle = error_handle.clone();
            let loading_handle = loading_handle.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                error_handle.set(None);
                loading_handle.set(true);
                let res = forgot_password(username).await;
                loading_handle.set(false);

                match res {
                    Err(e) => error_handle.set(Some(e.to_string())),
                    Ok(_) => navigator.push(&Route::ForgotPasswordPIN),
                }
            });
        },
    );

    html! {
    <HeroCenterContainer>
        <GlassContainer>
            <GlassContainerHeading>
                {"Reset your password"}
            </GlassContainerHeading>
            <GlassContainerParagraph>
                {"If you've forgotten your password, don't worry! You can enter your Bath username here and we'll send
                you
                an email to reset it."}
            </GlassContainerParagraph>

            <form class="mt-4" onsubmit={on_submit}>
                <Input handle={username_handle} input_label="Bath Username" placeholder="E.g. pk760"
                    disabled={loading} />
                <Button button_type="submit" dark_mode={false} class={classes!("mt-4")} disabled={loading}>
                    {"Let's do this!"}
                </Button>
            </form>

            <ErrorMessage message={(*error_handle).clone()} />
        </GlassContainer>
    </HeroCenterContainer>
    }
}
