use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    home::HomePage, login::LoginPage, not_found::NotFoundPage, signup::SignupPage,
    signup_activate::SignupActivatePage, signup_success::SignupSuccessPage,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/signup")]
    Signup,
    #[at("/signup/success")]
    SignupSuccess,
    #[at("/signup/activate")]
    ActivateAccount,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
        <HomePage />
        },
        Route::Login => html! {
        <LoginPage />
        },
        Route::Signup => html! {
        <SignupPage />
        },
        Route::SignupSuccess => html! {
        <SignupSuccessPage />
        },
        Route::ActivateAccount => html! {
        <SignupActivatePage />
        },
        Route::NotFound => html! {
        <NotFoundPage />
        },
    }
}
