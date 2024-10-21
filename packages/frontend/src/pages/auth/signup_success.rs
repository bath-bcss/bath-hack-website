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
                    { "We've sent an email to your Uni inbox. Please follow the link to continue :)" }
                </GlassContainerParagraph>
                <GlassContainerParagraph top_margin=true>
                    <strong>
                        { "Our emails are taking a long time (5-10 minutes) to arrive right now." }
                    </strong>
                    { " Please be patient, and " }
                    <strong>{ "check your Junk folder" }</strong>
                    { " too. If it's still not working, please contact us." }
                </GlassContainerParagraph>
                <GlassContainerParagraph top_margin=true>
                    { "We are currently working to resolve this issue, but for the meantime please wait patiently for your email to arrive." }
                </GlassContainerParagraph>
            </GlassContainer>
        </HeroCenterContainer>
    }
}
