use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Html,
    #[prop_or_default]
    pub full_width: bool,
}
#[function_component(FlashyHomepageSectionParagraph)]
pub fn flashy_homepage_section_paragraph(props: &Props) -> Html {
    html! {
    <div class={if props.full_width { "w-full" } else { "md:max-w-[70%]" }}>
        <p class="text-bcss-800 dark:text-bcss-300 text-lg mt-4">
            {props.children.clone()}
        </p>
    </div>
    }
}
