use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::{
        button::Button,
        flashy_homepage::section::{FlashyHomepageSection, SectionIcon},
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
                <div class="space-y-4 flex-1">
                    <h1 class="text-hero font-hero text-bcss-100 leading-none">
                        { "Bath Hack 2024" }
                    </h1>
                    <p class="text-heroSubtitle text-bcss-100">
                        {"The annual 24-hour hackathon for all Uni of Bath students"}
                    </p>
                    <p class="text-heroSubtitle text-bcss-100 font-bold italic">
                        {"13th - 14th April"}
                    </p>
                    <Button dark_mode={true} onclick={on_sign_up_click}>
                        {"Sign up!"}
                    </Button>
                </div>
                <img src="img/group.jpg" class="max-w-3xl rounded-2xl shadow-2xl shadow-bcss-400 dark:shadow-bcss-700 dark:brightness-90" />
            </div>
        </HeroGradientContainer>

        <div class="space-y-32 mt-32">
            <FlashyHomepageSection icon={SectionIcon::Emoji("👋".to_string())} title="Welcome to Bath Hack!"
                anchor="welcome" child_is_paragraph={true}>
                {"Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem
            pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud
            nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia
            pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem
            duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt
            duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris
            sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis."}
            </FlashyHomepageSection>
        </div>
    </>
    }
}
