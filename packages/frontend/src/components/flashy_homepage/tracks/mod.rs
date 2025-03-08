use track_grid_item::TrackCompany;
use yew::prelude::*;

use crate::components::flashy_homepage::tracks::track_grid_item::TrackGridItem;

mod track_grid_item;

#[function_component(TrackGrid)]
pub fn track_grid() -> Html {
    html! {
        <ul class="w-full grid md:grid-cols-3 grid-cols-1 gap-4 mt-8">
            <TrackGridItem
                track_name="Hackers' Choice"
                background_image="img/hero/l2.webp"
                track_caption="That's right, you also get a say! It is always common in hackathons to find projects that deserve praise but don't quite meet the requirements for a specific track. That's where you come in. Everyone will get a vote for Hackers' Choice. So when you see another project that inspires you in the presentations, give it a vote!"
                track_prize="Amazon Echo Pop"
            />
            <TrackGridItem
                track_name="Most Technically Impressive"
                background_image="img/tracks/technically_impressive.webp"
                track_caption="Looking for work done within the 24 hour challenge limit that is functional and has elegant code."
                track_prize="Mechanical keyboard"
                track_company={TrackCompany {
                name: "Airbus".to_owned(),
                link: "https://www.airbus.com/en".to_owned()
            }}
            />
            <TrackGridItem
                track_name="Best Use of AI"
                background_image="img/tracks/best_ai.webp"
                track_caption="Create a project demonstrating a high-quality use of AI."
                track_prize="Sony headphones"
                track_company={TrackCompany {
                name: "Siemens".to_owned(),
                link: "https://www.siemens.com/uk/en.html".to_owned()
            }}
            />
            <TrackGridItem
                track_name="Best User Interface"
                background_image="img/tracks/user_interface.webp"
                track_caption="No description provided."
                track_prize="Wacom drawing tablet"
                track_company={TrackCompany {
                name: "Kingfisher".to_owned(),
                link:  "https://www.kingfisher.com/".to_owned(),
            }}
            />
            <TrackGridItem
                track_name="Biggest Impact on Society"
                background_image="img/tracks/society.webp"
                track_caption="No description provided."
                track_prize="Coffee machine"
                track_company={TrackCompany {
            name: "Fivium".to_owned(),
            link: "https://www.fivium.co.uk/".to_owned()
        }}
            />
            <TrackGridItem
                track_name="Health and Wellbeing"
                background_image="img/tracks/health.webp"
                track_caption="HomeLink are looking for a student group who choose to focus on an idea that can improve the mental or physical health of users for an affordable price. Additionally, HomeLink is interested in ideas that are practically safe and adaptable for people to use in their everyday lives."
                track_prize="Stuffed goose"
                track_company={TrackCompany {
            name: "HomeLink".to_owned(),
            link: "https://homelink.com/".to_owned()
        }}
            />
            <TrackGridItem
                track_name="Market Ready Business Idea"
                background_image="img/tracks/business.webp"
                track_caption="To impress our internal judges for this track you should ensure your project is not just unique, but also fills a real gap in the current market and is a feasible idea to build a company (bonus points if you consider things like the cost of upscaling your idea)."
                track_prize="£25 SportsDirect gift card"
                track_company={TrackCompany {
            name: "House of Fraser".to_owned(),
            link: "https://www.houseoffraser.co.uk/".to_owned()
        }}
            />
        </ul>
    }
}
