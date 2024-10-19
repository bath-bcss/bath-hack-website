use yew::prelude::*;

use crate::components::flashy_homepage::tracks::track_grid_item::TrackGridItem;

mod track_grid_item;

#[function_component(TrackGrid)]
pub fn track_grid() -> Html {
    html! {
        <ul class="w-full grid md:grid-cols-3 grid-cols-1 gap-4 mt-8">
            <TrackGridItem
                track_name="Hackers' Choice"
                background_image="img/tracks/hackers_choice.webp"
                track_caption="People are great, and we famously always make good decisions as a group. The biggest decision of your collective lives is now upon you! Who’s got the funniest game? Which one really tugs at your heart strings? Which do you think is just the best? It really is up to you!"
                track_prize="£50 Amazon gift card + a DucksBath cap"
            />
            <TrackGridItem
                track_name="Best Concept"
                background_image="img/tracks/best_concept.webp"
                track_caption="This one's all about your vision! Even if you haven't managed to fully implement your game in a single week, our judges will be looking for an excellent unique idea. Perhaps your game has really quirky controls, or there's just something interesting about the story. As long as you've made some progress towards a prototype, you'll get considered for this!"
                track_prize="£50 Amazon gift card"
            />
            <TrackGridItem
                track_name="Best Art"
                background_image="img/tracks/best_art.webp"
                track_caption="With games, design is everything. We're looking for extremely well executed art that integrates well with your game's wider concept. Whether it's cute, ultra-realistic, or just satisfying to look at, we're certain the competition will be intense for this one!"
                track_prize="£25 Amazon gift card"
            />
            <TrackGridItem
                track_name="Best Physical Game"
                background_image="img/tracks/best_physical.webp"
                track_caption="That's right: there's absolutely no coding required for this one! Anything with a clear substantial non-computery element is eligible for this track. Ranging from board games to even a new version of football, it's entirely open to your interpretation. If you're unsure whether your project is eligible, you'll be able to ask our Committee during the kick-off event."
                track_prize="£25 Amazon gift card"
            />
        </ul>
    }
}
