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
            section::{
                heading::SectionIcon, page_section::FlashyHomepageSection,
                section_paragraph::FlashyHomepageSectionParagraph,
            },
            sponsors::SponsorsGrid,
            tracks::track_grid::TrackGrid,
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
            .set_href("https://thesubath.com/bcss")
            .expect_throw("Setting location.href");
    });

    html! {
    <>
        <ScrollingNavbar />
        <HeroGradientContainer>
            <div class="w-full px-4 sm:px-6 md:px-8 lg:px-[5%] xl:px-[10%] flex items-center">
                <div class="space-y-4 flex-1 md:mr-10">
                    <img src="img/logo.svg" alt="Bath Hack 24 Logo" class="lg:h-44 md:h-32 h-28" />
                    <h1 class="text-4xl md:text-5xl xl:text-6xl font-hero text-bcss-100 leading-none">
                        { "Bath Hack 2024" }
                    </h1>
                    <p class="text-heroSubtitle text-bcss-100">
                        {"The annual 24-hour hackathon for all Uni of Bath students"}
                    </p>
                    <p class="text-heroSubtitle text-bcss-100 font-bold italic">
                        {"13th - 14th April. Free to enter."}
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
                    {"We'll announce the full series of talks soon. Do you want to give one yourself? "}
                    <a href="mailto:su-bcss@bath.ac.uk" target="_blank" class="underline">
                        {"Get in touch at su-bcss@bath.ac.uk"}
                    </a>
                    {"!"}
                </FlashyHomepageSectionParagraph>
            </FlashyHomepageSection>

            <FlashyHomepageSection icon={SectionIcon::Icon(IconId::FontAwesomeSolidHeart)} title="Sponsors"
                anchor="sponsors">
                <FlashyHomepageSectionParagraph>
                    {"We'd never be able to run Bath Hack without these lovely people! Our sponsors financially support the
                    event and make it possible for us to offer free meals, snacks, merch, and more."}
                </FlashyHomepageSectionParagraph>
                <SponsorsGrid />
            </FlashyHomepageSection>

            <FlashyHomepageSection icon={SectionIcon::Icon(IconId::FontAwesomeSolidQuestion)} title="FAQs" anchor="faqs">
                <FAQs />
            </FlashyHomepageSection>

            <FlashyHomepageSection icon={SectionIcon::Icon(IconId::FontAwesomeSolidUniversalAccess)} title="Accessibility"
                anchor="accessibility">
                <FlashyHomepageSectionParagraph>
                    {"Bath Hack 2024 will take place on-campus in the Chancellor's Building. The entire building will be
                    reserved for the event, with various rooms in use for talks, workshops, etc."}
                </FlashyHomepageSectionParagraph>
                <FlashyHomepageSectionParagraph>
                    {"Lifts, double-width doors, hearing loop support and water fountains are available in the building. If
                you have any specific access requirements that we haven't thought of, please let us know! You can either
                "}
                    <a href="mailto:su-bcss@bath.ac.uk" class="underline">
                        {"get in touch at su-bcss@bath.ac.uk"}
                    </a>
                    {" or set your access requirements on our dashboard once you've made your account."}
                </FlashyHomepageSectionParagraph>
            </FlashyHomepageSection>

            <FlashyHomepageSection icon={SectionIcon::Icon(IconId::FontAwesomeSolidCircleInfo)} title="About BCSS"
                anchor="about">
                <FlashyHomepageSectionParagraph>
                    {"This event has been organised by the Committee of the Bath Computer Science Society. We put lots of
                    effort into organising it each year and aim to make it bigger and better every time."}
                </FlashyHomepageSectionParagraph>
                <FlashyHomepageSectionParagraph>
                    {"BCSS is a member society of the University of Bath Student Union and is also a student chapter of the
                British Computer Society. We are a society for anyone with any interest in Computing, open to any
                student
                studying any degree. Our key aim is to encourage more people into the field, as well as to teach
                important industry skills to help encourage our members to grow."}
                </FlashyHomepageSectionParagraph>
                <Button onclick={on_find_out_more_click.clone()} dark_mode={false} class={classes!("mt-4")}>
                    {"Find out more"}
                </Button>
            </FlashyHomepageSection>
        </div>
        <HomepageFooter />
    </>
    }
}
