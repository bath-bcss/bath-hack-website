use yew::prelude::*;

use crate::components::flashy_homepage::image_grid::image_grid_item::ImageGridItem;

#[function_component(ImageGrid)]
pub fn image_grid() -> Html {
    html! {
        <div
            class="hidden md:grid grid-cols-2 gap-x-8 gap-y-4 max-w-sm lg:max-w-md xl:max-w-xl 2xl:max-w-3xl"
        >
            <ImageGridItem src="societies/bcss.svg" href="https://thesubath.com/bcss" />
            <ImageGridItem src="societies/bias.png" href="https://thesubath.com/bias" />
            <ImageGridItem src="societies/bvgs.png" href="https://thesubath.com/bvgs" />
            <ImageGridItem src="societies/fineart.jpeg" href="https://thesubath.com/fineart" />
            <ImageGridItem src="societies/tabletop.png" href="https://thesubath.com/tabletop" />
            <ImageGridItem src="societies/wit.png" href="https://linktr.ee/wit.bath" />
        </div>
    }
}
