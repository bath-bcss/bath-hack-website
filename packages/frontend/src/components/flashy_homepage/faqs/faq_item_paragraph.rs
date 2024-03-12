use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Html,
}

#[function_component(FAQItemParagraph)]
pub fn faq_item_paragraph(props: &Props) -> Html {
    html! { <p class="text-bcss-900 dark:text-bcss-200">{ props.children.clone() }</p> }
}
