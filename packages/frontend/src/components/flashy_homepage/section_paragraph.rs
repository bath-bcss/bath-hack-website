use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Html,
}
#[function_component(FlashyHomepageSectionParagraph)]
pub fn flashy_homepage_section_paragraph(props: &Props) -> Html {
    html! {
    <p class="text-bcss-700 dark:text-bcss-300 text-lg mt-4">
        {props.children.clone()}
    </p>
    }
}
