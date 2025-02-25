use yew::prelude::*;

use crate::components::flashy_homepage::image_grid::image_grid_item::ImageGridItem;

#[function_component(ImageGrid)]
pub fn image_grid() -> Html {
    html! {
        <div
            class="hidden md:block columns-2 gap-x-8 space-y-4 max-w-sm lg:max-w-md xl:max-w-xl 2xl:max-w-3xl"
        >
            <ImageGridItem src="hero/p1.webp" />
            <ImageGridItem src="hero/l8.webp" />
            <ImageGridItem src="hero/p2.webp" />
            <ImageGridItem src="hero/p6.webp" />
            <ImageGridItem src="hero/l3.webp" />
            <ImageGridItem src="hero/p7.webp" />
            <ImageGridItem src="hero/l7.webp" />
            <ImageGridItem src="hero/p3.webp" />
        </div>
    }
}
