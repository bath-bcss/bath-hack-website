use yew::prelude::*;

use crate::components::flashy_homepage::tracks::track_grid_item::{TrackCompany, TrackGridItem};

#[function_component(TrackGrid)]
pub fn track_grid() -> Html {
    html! {
        <ul class="w-full grid md:grid-cols-3 grid-cols-1 gap-4 mt-8">
            <TrackGridItem
                track_name="Best Overall"
                background_image="img/tracks/best_overall.webp"
                track_caption="The ultimate prize: selected by our judges, this project will be a perfect combination of advanced technology, high-quality branding, strong vision, and general ingenuity. For this one, you'll really need to have done it all; it's basically like starting an entire business in 24 hours."
                track_prize="Monitor worth ~£250"
            />
            <TrackGridItem
                track_name="Hackers' Choice"
                background_image="img/tracks/hackers_choice.webp"
                track_caption="People are great, and we famously always make good decisions as a group. The biggest decision of your collective lives is now upon you! (Debatably) Who’s got the funniest project? Which project really tugs at your heart strings? Which project do you think is just the best (except your own, obviously). It really is up to you!"
                track_prize="£125 Amazon gift card"
            />
            <TrackGridItem
                track_name="Most Technically Impressive"
                background_image="img/tracks/technologically_impressive.webp"
                track_caption="Think you can use deep learning to solve P=NP? Or how about a universal password hash cracker? We're looking for the most mind-bogglingly techy projects which manage to do something incredible in just 24 hours. The standards are always high, and this year will be no different!"
                track_prize="DJI Tello programmable drone (Boost Combo)"
            />
            <TrackGridItem
                track_name="Embedded Systems"
                background_image="img/tracks/embedded_systems.webp"
                track_caption="Projects in the Embedded Track will focus on hardware and software for use in an embedded system. Each project need not produce a complete device. However for projects producing an embedded sub-system, the judges will need sufficient contextual information regarding the device that incorporates it in order to assess its merits."
                track_prize="TBA (worth ~£125 each)"
                track_company={TrackCompany{ name: "XMOS" .to_string(),
            link: "https://xmos.com" .to_string() }}
            />
            <TrackGridItem
                track_name="E-commerce"
                background_image="img/tracks/ecommerce.webp"
                track_caption=""
                track_prize="TBA (worth ~£125 each)"
                track_company={TrackCompany{ name: "SparkLayer" .to_string(),
            link: "https://sparklayer.io" .to_string() }}
            />
            <TrackGridItem
                track_name="Best use of AI"
                background_image="img/tracks/ai.webp"
                track_caption="To get this prize, your project needs to use AI tools (including pre-trained models, custom-built models, or even existing APIs such as OpenAI) to do something novel and cool. Consider building a solution that's either useful to parts of society, or just fun to use! Your final product doesn't need to be fully functional, but it's always an added bonus."
                track_prize="UE BOOM 3 Wireless Speaker"
            />
            <TrackGridItem
                track_name="Best Design and Concept"
                background_image="img/tracks/design.webp"
                track_caption="This one's new for 2024! You don't have to write any code at all for this track, but you need creativity and ambition! We're looking for a perfectly designed project concept/mockup with great potential to be made into a real implementation — preferably, this would be something novel and intelligent that could genuinely have a positive impact on peoples' lives."
                track_prize="XP-PEN Deco Pro Drawing Tablet"
            />
        </ul>
    }
}
