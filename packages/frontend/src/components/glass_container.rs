use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(GlassContainer)]
pub fn glass_container(props: &Props) -> Html {
    html! {
    <div class="p-12 bg-white/90 dark:bg-bcss-900/90 rounded-xl backdrop-blur drop-shadow-lg">
        {props.children.clone()}
    </div>
    }
}
