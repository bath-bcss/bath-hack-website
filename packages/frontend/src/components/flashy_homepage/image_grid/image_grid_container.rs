use yew::prelude::*;

use crate::components::flashy_homepage::image_grid::image_grid_item::ImageGridItem;

#[function_component(ImageGrid)]
pub fn image_grid() -> Html {
    html! {
        <div
            class="hidden md:block columns-2 gap-x-8 space-y-4 max-w-sm lg:max-w-md xl:max-w-xl 2xl:max-w-3xl"
        >
            <ImageGridItem src="societies/bcss.svg" />
            <ImageGridItem src="societies/bias.png" />
            <ImageGridItem src="societies/bvgs.png" />
            <ImageGridItem src="societies/fineart.jpeg" />
            <ImageGridItem src="societies/tabletop.png" />
            <ImageGridItem src="societies/wit.png" />
        </div>
    }
}
