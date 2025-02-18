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
                />
                <ScheduleItem event="Hacking starts" time="12:00" is_first=false />
                <ScheduleItem event="Hacking ends" day="Sun 6th Apr" time="12:00" is_first=false />
                <ScheduleItem event="Project presentation" time="13:00-14:30" is_first=false />
                <ScheduleItem
                    event="Prizegiving"
                    time="15:00-15:30"
                    is_first=false
                    is_last=true
                />
            </ul>
        </>
    }
}
