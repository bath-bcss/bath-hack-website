use yew::prelude::*;

use crate::components::flashy_homepage::image_grid_item::ImageGridItem;

#[function_component(ImageGrid)]
pub fn image_grid() -> Html {
    html! {
    <div class="hidden md:block columns-2 gap-x-8 space-y-4 max-w-sm lg:max-w-md xl:max-w-xl 2xl:max-w-3xl">
        <ImageGridItem src="home_p1.jpg" />
        <ImageGridItem src="home_l2.jpg" />
        <ImageGridItem src="home_l1.jpg" />
        <ImageGridItem src="home_l3.jpg" />
        <ImageGridItem src="home_p2.jpg" />
        <ImageGridItem src="home_l4.jpg" />
    </div>
    }
}
