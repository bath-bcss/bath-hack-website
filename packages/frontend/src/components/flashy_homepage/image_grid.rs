use yew::prelude::*;

use crate::components::flashy_homepage::image_grid_item::ImageGridItem;

#[function_component(ImageGrid)]
pub fn image_grid() -> Html {
    html! {
    <div class="hidden md:block columns-2 gap-x-8 space-y-4 max-w-sm lg:max-w-md xl:max-w-xl 2xl:max-w-3xl">
        <ImageGridItem src="home_p1.webp" is_portrait={true}  />
        <ImageGridItem src="home_l2.webp" />
        <ImageGridItem src="home_l1.webp" />
        <ImageGridItem src="home_l3.webp" />
        <ImageGridItem src="home_l4.webp" />
        <ImageGridItem src="home_l5.webp" />
        <ImageGridItem src="home_p2.webp" is_portrait={true} />
        <ImageGridItem src="home_l6.webp" />
        <ImageGridItem src="home_l7.webp" />
    </div>
    }
}
