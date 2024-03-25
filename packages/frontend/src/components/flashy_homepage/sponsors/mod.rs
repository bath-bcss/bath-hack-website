use yew::prelude::*;

use crate::components::flashy_homepage::sponsors::sponsor_item::SponsorItem;

mod sponsor_item;

#[function_component(SponsorsGrid)]
pub fn sponsors_grid() -> Html {
    html! {
        <div
            class="mt-8 grid grid-cols-1 md:grid-cols-3 lg:grid-cols-4 gap-x-6 gap-y-12 items-center"
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
            <SponsorItem name="GitHub" logo_url="img/sponsors/github.png" url="https://github.com" />
            <SponsorItem name="XMOS" logo_url="img/sponsors/xmos.png" url="https://www.xmos.com" />
            <SponsorItem
                name="University of Bath Department of Computer Science"
                logo_url="img/sponsors/uni.png"
                url="https://www.bath.ac.uk/departments/department-of-computer-science/"
            />
            <SponsorItem
                name="The SU Groups"
                logo_url="img/sponsors/su.svg"
                url="https://thesubath.com"
            />
        </div>
    }
}
