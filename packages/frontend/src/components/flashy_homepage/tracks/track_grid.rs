use yew::prelude::*;

use crate::components::flashy_homepage::tracks::track_grid_item::TrackGridItem;

#[function_component(TrackGrid)]
pub fn track_grid() -> Html {
    html! {
    <div class="w-full grid md:grid-cols-3 grid-cols-1 gap-4 mt-8">
        <TrackGridItem track_name="Hackers' Choice" background_image="img/track_hackers_choice.webp" />
    </div>
    }
}
