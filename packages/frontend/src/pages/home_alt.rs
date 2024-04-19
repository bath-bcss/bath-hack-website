use yew::prelude::*;

use crate::components::hero::gradient::HeroGradientContainer;

#[function_component(HomePageAlt)]
pub fn home_page_alt() -> Html {
    html! {
        <HeroGradientContainer>
            <div class="w-full px-4 sm:px-6 md:px-8 lg:px-[5%] xl:px-[10%] flex items-center">
                <div class="space-y-4 flex-1 md:mr-10">
                    <img src="img/logo.svg" role="presentation" class="lg:h-48 md:h-32 h-24" />
                    <h1
                        class="text-3xl sm:text-5xl md:text-6xl xl:text-7xl font-hero text-bcss-100 leading-none"
                    >
                        { "Thanks for participating!" }
                    </h1>
                    <p class="text-heroSubtitle text-bcss-100">
                        { "We hope you had a great time at Bath Hack 2024." }
                    </p>
                    <p class="text-heroSubtitle text-bcss-100">
                        { "Please check your emails for our feedback form and photo gallery." }
                    </p>
                </div>
            </div>
        </HeroGradientContainer>
    }
}
