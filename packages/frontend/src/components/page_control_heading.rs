use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Html,
}

#[function_component(PageControlHeading)]
pub fn page_control_heading(props: &Props) -> Html {
    html! {
    <h2 class="mt-4 text-lg font-bold text-bcss-800 dark:text-bcss-300">
        {props.children.clone()}
    </h2>
    }
}
