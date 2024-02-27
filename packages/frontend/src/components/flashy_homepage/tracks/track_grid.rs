use yew::prelude::*;

use crate::components::flashy_homepage::tracks::track_grid_item::TrackGridItem;

#[function_component(TrackGrid)]
pub fn track_grid() -> Html {
    html! {
    <div class="w-full grid md:grid-cols-3 grid-cols-1 gap-4 mt-8">
        <TrackGridItem track_name="Hackers' Choice" background_image="img/track_hackers_choice.webp"
            track_caption="Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis."
            track_prize="1 BILLION air molecules" track_company="BCSS" track_company_link="https://thesubath.com/bcss" />
    </div>
    }
}
