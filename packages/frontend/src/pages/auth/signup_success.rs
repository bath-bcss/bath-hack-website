use yew::prelude::*;

use crate::components::{
    glass_container::{
        heading::GlassContainerHeading, paragraph::GlassContainerParagraph, GlassContainer,
    },
    hero::center::HeroCenterContainer,
};

#[function_component(SignupSuccessPage)]
pub fn signup_success_page() -> Html {
    html! {
    <HeroCenterContainer>
        <GlassContainer>
            <GlassContainerHeading>
                {"Awesome!"}
            </GlassContainerHeading>
            <GlassContainerParagraph>
                {"We've sent an email to your Uni inbox. Please follow the link to continue :)"}
            </GlassContainerParagraph>
            <GlassContainerParagraph>
                {"It may take a while to arrive and it'll probably go to your Junk folder, so look out for it!"}
            </GlassContainerParagraph>
        </GlassContainer>
    </HeroCenterContainer>
    }
}
