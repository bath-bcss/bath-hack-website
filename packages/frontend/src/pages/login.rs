use yew::prelude::*;

use crate::{
    components::{button::Button, glass_container::GlassContainer, hero::HeroHeader, input::Input},
    data::auth::sign_in,
};

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let username_handle = use_state_eq(String::default);
    let username = (*username_handle).clone();
    let password_handle = use_state_eq(String::default);
    let password = (*password_handle).clone();

    let on_sign_in_callback = use_callback(
        (username.clone(), password.clone()),
        move |e: SubmitEvent, _| {
            e.prevent_default();

            let username = username.clone();
            let password = password.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = sign_in(username, password).await;
            });
        },
    );

    html! {
    <HeroHeader>
        <GlassContainer>
            <h1 class="text-3xl font-hero text-bcss-900 mb-4">
                {"Sign in"}
            </h1>

            <form onsubmit={on_sign_in_callback}>
                <Input handle={username_handle} input_label="Bath Username" placeholder="E.g. pk760" required={true}
                    input_type="text" button_class={classes!("mb-4")} />

                <Input handle={password_handle} input_label="Password" placeholder="NOT your uni password" required={true}
                    input_type="password" button_class={classes!("mb-4")} />

                <Button dark_mode={false} button_type="submit">
                    {"Sign in!"}
                </Button>
            </form>
        </GlassContainer>
    </HeroHeader>
    }
}
