use yew::prelude::*;

use crate::components::hero::gradient::HeroGradientContainer;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(HeroCenterContainer)]
pub fn hero_center_container(props: &Props) -> Html {
    html! {
    <HeroGradientContainer>
        <div class="flex flex-col justify-center space-y-4">
            {props.children.to_owned()}
        </div>
    </HeroGradientContainer>
    }
}
