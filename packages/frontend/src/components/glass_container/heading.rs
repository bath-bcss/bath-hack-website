use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Html,
}

#[function_component(GlassContainerHeading)]
pub fn glass_container_heading(props: &Props) -> Html {
    html! {
        <h1 class="text-3xl font-hero text-bcss-900 dark:text-bcss-200 mb-4">
            { props.children.clone() }
        </h1>
    }
}
