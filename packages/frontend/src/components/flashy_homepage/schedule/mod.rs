use yew::prelude::*;

use crate::components::flashy_homepage::{
    schedule::schedule_item::ScheduleItem,
    section::section_paragraph::FlashyHomepageSectionParagraph,
};

mod schedule_item;

#[function_component(Schedule)]
pub fn schedule() -> Html {
    html! {
        <>
            <FlashyHomepageSectionParagraph>
                { "Click on each item to see more information." }
            </FlashyHomepageSectionParagraph>
            <ul class="mt-4 space-y-2">
                <ScheduleItem
                    event="Sign-in and introduction"
                    day="Sat 5th Apr"
                    time="10:00-12:00"
                    is_first=true
                    has_caption=true
                >
                    { "Come to our reception to get checked into the event. We'll make sure you've got a group and help you find one if you don't. Then, explore our career's fair to network with our exciting placement and graduate employers." }
                </ScheduleItem>
                <ScheduleItem event="Hacking starts" time="12:00" is_first=false has_caption=true>
                    { "Get started with hacking! You'll have exactly 24 hours to build and document your project. Our Committee Members will be there to help with coding or practical issues, so look out for them in their hoodies." }
                </ScheduleItem>
                <ScheduleItem event="Free dinner!" time="19:00" has_caption=true>
                    { "That's right! Bath Hack wouldn't be the same without free food. Dinner, breakfast, lunch, and snacks are all included at no charge whatsoever, thanks to our lovely sponsors! We usually serve Domino's pizza for dinner, but we'll confirm this shortly. A range of options will be available to meet all dietary requirements, so make sure to specify them when you sign up!" }
                </ScheduleItem>
                <ScheduleItem event="Breakfast" time="08:00" has_caption=false />
                <ScheduleItem
                    event="Hacking ends & lunch"
                    day="Sun 6th Apr"
                    time="12:00"
                    is_first=false
                    has_caption=true
                >
                    { "You'll need to finish your coding exactly on time, and make a submission to our Devpost page. Our Committee will be there to help you should anything go wrong." }
                </ScheduleItem>
                <ScheduleItem
                    event="Project presentation"
                    time="13:00-14:30"
                    is_first=false
                    has_caption=true
                >
                    { "Our judges will go around and visit your project. Be prepared to give them a cool demonstration; often, the most creative demos win the best prizes! A lot of your chances depend on this, although your project itself should be impressive, too. You'll also take turns visiting other projects and decide on a Hacker's Choice vote." }
                </ScheduleItem>
                <ScheduleItem
                    event="Prizegiving"
                    time="15:00-15:30"
                    is_first=false
                    is_last=true
                    has_caption=true
                >
                    { "The moment you've all waited for! We'll conclude the event with a summary of all the amazing projects you'll all have inevitably created, and hand out our incredible prizes." }
                </ScheduleItem>
            </ul>
        </>
    }
}
