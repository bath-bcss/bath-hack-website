use bhw_types::requests::activate::AccountActivateRequest;
use serde::Deserialize;
use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::hooks::{use_location, use_navigator};

use crate::{
    components::{
        button::Button, error::ErrorMessage, glass_container::GlassContainer, hero::HeroHeader,
        input::Input,
    },
    data::sign_up::account_activate_request,
    router::Route,
};

#[derive(Debug, Deserialize)]
struct QueryParams {
    id: String,
    secret: String,
}

#[function_component(SignupActivatePage)]
pub fn signup_activate_page() -> Html {
    let location = use_location();

    let new_password_handle = use_state_eq(String::default);
    let new_password = (*new_password_handle).clone();

    let loading_handle = use_state_eq(|| false);
    let loading = (*loading_handle).clone();
    let error_handle = use_state_eq(|| None::<String>);
    let error = (*error_handle).clone();

    let navigator = use_navigator().expect_throw("Navigator not found");

    let on_activate_click = use_callback((location,), move |e: SubmitEvent, (location,)| {
        e.prevent_default();

        let location = location.clone().expect_throw("location was missing");

        let query_result = location.query::<QueryParams>();
        let query = match query_result {
            Err(e) => {
                error_handle.set(Some(e.to_string()));
                return;
            }
            Ok(q) => q,
        };

        let new_password = new_password.clone();
        let loading_handle = loading_handle.clone();
        let error_handle = error_handle.clone();
        let navigator = navigator.clone();
        wasm_bindgen_futures::spawn_local(async move {
            loading_handle.set(true);
            let resp = account_activate_request(&AccountActivateRequest {
                id: query.id,
                secret: query.secret,
                password: new_password,
            })
            .await;

            loading_handle.set(false);

            match resp {
                Err(e) => error_handle.set(Some(e.to_string())),
                Ok(_) => navigator.push(&Route::AccountHome),
            }
        });
    });

    html! {
    <HeroHeader>
        <GlassContainer>
            <h1 class="text-3xl font-hero text-bcss-900 dark:text-bcss-200">
                {"Welcome to Bath Hack 24!"}
            </h1>

            <p class="mt-4 dark:text-bcss-300">
                {"To get started, please create a password."}
            </p>
            <p class="dark:text-bcss-300">
                {"This should be different to your Uni password."}
            </p>

            <form class="mt-4" onsubmit={on_activate_click}>
                <Input input_label="New password" placeholder="NOT your uni password" input_type="password"
                    required={true} handle={new_password_handle} />

                <Button dark_mode={false} button_type="submit" class={classes!("mt-4")} disabled={loading}>
                    {"Activate account"}
                </Button>
            </form>

            <ErrorMessage message={error} />
        </GlassContainer>
    </HeroHeader>
    }
}
