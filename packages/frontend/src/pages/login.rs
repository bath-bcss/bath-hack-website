use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    components::{
        button::Button, error::ErrorMessage, glass_container::GlassContainer,
        input::Input, hero::center::HeroCenterContainer,
    },
    data::auth::sign_in,
    router::Route,
};

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let username_handle = use_state_eq(String::default);
    let username = (*username_handle).clone();
    let password_handle = use_state_eq(String::default);
    let password = (*password_handle).clone();

    let loading_handle = use_state_eq(|| false);
    let loading = (*loading_handle).clone();

    let error_handle = use_state_eq(|| None::<String>);
    let error = (*error_handle).clone();

    let navigator = use_navigator().expect_throw("Navigator not found");

    let on_sign_in_callback = use_callback(
        (username.clone(), password.clone(), navigator.clone()),
        move |e: SubmitEvent, _| {
            e.prevent_default();

            let username = username.clone();
            let password = password.clone();
            let error_handle = error_handle.clone();
            let loading_handle = loading_handle.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading_handle.set(true);
                let response = sign_in(username, password).await;
                loading_handle.set(false);

                match response {
                    Err(e) => error_handle.set(Some(e.to_string())),
                    Ok(_) => navigator.push(&Route::AccountHome),
                }
            });
        },
    );

    html! {
    <HeroCenterContainer>
        <GlassContainer home_link={true}>
            <h1 class="text-3xl font-hero text-bcss-900 dark:text-bcss-200 mb-4">
                {"Sign in"}
            </h1>

            <form onsubmit={on_sign_in_callback}>
                <Input handle={username_handle} input_label="Bath Username" placeholder="E.g. pk760" required={true}
                    input_type="text" input_class={classes!("mb-4")} disabled={loading.clone()} />

                <Input handle={password_handle} input_label="Password" placeholder="NOT your uni password"
                    required={true} input_type="password" input_class={classes!("mb-4")} disabled={loading.clone()} />

                <Button dark_mode={false} button_type="submit" disabled={loading.clone()}>
                    {"Sign in!"}
                </Button>

                <ErrorMessage message={error} />
            </form>
        </GlassContainer>
    </HeroCenterContainer>
    }
}
