use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::sidebar::account_sidebar::AccountSidebar,
    pages::{
        account::{account_group::AccountGroupPage, account_home::AccountHomePage},
        auth::{
            forgot_password::ForgotPasswordPage, forgot_password_pin::ForgotPasswordPINPage,
            login::LoginPage, logout::LogoutPage, signup::SignupPage,
            signup_activate::SignupActivatePage, 
            signup_success::SignupSuccessPage, signup_notice::SignupNoticePage,
        },
        home::HomePage,
        not_found::NotFoundPage,
    },
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/signup/notice")]
    SignupNotice,
    #[at("/signup")]
    Signup,
    #[at("/signup/success")]
    SignupSuccess,
    #[at("/signup/activate")]
    ActivateAccount,
    #[at("/login")]
    Login,
    #[at("/logout")]
    Logout,
    #[at("/reset/password")]
    ForgotPassword,
    #[at("/reset/password/pin")]
    ForgotPasswordPIN,

    #[not_found]
    #[at("/404")]
    NotFound,

    #[at("/account")]
    AccountHome,
    #[at("/account/*")]
    Account,
}

#[derive(Clone, Routable, PartialEq)]
pub enum AccountRoute {
    #[at("/account")]
    Profile,
    #[at("/account/groups")]
    Groups,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
        <HomePage />
        },
        Route::Login => html! {
        <LoginPage />
        },
        Route::Logout => html! {
        <LogoutPage />},
        Route::SignupNotice => html! {
        <SignupNoticePage />
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
        Route::ForgotPassword => html! {
        <ForgotPasswordPage />
        },
        Route::ForgotPasswordPIN => html! {
        <ForgotPasswordPINPage />
        },
        Route::NotFound => html! {
        <NotFoundPage />
        },

        Route::AccountHome | Route::Account => html! {
        <AccountSidebar>
            <Switch<AccountRoute> render={switch_account} />
        </AccountSidebar>
        },
    }
}

pub fn switch_account(route: AccountRoute) -> Html {
    match route {
        AccountRoute::Profile => html! {
        <AccountHomePage /> },
        AccountRoute::Groups => html! {
        <AccountGroupPage />},
    }
}
