use yew::prelude::*;

use crate::components::hero::HeroHeader;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
    <HeroHeader>
        <h1 class="text-hero font-hero text-bcss-900">{ "Bath Hack 2024" }</h1>
    </HeroHeader>
    }
}
