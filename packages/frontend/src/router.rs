use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::sidebar::account_sidebar::AccountSidebar,
    pages::{
        account::{account_group::AccountGroupPage, account_home::AccountHomePage},
        home::HomePage,
        login::LoginPage,
        not_found::NotFoundPage,
        signup::SignupPage,
        signup_activate::SignupActivatePage,
        signup_success::SignupSuccessPage,
    },
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
