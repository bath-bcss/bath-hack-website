use bhw_types::common_data::EVENT_NAME;
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
            navigator.push_with_state::<_, String>(
                &Route::SignupNotice,
                username
                    .trim()
                    .to_lowercase()
                    // If users accidentally enter an email instead of a username, be nice and kind
                    // and remove it for them
                    .trim_end_matches("@bath.ac.uk")
                    .to_string(),
            );
        },
    );

    html! {
        <HeroCenterContainer>
            <GlassContainer home_link=true>
                <GlassContainerHeading>{ "Sign Up to " }{ EVENT_NAME }</GlassContainerHeading>
                /*<GlassContainerParagraph>
                    { "Sign-ups are currently closed, and will open at 12pm on Thursday 6th March. Places are likely to go extremely quickly, so be ready!" }
                </GlassContainerParagraph>
                <CountdownTimer
                    time_to={Utc.with_ymd_and_hms(2025, 3, 6, 12, 0, 0).unwrap()}
                    target_name="Sign-ups open in"
                    light=true
                    class={classes!("mt-4")}
                />*/<GlassContainerParagraph>
                    { "We need your username to identify you and send event-related communications." }
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
