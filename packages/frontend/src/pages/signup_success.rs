use yew::prelude::*;

use crate::components::{glass_container::GlassContainer, hero::center::HeroCenterContainer};

#[function_component(SignupSuccessPage)]
pub fn signup_success_page() -> Html {
    html! {
    <HeroCenterContainer>
        <GlassContainer>
            <h1 class="text-2xl text-bcss-900 dark:text-bcss-100">
                {"Awesome!"}
            </h1>
            <p class="dark:text-bcss-200">
                {"We've sent an email to your Uni inbox. Please follow the link to continue :)"}
            </p>
            <p class="dark:text-bcss-200">
                {"It may take a while to arrive and it'll probably go to your Junk folder, so look out for it!"}
            </p>
        </GlassContainer>
    </HeroCenterContainer>
    }
}
