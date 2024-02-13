use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(HeroHeader)]
pub fn hero_header(props: &Props) -> Html {
    html! {
    <div class="h-screen w-full bg-bcss-300 flex justify-center items-center p-8">
        <div>
            { props.children.clone() }
        </div>
    </div>
    }
}
