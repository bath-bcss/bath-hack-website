use yew::prelude::*;

use crate::data::image_url::get_image_url;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub src: String,
    pub href: String,
}

#[function_component(ImageGridItem)]
pub fn image_grid_item(props: &Props) -> Html {
    let src = use_memo((props.src.clone(),), |(src,)| get_image_url(src.clone()));

    html! {
        <p
            class="h-auto w-[250px] rounded-2xl overflow-hidden dark:brightness-90 bg-bcss-400 dark:bg-bcss-800"
        >
            <a href={props.href.clone()} target="_blank" rel="noreferrer">
                <img src={(*src).clone()} loading="lazy" role="presentation" />
            </a>
        </p>
    }
}
