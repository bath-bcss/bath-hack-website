use yew::prelude::*;

use crate::{
    components::{flashy_homepage::section::{
        heading::FlashyHomepageSectionHeading, section_paragraph::FlashyHomepageSectionParagraph,
    }, image::Image},
    data::image_url::get_image_url,
};

use super::heading::SectionIcon;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub icon: SectionIcon,
    pub title: String,
    pub anchor: String,
    #[prop_or_default]
    pub image: Option<String>,

    pub children: Html,

    #[prop_or_default]
    pub child_is_paragraph: bool,
}

#[function_component(FlashyHomepageSection)]
pub fn flashy_homepage_section(props: &Props) -> Html {
    let image_src = use_memo((props.image.clone(),), |(image_src,)| {
        image_src.clone().map(get_image_url)
    });

    html! {
        <div class="max-w-7xl mx-auto px-4 sm:px-6 md:px-8 md:flex">
            <div class="flex-1">
                <FlashyHomepageSectionHeading
                    anchor={props.anchor.clone()}
                    icon={props.icon.clone()}
                >
                    { props.title.clone() }
                </FlashyHomepageSectionHeading>
                if props.child_is_paragraph {
                    <FlashyHomepageSectionParagraph>
                        { props.children.clone() }
                    </FlashyHomepageSectionParagraph>
                } else {
                    { props.children.clone() }
                }
            </div>
            if let Some(image_src) = (*image_src).clone() {
                <div class="md:max-w-[40%] mt-14 md:mt-20 md:ml-8">
                    <Image src={image_src} />
                </div>
            }
        </div>
    }
}
