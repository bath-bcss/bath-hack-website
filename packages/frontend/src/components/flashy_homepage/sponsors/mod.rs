use yew::prelude::*;

use crate::components::flashy_homepage::{
    section::section_paragraph::FlashyHomepageSectionParagraph, sponsors::sponsor_item::SponsorItem,
};

mod sponsor_item;

#[function_component(SponsorsGrid)]
pub fn sponsors_grid() -> Html {
    html! {
        <>
            <div
                class="mt-8 mb-8 grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 auto-rows-fr gap-4 sm:gap-x-8 sm:gap-y-4 items-center"
                role="list"
            >
                <SponsorItem
                    name="AWS"
                    logo_url="img/sponsors/aws.svg"
                    url="https://aws.amazon.com"
                />
                <SponsorItem
                    name="GitHub"
                    logo_url="img/sponsors/github.png"
                    url="https://github.com"
                />
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
                <SponsorItem
                    name="XMOS"
                    logo_url="img/sponsors/xmos.webp"
                    url="https://www.xmos.com/"
                />
                <SponsorItem
                    name="Telekinetix"
                    logo_url="img/sponsors/telekinetix.webp"
                    url="https://www.telekinetix.com/"
                />
                <SponsorItem
                    name="Tutor Hunt"
                    logo_url="img/sponsors/tutorhunt.png"
                    url="https://www.tutorhunt.com/"
                />
                <SponsorItem
                    name="CiteAb"
                    logo_url="img/sponsors/citeab.webp"
                    url="https://www.citeab.com/"
                />
                <SponsorItem
                    name="Dorset Software"
                    logo_url="img/sponsors/dorset.jpg"
                    url="https://www.dorsetsoftware.com/Home"
                />
                <SponsorItem
                    name="Elanco"
                    logo_url="img/sponsors/elanco.svg"
                    url="https://www.elanco.com/en-gb"
                />
            </div>
            <FlashyHomepageSectionParagraph>
                { "The following non-sponsor organisations are also generously supporting the event:" }
            </FlashyHomepageSectionParagraph>
            <div
                class="mt-8 grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 auto-rows-fr gap-4 sm:gap-x-8 sm:gap-y-4 items-center"
                role="list"
            >
                <SponsorItem name="AMD" logo_url="img/sponsors/amd.png" url="https://amd.com" />
                <SponsorItem
                    name="Department of Computer Science"
                    logo_url="img/sponsors/uni.svg"
                    url="https://www.bath.ac.uk/departments/department-of-computer-science/"
                />
            </div>
        </>
    }
}
