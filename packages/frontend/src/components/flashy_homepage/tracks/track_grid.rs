use yew::prelude::*;

use crate::components::flashy_homepage::tracks::track_grid_item::TrackGridItem;

#[function_component(TrackGrid)]
pub fn track_grid() -> Html {
    html! {
        <ul class="w-full grid md:grid-cols-3 grid-cols-1 gap-4 mt-8">
            <TrackGridItem
                track_name="Best Overall"
                background_image="img/tracks/best_overall.webp"
                track_caption="The ultimate prize: selected by our judges, this project will be a perfect combination of advanced technology, high-quality branding, strong vision, and general ingenuity. For this one, you'll really need to have done it all; it's basically like starting an entire business in 24 hours."
                track_prize="TBA"
                track_company="BCSS"
                track_company_link="https://thesubath.com/bcss"
            />
            <TrackGridItem
                track_name="Hackers' Choice"
                background_image="img/tracks/hackers_choice.webp"
                track_caption="People are great, and we famously always make good decisions as a group. The biggest decision of your collective lives is now upon you! (Debatably) Who’s got the funniest project? Which project really tugs at your heart strings? Which project do you think is just the best (except your own, obviously). It really is up to you!"
                track_prize="£125 Amazon Gift Card"
                track_company="BCSS"
                track_company_link="https://thesubath.com/bcss"
            />
            <TrackGridItem
                track_name="Most Technically Impressive"
                background_image="img/tracks/technologically_impressive.webp"
                track_caption="Think you can use deep learning to solve P=NP? Or how about a universal password hash cracker? We're looking for the most mind-bogglingly techy projects which manage to do something incredible in just 24 hours. The standards are always high, and this year will be no different!"
                track_prize="TBA"
                track_company="BCSS"
                track_company_link="https://thesubath.com/bcss"
            />
            <TrackGridItem
                track_name="Embedded Systems"
                background_image="img/tracks/embedded_systems.webp"
                track_caption="Projects in the Embedded Track will focus on hardware and software for use in an embedded system. Each project need not produce a complete device. However for projects producing an embedded sub-system, the judges will need sufficient contextual information regarding the device that incorporates it in order to assess its merits."
                track_prize="TBA (worth ~£125 each)"
                track_company="XMOS"
                track_company_link="https://xmos.com"
            />
            <li
                class="h-40 flex justify-center items-center border-2 border-gray-200 dark:border-bcss-800 rounded-2xl"
            >
                <p class="text-bcss-500 dark:text-bcss-400 font-light">
                    { "More tracks coming soon..." }
                </p>
            </li>
        </ul>
    }
}
