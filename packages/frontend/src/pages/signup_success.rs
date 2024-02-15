use yew::prelude::*;

use crate::components::{glass_container::GlassContainer, hero::HeroHeader};

#[function_component(SignupSuccessPage)]
pub fn signup_success_page() -> Html {
    html! {
    <HeroHeader>
        <GlassContainer>
            <h1 class="text-2xl text-bcss-900">
                {"Awesome!"}
            </h1>
            <p>
                {"We've sent an email to your Uni inbox. Please follow the link to continue :)"}
            </p>
        </GlassContainer>
    </HeroHeader>
    }
}
