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
            tracks::TrackGrid,
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
                        <h1
                            class="text-3xl sm:text-5xl md:text-6xl xl:text-7xl font-hero text-white dark:text-bcss-100 leading-none"
                        >
                            { "WiTathon 2025" }
                        </h1>
                        <p class="text-heroSubtitle text-white dark:text-bcss-100">
                            { "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat."}
                        </p>
                        <p
                            class="text-heroSubtitle text-white dark:text-bcss-100 font-bold italic"
                        >
                            { "XXth — XXth Mar. Free to enter." }
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
                    title="Game Jam: New for 2024!"
                    anchor="welcome"
                    image="home_section_intro.webp"
                >
                    <FlashyHomepageSectionParagraph full_width=true>
                        { "BCSS' brand new competition is launching in 2024. We've joined forces with 4 societies to bring you a week-long game jam. Get together in groups of 4 to build your very own game and win prizes!" }
                        { " You don't need any coding knowledge: even physical board games are eligible for entry." }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph full_width=true>
                        { "You'll have just over 7 days to build an innovative project with your team and impress the judges.
                It's a fun, inclusive, and beginner-friendly atmosphere with a range of ways to compete." }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph full_width=true>
                        { "The event starts with a fun kick-off and team-building session, and ends with an exciting showcase you can bring all your friends to (free pizza included)." }
                        { " You'll build your projects to a " }
                        <strong>{ "mystery theme" }</strong>
                        { " that we'll reveal at the kick-off." }
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
                        { "The competition is split into “tracks”, to give your project ideas a general guiding theme. Don't
                worry — you won't need to choose a track at the start, as your entry will automatically be considered
                for all of them. So even if your project might fall into the scope of multiple tracks, you won't need to
                pick which you run for." }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph>
                        { "Prizes shown are per member of your group (for groups with 4 members) and are subject to change
                    before the event." }
                    </FlashyHomepageSectionParagraph>
                    <TrackGrid />
                </FlashyHomepageSection>
                <FlashyHomepageSection
                    icon={SectionIcon::Icon(IconId::FontAwesomeSolidClock)}
                    title="Schedule"
                    anchor="schedule"
                    image="home_section_schedule.webp"
                >
                    <Schedule />
                </FlashyHomepageSection>
                <FlashyHomepageSection
                    icon={SectionIcon::Icon(IconId::FontAwesomeSolidHeart)}
                    title="Sponsored by DucksBath"
                    anchor="sponsor"
                    image="sponsors/ducksbath.webp"
                >
                    <FlashyHomepageSectionParagraph>
                        { "DucksBath are kindly sponsoring Game Jam with a range of merch that you'll be able to find around our events. Keep your eyes peeled for their cool duck-themed socks and caps! You can visit them in The Market on campus for the whole range of products." }
                    </FlashyHomepageSectionParagraph>
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
                        { "Game Jam 2024 will take place across two events on-campus. We'll be using exclusively wheelchair-accessible rooms." }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph>
                        { "Lifts, double-width doors, hearing loop support and water fountains are available for all involved rooms. If
                you have any specific access requirements that we haven't thought of, please let us know! You can either
                " }
                        <a href="mailto:su-bcss@bath.ac.uk" class="underline">
                            { "get in touch at su-bcss@bath.ac.uk" }
                        </a>
                        { " or set your access requirements on our dashboard once you've made your account." }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph>
                        { "You can also " }
                        <a
                            href="https://www.bath.ac.uk/publications/campus-accessibility-map/attachments/university-campus-accessibility-map.pdf"
                            target="_blank"
                            class="underline"
                        >
                            { "download the Campus Accessibility map" }
                        </a>
                        { "." }
                    </FlashyHomepageSectionParagraph>
                </FlashyHomepageSection>
                <FlashyHomepageSection
                    icon={SectionIcon::Icon(IconId::FontAwesomeSolidCircleInfo)}
                    title="About BCSS"
                    anchor="about"
                >
                    <FlashyHomepageSectionParagraph>
                        { "This event has been organised by the Committee of the Bath Computer Science Society, in collaboration with Bath Illustration and Animation Society, Bath Video Game Society, Bath Fine Art Society, Tabletop Gaming Society, and Women in Technology. It's our first time running this exciting collaboration, and we hope to continue it in future years!" }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph>
                        { "BCSS is a member society of the University of Bath Student Union and is also a student chapter of the
                British Computer Society. We are a society for anyone with any interest in Computing, open to any
                student
                studying any degree. Our key aim is to encourage more people into the field, as well as to teach
                important industry skills to help encourage our members to grow." }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph>
                        { "Our society is sponsored by:" }
                        <a href="https://www.qube-rt.com/" target="_blank" rel="noreferrer">
                            <img src="/img/sponsors/qrt.webp" class="w-36 mt-2" alt="QRT" />
                        </a>
                    </FlashyHomepageSectionParagraph>
                    <Button
                        onclick={on_find_out_more_click.clone()}
                        background_is_dark=false
                        class={classes!("mt-4")}
                    >
                        { "Find out more" }
                    </Button>
                </FlashyHomepageSection>
            </div>
            <HomepageFooter />
        </>
    }
}
