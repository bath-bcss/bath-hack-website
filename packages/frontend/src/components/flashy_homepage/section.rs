use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::flashy_homepage::section_paragraph::FlashyHomepageSectionParagraph;

#[derive(PartialEq, Clone)]
pub enum SectionIcon {
    Icon(IconId),
    Emoji(String),
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub icon: SectionIcon,
    pub title: String,
    pub anchor: String,

    pub children: Html,

    #[prop_or_default]
    pub child_is_paragraph: bool,
}

#[function_component(FlashyHomepageSection)]
pub fn flashy_homepage_section(props: &Props) -> Html {
    let icon_component = use_memo((props.icon.clone(),), |(icon,)| match icon {
        SectionIcon::Icon(id) => html! {
        <Icon icon_id={id.clone()} class={classes!("text-bcss-800")} width="48" height="48" />
        },
        SectionIcon::Emoji(emoji) => html! {
        <p class="text-6xl">{emoji.clone()}</p>
        },
    });

    html! {
    <div class="max-w-7xl mx-auto px-4 sm:px-6 md:px-8">
        {(*icon_component).clone()}
        <h1 class="text-4xl tracking-tight text-bcss-900 dark:text-bcss-200 font-bold mt-6" id={props.anchor.clone()}>
            {props.title.clone()}
        </h1>

        if props.child_is_paragraph {
        <FlashyHomepageSectionParagraph>
            {props.children.clone()}
        </FlashyHomepageSectionParagraph>
        } else {
        {props.children.clone()}
        }
    </div>
    }
}
