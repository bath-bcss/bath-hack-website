use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::{button::Button, hero::HeroHeader},
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
        <HeroHeader>
            <h1 class="text-center text-hero font-hero text-bcss-100 leading-none">
                { "Bath Hack 2024" }
            </h1>
            <p class="text-center text-heroSubtitle text-bcss-100">
                {"The annual 24-hour hackathon for all Uni of Bath students"}
            </p>
            <p class="text-center text-heroSubtitle text-bcss-100 font-bold italic">
                {"13th - 14th April"}
            </p>
            <div class="flex justify-center">
                <Button dark_mode={true} onclick={on_sign_up_click}>
                    {"Sign up!"}
                </Button>
            </div>
        </HeroHeader>
        <h1>{"hi"}</h1>
    </>
    }
}
