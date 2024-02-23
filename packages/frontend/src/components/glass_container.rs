use yew::prelude::*;
use yew_router::components::Link;

use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    #[prop_or_default]
    pub home_link: bool,
}

#[function_component(GlassContainer)]
pub fn glass_container(props: &Props) -> Html {
    html! {
    <>
        <div class="p-12 bg-white/90 dark:bg-bcss-900/90 rounded-xl backdrop-blur drop-shadow-lg">
            {props.children.clone()}
        </div>

        if props.home_link {
        <p class="text-center text-bcss-200 dark:text-bcss-300 hover:underline">
            <Link<Route> to={Route::Home}>
                {"Back home"}
            </Link<Route>>
        </p>
        }
    </>
    }
}
