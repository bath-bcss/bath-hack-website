use bhw_types::requests::sign_up::SignUpRequest;
use gloo_console::log;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    components::{button::Button, glass_container::GlassContainer, hero::HeroHeader, input::Input},
    data::sign_up::sign_up_request,
    router::Route,
};

#[function_component(SignupPage)]
pub fn sign_up_page() -> Html {
    let username_handle = use_state(String::default);
    let username = (*username_handle).clone();
    let on_username_change = {
        let username_handle = username_handle.clone();

        use_callback((), move |value: InputEvent, _| {
            let target = value.target_dyn_into::<HtmlInputElement>();
            if let Some(target) = target {
                username_handle.set(target.value());
            }
        })
    };

    let loading_handle = use_state(|| false);
    let loading = (*loading_handle).clone();
    let navigator = use_navigator().unwrap();
    let on_form_submit = {
        let loading_handle = loading_handle.clone();
        let navigator = navigator.clone();

        use_callback((navigator,), move |e: SubmitEvent, (navigator,)| {
            e.prevent_default();

            let n = (*navigator).clone();

            let loading_handle = loading_handle.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let n = n.clone();

                loading_handle.set(true);
                let response = sign_up_request(SignUpRequest {
                    bath_username: "pal".to_string(),
                })
                .await;

                loading_handle.set(false);
                if let Err(response) = response {
                    log!("something went wrong :(");
                    log!(response.to_string());
                } else if let Ok(response) = response {
                    if response.success {
                        n.push(&Route::SignupSuccess);
                    } else {
                        window()
                            .unwrap()
                            .alert_with_message("Request failed")
                            .unwrap();
                    }
                }
            });
        })
    };

    html! {
    <HeroHeader>
        <GlassContainer>
            <h1 class="text-3xl font-hero text-bcss-900 mb-4">
                {"Sign Up to Bath Hack"}
            </h1>

            <form onsubmit={on_form_submit}>
                <Input input_label="Bath Username" placeholder="E.g. pk760" onchange={on_username_change}
                    value={username} />

                <Button dark_mode={false} class={classes!("mt-4")} button_type="submit" disabled={loading}>
                    {"Sign up!"}
                </Button>
            </form>
        </GlassContainer>
    </HeroHeader>
    }
}
