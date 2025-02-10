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
                <GlassContainerHeading>{ "Awesome!" }</GlassContainerHeading>
                <GlassContainerParagraph>
                    { "We've sent an email to your Uni inbox. Please follow the link in it to continue :)" }
                </GlassContainerParagraph>
                <GlassContainerParagraph top_margin=true>
                    <strong>{ "Our emails can take a long time (5-10 minutes) to arrive." }</strong>
                    { " Please be patient, and " }
                    <strong>{ "check your Junk folder" }</strong>
                    { " too. If it's still not working, " }
                    <a href="mailto:su-bcss@bath.ac.uk" class="underline">
                        { "please contact us (su-bcss@bath.ac.uk)" }
                    </a>
                    { " from your Uni email and we'll activate your account manually." }
                </GlassContainerParagraph>
                <GlassContainerParagraph top_margin=true>
                    { "Your activation link is valid for 30 minutes. After this time, please restart the sign-up process." }
                </GlassContainerParagraph>
            </GlassContainer>
        </HeroCenterContainer>
    }
}
