use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::{
    button::Button, glass_container::GlassContainer, hero::HeroHeader, input::Input,
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

    let on_form_submit = {
        use_callback((), |e: SubmitEvent, _| {
            e.prevent_default();
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

                <Button dark_mode={false} class={classes!("mt-4")} button_type="submit">
                    {"Sign up!"}
                </Button>
            </form>
        </GlassContainer>
    </HeroHeader>
    }
}
