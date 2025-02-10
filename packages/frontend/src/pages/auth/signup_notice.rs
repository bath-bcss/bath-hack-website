use bhw_types::requests::sign_up::{PossibleSignUpResponse, SignUpRequest};
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
    pages::auth::signup_activate::SignupActivateQueryParams,
    router::Route,
};

#[function_component(SignupNoticePage)]
pub fn signup_notice_page() -> Html {
    let username = use_location()
        .expect_throw("Location not found")
        .state::<String>();

    let navigator = use_navigator().expect_throw("Navigator not found");
    let loading_handle = use_state_eq(|| false);
    let loading = *loading_handle;
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
                } else if let Ok(response) = response {
                    if let PossibleSignUpResponse::Finished(response) = response {
                        navigator
                            .push_with_query(
                                &Route::ActivateAccount,
                                &SignupActivateQueryParams {
                                    id: response.id,
                                    secret: response.secret,
                                },
                            )
                            .expect("Query to serialise trivially");
                    } else {
                        navigator.push(&Route::SignupSuccess);
                    }
                }
            });
        },
    );

    if let Some(username) = username {
        html! {
            <HeroCenterContainer>
                <GlassContainer home_link=true>
                    <GlassContainerHeading>{ "Important information" }</GlassContainerHeading>
                    <GlassContainerParagraph>
                        { "Hi " }
                        { username.clone() }
                        { "!" }
                        { " We're so excited that you're joining our event!" }
                    </GlassContainerParagraph>
                    <GlassContainerParagraph top_margin=true>
                        { "By continuing, you confirm you have read and understood our " }
                        <a
                            href="https://docs.google.com/document/d/1qdNYvHsxai4Xr7qLl1RnQOMxZDuBqXSO7oGSoh-SVCo/edit?usp=sharing"
                            target="_blank"
                            class="underline"
                        >
                            { "Privacy Policy" }
                        </a>
                        { ". This website is operated jointly by WiT and BCSS; both Committees will have access to your information (as well as the specified third-parties)." }
                    </GlassContainerParagraph>
                    <Button
                        background_is_dark=false
                        class={classes!("mt-4")}
                        onclick={on_agree_click}
                        disabled={loading}
                    >
                        { "I confirm, sign up!" }
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
