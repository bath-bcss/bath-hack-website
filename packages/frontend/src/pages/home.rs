use gloo_utils::window;
use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_icons::IconId;
use yew_router::prelude::*;

use crate::{
    components::{
        button::Button,
        flashy_homepage::{
            faqs::FAQs,
            footer::HomepageFooter,
            image_grid::image_grid_container::ImageGrid,
            schedule::Schedule,
            section::{
                heading::SectionIcon, page_section::FlashyHomepageSection,
                section_paragraph::FlashyHomepageSectionParagraph,
            },
            sponsors::SponsorsGrid,
        },
        hero::gradient::HeroGradientContainer,
        nav::ScrollingNavbar,
    },
    router::Route,
};

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let navigator = use_navigator().expect_throw("Navigator not found");

    let on_sign_up_click = use_callback(navigator.clone(), move |_, _| {
        navigator.push(&Route::Signup);
    });

    let on_find_out_more_click = use_callback((), |e: MouseEvent, _| {
        e.prevent_default();
        window()
            .location()
            .set_href("https://www.instagram.com/bath.wit/")
            .expect_throw("Setting location.href");
    });

    html! {
        <>
            <ScrollingNavbar />
            <HeroGradientContainer>
                <div class="w-full px-4 sm:px-6 md:px-8 lg:px-[5%] xl:px-[10%] flex items-center">
                    <div class="space-y-4 flex-1 md:mr-10">
                        <h1
                            class="text-3xl sm:text-5xl md:text-6xl xl:text-7xl font-hero text-white dark:text-bcss-100 leading-none"
                        >
                            { "WiTathon 2025 — Reinvent the Wheel" }
                        </h1>
                        <p class="text-heroSubtitle text-white dark:text-bcss-100">
                            { "Take the chance to evaluate problems from a new angle. Develop unique solutions. Play to your strengths from graphic design to hardware development." }
                        </p>
                        <p
                            class="text-heroSubtitle text-white dark:text-bcss-100 font-bold italic"
                        >
                            { "8th — 9th Mar. Free to enter." }
                        </p>
                        <Button background_is_dark=true onclick={on_sign_up_click.clone()}>
                            { "Sign up!" }
                        </Button>
                    </div>
                    <ImageGrid />
                </div>
            </HeroGradientContainer>
            <div class="space-y-32 mt-32">
                <FlashyHomepageSection
                    icon={SectionIcon::Emoji("👋".to_string())}
                    title="A hackathon made for women by women"
                    anchor="welcome"
                    image="sections/intro.webp"
                >
                    <FlashyHomepageSectionParagraph full_width=true>
                        { "Women in Technology (WiT) aims to create an environment where women and gender minorities can support each other and share our love and passion for technology." }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph full_width=true>
                        { "The WiTathon is an annual event that aims to allow time for us to get together and work on personal projects that lie beyond typical university projects. The WiTathon is ran completely by you; you can choose to focus on wherever your strengths lie, no matter if that’s in Python or C++ or if it’s in business and marketing." }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph full_width=true>
                        { "This year's theme is " }
                        <strong>{ "Reinvent the Wheel" }</strong>
                        { ". We want you to take something boring and outdated and improve it. We don’t believe in \"if it isn't broken, don't fix it\". Everything has room for improvement." }
                    </FlashyHomepageSectionParagraph>
                    <Button
                        onclick={on_sign_up_click.clone()}
                        background_is_dark=false
                        class={classes!("mt-4")}
                    >
                        { "Sign up now!" }
                    </Button>
                </FlashyHomepageSection>
                <FlashyHomepageSection
                    icon={SectionIcon::Icon(IconId::FontAwesomeSolidShuffle)}
                    title="Tracks"
                    anchor="tracks"
                >
                    <FlashyHomepageSectionParagraph>
                        { "A hackathon track is a category that has a specific prize you can choose to focus your project on for a chance of winning. All projects will be considered for all tracks." }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph>
                        { "Our tracks will be released closer to the event date." }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph>
                        { "There will be tracks based on technical ability, User graphics and entrepreneurial approach. This is a hacking competition but we encourage students of all technical ability to attend." }
                    </FlashyHomepageSectionParagraph>
                    /* <TrackGrid /> */
                </FlashyHomepageSection>
                <FlashyHomepageSection
                    icon={SectionIcon::Icon(IconId::FontAwesomeSolidClock)}
                    title="Schedule"
                    anchor="schedule"
                    image="sections/schedule.webp"
                >
                    <Schedule />
                </FlashyHomepageSection>
                <FlashyHomepageSection
                    icon={SectionIcon::Icon(IconId::FontAwesomeSolidHeart)}
                    title="Sponsors"
                    anchor="sponsor"
                >
                    <FlashyHomepageSectionParagraph>
                        { "We are so grateful to every sponsor of this event. We would not be able to host WiTathon 2025 without their continued support." }
                    </FlashyHomepageSectionParagraph>
                    <SponsorsGrid />
                </FlashyHomepageSection>
                <FlashyHomepageSection
                    icon={SectionIcon::Icon(IconId::FontAwesomeSolidQuestion)}
                    title="FAQs"
                    anchor="faqs"
                >
                    <FAQs />
                </FlashyHomepageSection>
                <FlashyHomepageSection
                    icon={SectionIcon::Icon(IconId::FontAwesomeSolidUniversalAccess)}
                    title="Accessibility"
                    anchor="accessibility"
                >
                    <FlashyHomepageSectionParagraph>
                        { "The event will all take place on the ground floor of 1 West, no stairs for access. All rooms used are wheelchair accessible. There will be toilets and a water fountain nearby and committee members available 24/7 during the event." }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph>
                        { "Please " }
                        <a href="mailto:witbath.ac.uk" class="underline">
                            { "email wit@bath.ac.uk" }
                        </a>
                        { " if you have any accessibility requirements." }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph>
                        { "Please bring your own laptops and any other devices you will need." }
                    </FlashyHomepageSectionParagraph>
                </FlashyHomepageSection>
                <FlashyHomepageSection
                    icon={SectionIcon::Icon(IconId::FontAwesomeSolidCircleInfo)}
                    title="About WiT"
                    anchor="about"
                >
                    <FlashyHomepageSectionParagraph>
                        { "Women in Technology (WiT) aims to create an environment where women and gender minorities can support each other and share our love and passion for technology. We want to ensure everyone interested in learning about and entering the technology industry feels confident and safe to do so." }
                    </FlashyHomepageSectionParagraph>
                    <Button
                        onclick={on_find_out_more_click.clone()}
                        background_is_dark=false
                        class={classes!("mt-4")}
                    >
                        { "Visit our website" }
                    </Button>
                </FlashyHomepageSection>
            </div>
            <HomepageFooter />
        </>
    }
}
