use bhw_types::requests::sign_up::SignUpRequest;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    components::{
        button::Button, error::ErrorMessage, glass_container::GlassContainer,
        hero::center::HeroCenterContainer, input::Input,
    },
    data::sign_up::sign_up_request,
    router::Route,
};

#[function_component(SignupPage)]
pub fn sign_up_page() -> Html {
    let username_handle = use_state_eq(String::default);
    let username = (*username_handle).clone();

    let loading_handle = use_state_eq(|| false);
    let loading = (*loading_handle).clone();
    let error_handle = use_state_eq(|| Option::<String>::None);
    let error = (*error_handle).clone();

    let navigator = use_navigator().unwrap();
    let on_form_submit = {
        let loading_handle = loading_handle.clone();
        let navigator = navigator.clone();
        let username = username.clone();
        let error_handle = error_handle.clone();

        use_callback(
            (navigator, username),
            move |e: SubmitEvent, (navigator, username)| {
                e.prevent_default();

                let n = (*navigator).clone();

                let loading_handle = loading_handle.clone();
                let error_handle = error_handle.clone();
                let username = username.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let n = n.clone();

                    loading_handle.set(true);
                    error_handle.set(None);
                    let response = sign_up_request(&SignUpRequest {
                        bath_username: username.clone(),
                    })
                    .await;

                    loading_handle.set(false);
                    if let Err(response) = response {
                        error_handle.set(Some(response.to_string()));
                    } else if let Ok(_) = response {
                        n.push(&Route::SignupSuccess);
                    }
                });
            },
        )
    };

    html! {
    <HeroCenterContainer>
        <GlassContainer home_link={true}>
            <h1 class="text-3xl font-hero text-bcss-900 dark:text-bcss-200 mb-4">
                {"Sign Up to Bath Hack"}
            </h1>

            <form onsubmit={on_form_submit}>
                <Input input_label="Bath Username" placeholder="E.g. pk760" handle={username_handle} required={true} />

                <Button dark_mode={false} class={classes!("mt-4")} button_type="submit" disabled={loading}>
                    {"Sign up!"}
                </Button>
            </form>

            <ErrorMessage message={error} />
        </GlassContainer>
    </HeroCenterContainer>
    }
}
