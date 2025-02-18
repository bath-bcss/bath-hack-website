use yew::prelude::*;

use crate::components::flashy_homepage::sponsors::sponsor_item::SponsorItem;

mod sponsor_item;

#[function_component(SponsorsGrid)]
pub fn sponsors_grid() -> Html {
    html! {
        <div
            class="mt-8 grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 auto-rows-fr gap-4 sm:gap-x-8 sm:gap-y-4 items-center"
            role="list"
        >
            <SponsorItem
                name="Multimatic"
                logo_url="img/sponsors/multimatic.svg"
                url="https://www.multimatic.com"
            />
            <SponsorItem
                name="SparkLayer"
                logo_url="img/sponsors/sparklayer.png"
                url="https://www.sparklayer.io/"
            />
        </div>
    }
}
