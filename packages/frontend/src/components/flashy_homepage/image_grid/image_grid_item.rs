use yew::prelude::*;

use crate::data::image_url::get_image_url;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub src: String,
    #[prop_or_default]
    pub is_portrait: bool,
}

#[function_component(ImageGridItem)]
pub fn image_grid_item(props: &Props) -> Html {
    let src = use_memo((props.src.clone(),), |(src,)| get_image_url(src.clone()));

    let (width, height) = if props.is_portrait {
        (400, 650)
    } else {
        (650, 400)
    };

    html! {
        <img
            src={(*src).clone()}
            class="w-full h-auto rounded-2xl shadow-2xl shadow-bcss-400 dark:shadow-bcss-700 dark:brightness-90 bg-bcss-400"
            loading="lazy"
            width={width.to_string()}
            height={height.to_string()}
            role="presentation"
        />
    }
}
