use bhw_types::common_data::EVENT_INSTANCE_NAME;
use gloo_utils::window;
use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_icons::IconId;

use crate::{
    components::{
        button::Button,
        flashy_homepage::{
            faqs::FAQs,
            footer::HomepageFooter,
            image_grid::image_grid_container::ImageGrid,
            image_row::image_row::ImageRow,
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
    data::image_url::get_image_url,
};

#[function_component(HomePage)]
pub fn home_page() -> Html {
    // let navigator = use_navigator().expect_throw("Navigator not found");

    /*let on_sign_up_click = use_callback(navigator.clone(), move |_, _| {
        navigator.push(&Route::Signup);
    });*/

    let on_find_out_more_click = use_callback((), |e: MouseEvent, _| {
        e.prevent_default();
        window()
            .location()
            .set_href("https://bcss.bathcs.com/")
            .expect_throw("Setting location.href");
    });

    html! {
        <>
            <ScrollingNavbar />
            <HeroGradientContainer>
                <div class="w-full px-4 sm:px-6 md:px-8 lg:px-[5%] xl:px-[10%] flex items-center">
                    <div class="space-y-4 flex-1 md:mr-10">
                        <img src={get_image_url("logo.svg".to_string())} class="h-36 xl:h-48 mb-2" />
                        <h1
                            class="text-4xl md:text-5xl xl:text-6xl font-hero text-white dark:text-bcss-100 leading-none"
                        >
                            { EVENT_INSTANCE_NAME }
                        </h1>
                        <p class="text-heroSubtitle text-white dark:text-bcss-100 leading-tight">
                            { "Join 200 engineers, designers, and creators in Bath's ultimate 24-hour challenge. Win incredible prizes, eat free food, and make amazing memories." }
                        </p>
                        <p
                            class="text-heroSubtitle text-white dark:text-bcss-100 font-bold italic"
                        >
                            { "5th — 6th Apr. Free to enter." }
                        </p>
                        <p
                            class="text-3xl text-white dark:text-bcss-100 font-bold py-4 px-6 inline-block bg-white/20 rounded-xl"
                        >
                            { "Sign-ups are now full!" }
                        </p>
                        /*<Button background_is_dark=true onclick={on_sign_up_click.clone()}>
                            { "Sign up!" }
                        </Button>*//*<CountdownTimer
                            time_to={Utc.with_ymd_and_hms(2025, 3, 6, 12, 0, 0).unwrap()}
                            target_name="Sign-ups open in"
                        />*/
                    </div>
                    <ImageGrid />
                </div>
            </HeroGradientContainer>
            <div class="space-y-32 mt-32">
                <FlashyHomepageSection
                    icon={SectionIcon::Emoji("⏰".to_string())}
                    title="The Ultimate 24-Hour Challenge"
                    anchor="welcome"
                    image="sections/intro.webp"
                >
                    <FlashyHomepageSectionParagraph full_width=true>
                        { "The Bath Computer Science Society's official hackathon is back for 2025, and it's bigger than ever. Join us for an entire day (literally) for hacking, learning, building, and sharing fun memories with your friends." }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph full_width=true>
                        { "The goal is simple: build something cool that will impress the judges, and win some of our incredible prizes. We're looking for innovation and creativity, not just sheer complexity. It's super fun, inclusive, and perfect for all levels of ability." }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph full_width=true>
                        { "It's open to " }
                        <strong>{ "all students at Bath" }</strong>
                        { " and we have loads of non-CS participants joining us every year. Are you ready for the best 24 hours of your life?" }
                    </FlashyHomepageSectionParagraph>
                    /*<Button
                        onclick={on_sign_up_click.clone()}
                        background_is_dark=false
                        class={classes!("mt-4")}
                    >
                        { "Sign up now!" }
                    </Button>*/<p
                        class="mt-6 text-3xl text-bcss-950 dark:text-bcss-100 font-bold py-4 px-6 inline-block bg-bcss-100 dark:bg-bcss-800 rounded-xl"
                    >
                        { "Sign-ups are now full!" }
                    </p>
                    /*<CountdownTimer
                        time_to={Utc.with_ymd_and_hms(2025, 3, 6, 12, 0, 0).unwrap()}
                        target_name="Sign-ups open in"
                        light=true
                        class={classes!("mt-4")}
                    />*/
                </FlashyHomepageSection>
                <FlashyHomepageSection
                    icon={SectionIcon::Icon(IconId::FontAwesomeSolidShuffle)}
                    title="Tracks"
                    anchor="tracks"
                >
                    <FlashyHomepageSectionParagraph>
                        { "Prizes are attached to tracks, which give you a guiding aim for your project. You don't have to commit at the start, and your project will be automatically considered for all relevant tracks." }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph>
                        <strong>{ "Our tracks will be released in the coming weeks" }</strong>
                        { " — keep an eye on your emails!" }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph>
                        { "We'll have a range of options based on popular vote, technical ability, and creativity/design." }
                    </FlashyHomepageSectionParagraph>
                    /* <TrackGrid /> */
                </FlashyHomepageSection>
                <FlashyHomepageSection
                    icon={SectionIcon::Icon(IconId::FontAwesomeSolidClock)}
                    title="Schedule"
                    anchor="schedule"
                    image="sections/schedule.webp"
                >
                    <FlashyHomepageSectionParagraph>
                        { "This schedule is indicative so you can plan your availability. We'll release more detail nearer to the time." }
                    </FlashyHomepageSectionParagraph>
                    <Schedule />
                </FlashyHomepageSection>
                <FlashyHomepageSection
                    icon={SectionIcon::Icon(IconId::FontAwesomeSolidHeart)}
                    title="Sponsors"
                    anchor="sponsor"
                >
                    <FlashyHomepageSectionParagraph>
                        { "Our sponsors make the event possible: they fund our food, snacks, prizes, t-shirts, and everything else. We're immensely thankful to them, as Bath Hack would not be able to run without their support! Many of them will be present at our careers fair for some valuable networking." }
                    </FlashyHomepageSectionParagraph>
                    <SponsorsGrid />
                </FlashyHomepageSection>
                <ImageRow />
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
                        { "The event will use the entirety of the Chancellor's Building, with presentations taking place in the ground-floor lecture theatres as well as the other classrooms. Designated spaces can be used for hacking. All rooms used are wheelchair accessible. There are toilets and water fountains on all floors and Committee Members will be available 24/7 during the event." }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph>
                        { "Please " }
                        <a href="mailto:su-bcss@bath.ac.uk" class="underline">
                            { "email su-bcss@bath.ac.uk" }
                        </a>
                        { " if you have any questions about accessibility." }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph>
                        { "You can use either your own laptop/devices, or the computer labs on floors 4 and 5." }
                    </FlashyHomepageSectionParagraph>
                </FlashyHomepageSection>
                <FlashyHomepageSection
                    icon={SectionIcon::Icon(IconId::FontAwesomeSolidCircleInfo)}
                    title="About BCSS"
                    anchor="about"
                >
                    <FlashyHomepageSectionParagraph>
                        { "This event has been organised by the Committee of the Bath Computer Science Society. We put lots of effort into organising it each year and aim to make it bigger and better every time." }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph>
                        { "BCSS is a member society of the University of Bath Student Union and is also a student chapter of the British Computing Society. We are a society for anyone with any interest in Computing, open to any student studying any degree. Our key aim is to encourage more people into the field, as well as to teach important industry skills to help encourage our members to grow." }
                    </FlashyHomepageSectionParagraph>
                    <FlashyHomepageSectionParagraph>
                        { "We are sponsored by" }
                        <a
                            href="https://www.qube-rt.com/"
                            target="_blank"
                            rel="noreferrer"
                            class="mt-3 block"
                        >
                            <img
                                src={get_image_url("sponsors/qrt.svg".to_owned())}
                                class="h-16 w-auto inline-block"
                            />
                        </a>
                    </FlashyHomepageSectionParagraph>
                    <Button
                        onclick={on_find_out_more_click.clone()}
                        background_is_dark=false
                        class={classes!("mt-8")}
                    >
                        { "Find out more" }
                    </Button>
                </FlashyHomepageSection>
            </div>
            <HomepageFooter />
        </>
    }
}
