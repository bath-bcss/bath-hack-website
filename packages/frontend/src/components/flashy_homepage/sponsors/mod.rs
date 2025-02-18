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
                name="Frasers"
                logo_url="img/sponsors/frasers.svg"
                url="https://www.houseoffraser.co.uk"
            />
            <SponsorItem
                name="Airbus"
                logo_url="img/sponsors/airbus.svg"
                url="https://www.airbus.com"
            />
            <SponsorItem
                name="Siemens"
                logo_url="img/sponsors/siemens.svg"
                url="https://www.siemens.com/uk/en/industries.html"
            />
            <SponsorItem
                name="HomeLINK"
                logo_url="img/sponsors/homelink.svg"
                url="https://www.homelink.com/smart-home"
            />
            <SponsorItem
                name="Kingfisher"
                logo_url="img/sponsors/kingfisher.svg"
                url="https://www.kingfisher.com"
            />
            <SponsorItem
                name="Fivium"
                logo_url="img/sponsors/fivium.png"
                url="https://www.fivium.co.uk"
            />
        </div>
    }
}
