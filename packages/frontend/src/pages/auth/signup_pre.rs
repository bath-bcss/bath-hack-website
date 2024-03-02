use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    components::{
        button::Button,
        glass_container::{
            heading::GlassContainerHeading, paragraph::GlassContainerParagraph, GlassContainer,
        },
        hero::center::HeroCenterContainer,
    },
    router::Route,
};

#[function_component(SignupPrePage)]
pub fn signup_pre_page() -> Html {
    let navigator = use_navigator().expect_throw("Navigator not found");
    let on_agree_click = use_callback((), move |_, _| {
        navigator.push(&Route::Signup);
    });

    html! {
    <HeroCenterContainer>
        <GlassContainer home_link={true}>
            <GlassContainerHeading>
                {"Sign up to Bath Hack 24!"}
            </GlassContainerHeading>
            <GlassContainerParagraph>
                {"We're so excited that you're considering joining our event!"}
            </GlassContainerParagraph>
            <GlassContainerParagraph top_margin={true}>
                {"First, the legal bits. We will process your personal information under The SU Bath's "}
                <a href="https://www.thesubath.com/pageassets/privacy/Privacy-Policy-2022.pdf" target="_blank"
                    class="underline">
                    {"Privacy Policy"}
                </a>
                {". BCSS Committee members will have access to the information you provide on a legitimate interest basis,
                as they need the information to run the event."}
            </GlassContainerParagraph>
            <GlassContainerParagraph>
                {"To access your rights under the Data Protection Act 2018 and (where relevant) the General Data Protection
                Act, please email su-bcss@bath.ac.uk from your University email account."}
            </GlassContainerParagraph>

            <GlassContainerParagraph top_margin={true}>
                {"By pressing the button below, you accept the above points and agree to allow BCSS to process the personal
                information you provide."}
            </GlassContainerParagraph>
            <Button dark_mode={false} class={classes!("mt-4")} onclick={on_agree_click}>
                {"I agree"}
            </Button>
        </GlassContainer>
    </HeroCenterContainer>
    }
}
