use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Html,
}

#[function_component(PageControlParagraph)]
pub fn page_control_paragraph(props: &Props) -> Html {
    html! {
    <p class="my-2 text-bcss-900 dark:text-bcss-300">
        {props.children.clone()}
    </p>
    }
}
