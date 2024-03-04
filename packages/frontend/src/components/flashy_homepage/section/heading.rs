use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(PartialEq, Clone)]
pub enum SectionIcon {
    Icon(IconId),
    Emoji(String),
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Html,
    pub anchor: String,
    pub icon: SectionIcon,
}

#[function_component(FlashyHomepageSectionHeading)]
pub fn flashy_homepage_section_heading(props: &Props) -> Html {
    let icon_component = use_memo((props.icon.clone(),), |(icon,)| match icon {
        SectionIcon::Icon(id) => html! {
        <Icon icon_id={id.clone()} class={classes!("text-bcss-800", "dark:text-bcss-300" )} width="48" height="48"
            role="presentation" />
        },
        SectionIcon::Emoji(emoji) => html! {
        <p class="text-6xl" role="presentation">{emoji.clone()}</p>
        },
    });

    html! {
    <div class="w-full relative">
        <p class="invisible absolute -top-28" id={props.anchor.clone()}></p>
        {(*icon_component).clone()}
        <h1 class="text-4xl tracking-tight text-bcss-900 dark:text-bcss-200 font-bold mt-6">
            {props.children.clone()}
        </h1>
    </div>
    }
}
