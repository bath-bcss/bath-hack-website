use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{switch, Route};

mod components;
mod data;
mod pages;
mod router;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <div id="modal_host" />
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
