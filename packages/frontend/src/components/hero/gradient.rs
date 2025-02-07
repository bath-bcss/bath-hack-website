use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(HeroGradientContainer)]
pub fn hero_gradient_container(props: &Props) -> Html {
    html! {
        <div
            class="h-screen w-full bg-gradient-to-br from-green-500 dark:from-green-800 to-bcss-300 dark:to-bcss-800 flex justify-center items-center p-2 md:p-4 lg:p-8 overflow-hidden"
        >
            { props.children.clone() }
        </div>
    }
}
