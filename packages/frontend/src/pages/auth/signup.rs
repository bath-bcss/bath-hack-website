use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    components::{
        button::Button,
        form::input::Input,
        glass_container::{
            heading::GlassContainerHeading, paragraph::GlassContainerParagraph, GlassContainer,
        },
        hero::center::HeroCenterContainer,
    },
    router::Route,
};

#[function_component(SignupPage)]
pub fn sign_up_page() -> Html {
    let username_handle = use_state_eq(String::default);
    let username = (*username_handle).clone();

    let navigator = use_navigator().unwrap();
    let on_form_submit = use_callback(
        (navigator, username),
        move |e: SubmitEvent, (navigator, username)| {
            e.prevent_default();
            navigator.push_with_state::<_, String>(&Route::SignupNotice, username.clone());
        },
    );

    html! {
        <HeroCenterContainer>
            <GlassContainer home_link=true>
                <GlassContainerHeading>{ "Sign Up to Bath Hack" }</GlassContainerHeading>
                <GlassContainerParagraph>
                    { "We'll send an email to your University email account with a link to set up your account." }
                </GlassContainerParagraph>
                <form onsubmit={on_form_submit} class="mt-4">
                    <Input
                        input_label="Bath Username"
                        placeholder="E.g. pk760"
                        handle={username_handle}
                        required=true
                    />
                    <Button background_is_dark=false class={classes!("mt-4")} button_type="submit">
                        { "Sign up!" }
                    </Button>
                </form>
            </GlassContainer>
        </HeroCenterContainer>
    }
}
