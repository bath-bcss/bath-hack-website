use pages::{home::HomePage, login::LoginPage, not_found::NotFoundPage};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

mod components;
mod pages;
mod router;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
        <HomePage /> },
        Route::Login => html! {
        <LoginPage /> },
        Route::NotFound => html! {
        <NotFoundPage /> },
    }
}

#[function_component]
fn App() -> Html {
    html! {
    <BrowserRouter>
        <Switch<Route> render={switch} />
    </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
