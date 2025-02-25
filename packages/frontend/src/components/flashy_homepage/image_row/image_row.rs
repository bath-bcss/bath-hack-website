use yew::prelude::*;

use crate::{components::image::Image, data::image_url::get_image_url};

#[function_component(ImageRow)]
pub fn image_row() -> Html {
    html! {
        <div class="flex justify-center w-full space-x-8 px-8 overflow-x-hidden">
            <Image src={get_image_url("row/l1.webp".to_owned())} width="300" />
            <Image src={get_image_url("row/l2.webp".to_owned())} width="300" />
            <Image src={get_image_url("row/l3.webp".to_owned())} width="300" />
            <Image src={get_image_url("row/l5.webp".to_owned())} width="300" />
            <Image src={get_image_url("row/l6.webp".to_owned())} width="300" />
            <Image src={get_image_url("row/l7.webp".to_owned())} width="300" />
        </div>
    }
}
