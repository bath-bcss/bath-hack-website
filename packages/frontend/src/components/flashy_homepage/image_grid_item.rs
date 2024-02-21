use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub src: String,
}

#[function_component(ImageGridItem)]
pub fn image_grid_item(props: &Props) -> Html {
    let src = use_memo((props.src.clone(),), |(src,)| {
        let mut src = (*src).clone();
        let using_cdn = env!("BHW_FRONTEND_USE_NETLIFY_IMAGE_CDN") == "true";

        if using_cdn {
            src.insert_str(0, "/home-page-imgs/")
        } else {
            src.insert_str(0, "/img/");
        }
        src
    });

    html! {
    <img src={(*src).clone()}
        class="w-full h-auto rounded-2xl shadow-2xl shadow-bcss-400 dark:shadow-bcss-700 dark:brightness-90" loading="lazy"
        width="400" />
    }
}
