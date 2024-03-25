use bhw_types::requests::sign_up::SignUpRequest;
use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::{
    components::{Link, Redirect},
    hooks::{use_location, use_navigator},
};

use crate::{
    components::{
        button::Button,
        error::ErrorMessage,
        glass_container::{
            heading::GlassContainerHeading, paragraph::GlassContainerParagraph, GlassContainer,
        },
        hero::center::HeroCenterContainer,
    },
    data::sign_up::sign_up_request,
    router::Route,
};

#[function_component(SignupNoticePage)]
pub fn signup_notice_page() -> Html {
    let username = use_location()
        .expect_throw("Location not found")
        .state::<String>();

    let navigator = use_navigator().expect_throw("Navigator not found");
    let loading_handle = use_state_eq(|| false);
    let loading = (*loading_handle).clone();
    let error_handle = use_state_eq(|| None::<String>);
    let error = (*error_handle).clone();

    let on_agree_click = use_callback(
        (username.clone(), navigator),
        move |_, (username, navigator)| {
            let loading_handle = loading_handle.clone();
            let error_handle = error_handle.clone();
            let navigator = navigator.clone();

            let username = username.clone().unwrap_throw();
            wasm_bindgen_futures::spawn_local(async move {
                loading_handle.set(true);
                error_handle.set(None);
                let response = sign_up_request(&SignUpRequest {
                    bath_username: username.as_str().to_string(),
                })
                .await;

                loading_handle.set(false);
                if let Err(response) = response {
                    error_handle.set(Some(response.to_string()));
                } else if let Ok(_) = response {
                    navigator.push(&Route::SignupSuccess);
                }
            });
        },
    );

    if let Some(username) = username {
        html! {
            <HeroCenterContainer>
                <GlassContainer home_link=true>
                    <GlassContainerHeading>{ "One moment..." }</GlassContainerHeading>
                    <GlassContainerParagraph>
                        { "Hi " }
                        { username.clone() }
                        { "!" }
                        { " We're so excited that you're joining our event!" }
                    </GlassContainerParagraph>
                    <GlassContainerParagraph top_margin=true>
                        { "Before we continue setting up your account: the legal bits! We will process your personal
                    information under The SU Bath's " }
                        <a
                            href="https://www.thesubath.com/pageassets/privacy/Privacy-Policy-2022.pdf"
                            target="_blank"
                            class="underline"
                        >
                            { "Privacy Policy" }
                        </a>
                        { ". BCSS Committee members will have access to the information you provide on a legitimate interest
                    basis,
                    as they need the information to run the event." }
                    </GlassContainerParagraph>
                    <GlassContainerParagraph>
                        { "Your information may be processed securely by third-parties also on a legitimate interest basis:
                    to facilitate the secure recording of your details and ensure communication." }
                    </GlassContainerParagraph>
                    <GlassContainerParagraph>
                        { "If you have any questions or would like to exercise your rights under the Data Protection Act
                    2018 and (where relevant) the General Data Protection Act, please email su-bcss@bath.ac.uk from your
                    University email account." }
                    </GlassContainerParagraph>
                    <GlassContainerParagraph top_margin=true>
                        { "By pressing the button below, you accept the above points and agree to allow BCSS to process the
                    personal information you provide." }
                    </GlassContainerParagraph>
                    <Button
                        dark_mode=false
                        class={classes!("mt-4")}
                        onclick={on_agree_click}
                        disabled={loading}
                    >
                        { "I agree, sign up!" }
                    </Button>
                    <ErrorMessage message={error.clone()} />
                    if error.is_some() {
                        <GlassContainerParagraph top_margin=true>
                            <Link<Route> to={Route::Signup} classes={classes!("underline")}>
                                { "Change my username" }
                            </Link<Route>>
                        </GlassContainerParagraph>
                    }
                </GlassContainer>
            </HeroCenterContainer>
        }
    } else {
        html! { <Redirect<Route> to={Route::Signup} /> }
    }
}
