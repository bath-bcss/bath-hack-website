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
            section::{
                heading::SectionIcon, page_section::FlashyHomepageSection,
                section_paragraph::FlashyHomepageSectionParagraph,
            },
            tracks::track_grid::TrackGrid,
        },
        hero::gradient::HeroGradientContainer,
        nav::ScrollingNavbar,
    },
    router::Route,
};

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let navigator = use_navigator().unwrap();

    let on_sign_up_click = use_callback(navigator.clone(), move |_e, _| {
        navigator.push(&Route::Signup);
    });

    html! {
    <>
        <ScrollingNavbar />
        <HeroGradientContainer>
            <div class="w-full px-4 sm:px-6 md:px-8 lg:px-[10%] flex items-center">
                <div class="space-y-4 flex-1 md:mr-10">
                    <h1 class="text-hero font-hero text-bcss-100 leading-none">
                        { "Bath Hack 2024" }
                    </h1>
                    <p class="text-heroSubtitle text-bcss-100">
                        {"The annual 24-hour hackathon for all Uni of Bath students"}
                    </p>
                    <p class="text-heroSubtitle text-bcss-100 font-bold italic">
                        {"13th - 14th April. £0."}
                    </p>
                    <Button dark_mode={true} onclick={on_sign_up_click.clone()}>
                        {"Sign up!"}
                    </Button>
                </div>
                <ImageGrid />
            </div>
        </HeroGradientContainer>

        <div class="space-y-32 mt-32">
            <FlashyHomepageSection icon={SectionIcon::Emoji("👋".to_string())} title="Welcome to Bath Hack!"
                anchor="welcome" image="home_section1.webp">
                <FlashyHomepageSectionParagraph full_width={true}>
                    {"The official BCSS Hackathon is back for 2024 — another exciting year of projects, prizes, and (most
                    importantly) pizza! As per usual, it's open to "}
                    <strong>
                        {"all University of Bath students"}
                    </strong>
                    {"; not just Computer Scientists."}
                </FlashyHomepageSectionParagraph>
                <FlashyHomepageSectionParagraph full_width={true}>
                    {"You'll have exactly one day to build an innovative project with your team and impress the judges. It's
                a fun,
                inclusive, and beginner-friendly atmosphere with a wide range of tracks and talks."}
                </FlashyHomepageSectionParagraph>
                <FlashyHomepageSectionParagraph full_width={true}>
                    {"Are you ready for the most exciting 24 hours of your life?"}
                </FlashyHomepageSectionParagraph>
                <Button onclick={on_sign_up_click.clone()} dark_mode={false} class={classes!("mt-4")}>
                    {"Sign up now!"}
                </Button>
            </FlashyHomepageSection>

            <FlashyHomepageSection icon={SectionIcon::Icon(IconId::FontAwesomeSolidShuffle)} title="Tracks" anchor="tracks">
                <FlashyHomepageSectionParagraph>
                    {"The competition's split into \"tracks\", to give your project ideas a general guiding theme. Don't
                worry — you won't need to choose a track at the start, as your entry will automatically be considered
                for all of them. So even if your project might fall into the scope of multiple tracks, you won't need to
                pick which you run for."}
                </FlashyHomepageSectionParagraph>
                <TrackGrid />
            </FlashyHomepageSection>

            <FlashyHomepageSection icon={SectionIcon::Icon(IconId::FontAwesomeSolidComments)} title="Talks" anchor="talks"
                image="home_section3.webp">
                <FlashyHomepageSectionParagraph full_width={true}>
                    {"Bath Hack is about more than just coding: come along to our interesting series of talks from various
                    industry experts and University academics."}
                </FlashyHomepageSectionParagraph>
                <FlashyHomepageSectionParagraph full_width={true}>
                    {"Some might provide inspiration for your projects (or even your career), but others will be more
                    light-hearted, providing a fun break from the work."}
                </FlashyHomepageSectionParagraph>
                <FlashyHomepageSectionParagraph full_width={true}>
                    {"We'll announce the full series of talks soon. Do you want to give one yourself? Get in touch at "}
                    <a href="mailto:su-bcss@bath.ac.uk" target="_blank" class="underline">
                        {"su-bcss@bath.ac.uk"}
                    </a>
                    {"!"}
                </FlashyHomepageSectionParagraph>
            </FlashyHomepageSection>

            <FlashyHomepageSection icon={SectionIcon::Icon(IconId::FontAwesomeSolidQuestion)} title="FAQs" anchor="faqs">
                <FAQs />
            </FlashyHomepageSection>
        </div>
        <HomepageFooter />
    </>
    }
}
