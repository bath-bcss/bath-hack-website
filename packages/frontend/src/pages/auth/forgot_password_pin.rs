use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    components::{
        button::Button,
        error::ErrorMessage,
        form::input::Input,
        glass_container::{
            heading::GlassContainerHeading, paragraph::GlassContainerParagraph, GlassContainer,
        },
        hero::center::HeroCenterContainer,
    },
    data::auth::forgot_password_pin,
    router::Route,
};

#[function_component(ForgotPasswordPINPage)]
pub fn forgot_password_pin_page() -> Html {
    let pin_handle = use_state_eq(String::default);
    let new_password_handle = use_state_eq(String::default);

    let loading_handle = use_state_eq(|| false);
    let loading = *loading_handle;
    let error_handle = use_state_eq(|| None::<String>);
    let navigator = use_navigator().expect_throw("Navigator not found");

    let on_submit = use_callback(
        (
            pin_handle.clone(),
            new_password_handle.clone(),
            loading_handle.clone(),
            error_handle.clone(),
            navigator,
        ),
        |e: SubmitEvent,
         (pin_handle, new_password_handle, loading_handle, error_handle, navigator)| {
            e.prevent_default();

            let loading_handle = loading_handle.clone();
            let pin = pin_handle.to_string();
            let new_password = new_password_handle.to_string();
            let navigator = navigator.clone();
            let error_handle = error_handle.clone();

            wasm_bindgen_futures::spawn_local(async move {
                error_handle.set(None);
                loading_handle.set(true);
                let res = forgot_password_pin(pin, new_password).await;
                loading_handle.set(false);

                match res {
                    Err(e) => error_handle.set(Some(e.to_string())),
                    Ok(_) => navigator.push(&Route::Login),
                }
            });
        },
    );

    html! {
        <HeroCenterContainer>
            <GlassContainer>
                <GlassContainerHeading>{ "Reset your password" }</GlassContainerHeading>
                <GlassContainerParagraph>
                    { "Thanks! If your entered username exists in our database, we've sent you an email with a PIN to reset
                your
                password." }
                </GlassContainerParagraph>
                <GlassContainerParagraph>
                    { "For security reasons, we won't tell you if your username didn't exist. Also, you can only make one
                reset
                request every 15 minutes — if it's been less than that since your last one, we can't tell you!" }
                </GlassContainerParagraph>
                <GlassContainerParagraph top_margin=true>
                    { "If you received your PIN, please enter it here." }
                </GlassContainerParagraph>
                <form class="mt-4" onsubmit={on_submit}>
                    <Input
                        handle={pin_handle}
                        input_label="Reset PIN"
                        placeholder="abcdef123"
                        disabled={loading}
                    />
                    <Input
                        handle={new_password_handle}
                        input_label="New password"
                        input_type="password"
                        container_class={classes!("mt-4")}
                        disabled={loading}
                    />
                    <Button
                        background_is_dark=false
                        class={classes!("mt-4")}
                        disabled={loading}
                        button_type="submit"
                    >
                        { "Change my password!" }
                    </Button>
                </form>
                <ErrorMessage message={(*error_handle).clone()} />
            </GlassContainer>
        </HeroCenterContainer>
    }
}
