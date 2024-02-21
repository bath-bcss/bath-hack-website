use yew::prelude::*;
use yew_icons::IconId;
use yew_router::prelude::*;

use crate::{
    components::{
        button::Button,
        flashy_homepage::{
            footer::HomepageFooter,
            image_grid::ImageGrid,
            section::{FlashyHomepageSection, SectionIcon},
            section_paragraph::FlashyHomepageSectionParagraph,
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
                anchor="welcome">
                <FlashyHomepageSectionParagraph>
                    {"The official BCSS Hackathon is back for 2024 — another exciting year of projects, prizes, and (most
                    importantly) pizza! As per usual, it's open to "}
                    <strong>
                        {"all University of Bath students"}
                    </strong>
                    {"; not just Computer Scientists."}
                </FlashyHomepageSectionParagraph>
                <FlashyHomepageSectionParagraph>
                    {"You'll have exactly one day to build an innovative project with your team and impress the judges. It's
                a fun,
                inclusive, and beginner-friendly atmosphere with a wide range of tracks and talks."}
                </FlashyHomepageSectionParagraph>
                <FlashyHomepageSectionParagraph>
                    {"Are you ready for the most exciting 24 hours of your life?"}
                </FlashyHomepageSectionParagraph>
                <Button onclick={on_sign_up_click.clone()} dark_mode={false} class={classes!("mt-4")}>
                    {"Sign up now!"}
                </Button>
            </FlashyHomepageSection>

            <FlashyHomepageSection icon={SectionIcon::Icon(IconId::FontAwesomeSolidShuffle)} title="Tracks" anchor="tracks"
                child_is_paragraph={true}>
                {"hi"}
            </FlashyHomepageSection>
        </div>

        <HomepageFooter />
    </>
    }
}
