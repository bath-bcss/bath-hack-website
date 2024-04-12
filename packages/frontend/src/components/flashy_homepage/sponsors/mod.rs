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
                name="Amazon Web Services"
                logo_url="img/sponsors/aws.svg"
                url="https://aws.amazon.com/"
            />
            <SponsorItem name="Arm" logo_url="img/sponsors/arm.svg" url="https://www.arm.com" />
            <SponsorItem
                name="Hewlett Packard Enterprise"
                logo_url="img/sponsors/hpe.svg"
                url="https://www.hpe.com"
            />
            <SponsorItem
                name="GitHub"
                logo_url="img/sponsors/github.webp"
                url="https://github.com"
            />
            <SponsorItem
                name="Unilever"
                logo_url="img/sponsors/unilever.svg"
                url="https://www.unilever.com"
            />
            <SponsorItem name="XMOS" logo_url="img/sponsors/xmos.webp" url="https://www.xmos.com" />
            <SponsorItem
                name="SparkLayer"
                logo_url="img/sponsors/sparklayer.webp"
                url="https://sparklayer.io"
            />
            <SponsorItem
                name="CiteAB"
                logo_url="img/sponsors/citeab.webp"
                url="https://citeab.com"
            />
            <SponsorItem
                name="Telekinetix"
                logo_url="img/sponsors/telekinetix.webp"
                url="https://www.telekinetix.com"
            />
            <SponsorItem
                name="University of Bath Department of Computer Science"
                logo_url="img/sponsors/uni.webp"
                url="https://www.bath.ac.uk/departments/department-of-computer-science/"
            />
            <SponsorItem
                name="The SU Groups"
                logo_url="img/sponsors/thesubath.webp"
                url="https://thesubath.com"
            />
        </div>
    }
}
