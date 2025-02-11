use bhw_types::requests::activate::AccountActivateRequest;
use serde::{Deserialize, Serialize};
use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::hooks::{use_location, use_navigator};

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
    data::sign_up::account_activate_request,
    pages::account::account_home::InitialSignupState,
    router::Route,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct SignupActivateQueryParams {
    pub id: String,
    pub secret: String,
}

#[function_component(SignupActivatePage)]
pub fn signup_activate_page() -> Html {
    let location = use_location();

    let new_password_handle = use_state_eq(String::default);
    let new_password = (*new_password_handle).clone();

    let loading_handle = use_state_eq(|| false);
    let loading = *loading_handle;
    let error_handle = use_state_eq(|| None::<String>);
    let error = (*error_handle).clone();

    let navigator = use_navigator().expect_throw("Navigator not found");

    let on_activate_click = use_callback(
        (location, new_password),
        move |e: SubmitEvent, (location, new_password)| {
            e.prevent_default();

            let location = location.clone().expect_throw("location was missing");

            let query_result = location.query::<SignupActivateQueryParams>();
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
                    Ok(_) => navigator.push_with_state(&Route::AccountHome, InitialSignupState),
                }
            });
        },
    );

    html! {
        <HeroCenterContainer>
            <GlassContainer>
                <GlassContainerHeading>{ "One last step..." }</GlassContainerHeading>
                <GlassContainerParagraph top_margin=true>
                    { "To continue, please create a password." }
                </GlassContainerParagraph>
                <GlassContainerParagraph>
                    { "This should be different to your Uni password." }
                </GlassContainerParagraph>
                <form class="mt-4" onsubmit={on_activate_click}>
                    <Input
                        input_label="New password"
                        placeholder="NOT your uni password"
                        input_type="password"
                        required=true
                        handle={new_password_handle}
                        autocomplete="new-password"
                    />
                    <Button
                        background_is_dark=false
                        button_type="submit"
                        class={classes!("mt-4")}
                        disabled={loading}
                    >
                        { "Activate account" }
                    </Button>
                </form>
                <ErrorMessage message={error} />
            </GlassContainer>
        </HeroCenterContainer>
    }
}
