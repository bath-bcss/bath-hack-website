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
                    event="Introductory presentation"
                    day="Sat 8th Mar"
                    time="11:00-12:00"
                    is_first=true
                />
                <ScheduleItem event="Hacking starts" time="12:00" is_first=false />
                <ScheduleItem event="Free pizza provided" time="17:00-18:00" is_first=false />
                <ScheduleItem
                    event="Games and films"
                    time="18:00-23:00"
                    is_first=false
                    has_caption=true
                >
                    { "To help stay in a creative mood we will play some movies and simple games to provide a break when you get tired of debugging and error screens." }
                </ScheduleItem>
                <ScheduleItem
                    event="Coffee and snacks"
                    day="Sun 9th Mar"
                    time="08:00-10:00"
                    is_first=false
                />
                <ScheduleItem
                    event="Networking event"
                    time="10:00-11:00"
                    is_first=false
                    has_caption=true
                >
                    { "We are giving time for you to meet our sponsors and discuss what work they do in their companies and any early careers roles they have open. We also invite you to seek some final feedback and tips on your hackathon projects before submission." }
                </ScheduleItem>
                <ScheduleItem
                    event="Hacking ends, free lunch provided"
                    time="12:00"
                    is_first=false
                />
                <ScheduleItem event="Project presentation" time="13:00-14:00" is_first=false />
                <ScheduleItem
                    event="Judging period"
                    time="14:00-15:00"
                    is_first=false
                    has_caption=true
                >
                    { "During the judging period we will proidve some fun and short games to get moving again after staring at screens coding for 24 hours." }
                </ScheduleItem>
                <ScheduleItem
                    event="Awards presentation"
                    time="15:00-15:30"
                    is_first=false
                    is_last=true
                />
            </ul>
        </>
    }
}
