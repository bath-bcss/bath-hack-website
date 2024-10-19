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
                    event="Team building and kick-off"
                    day="Wed 30th Oct"
                    time="CB 3.5, 18:15-21:05"
                    is_first=true
                >
                    { "We'll introduce the event and announce the surprise mystery theme! You'll get a chance to meet all our societies' committees and members and build your team of 4. Try getting a varied team with members from various societies for best results! Remember, this is just as much about design and concept as coding and execution. Of course, the kick-off event will feature Wii gameplay in your teams and we'll go to the SU for relaxed drinks straight after!" }
                </ScheduleItem>
                <ScheduleItem event="Development time starts" day="Sat 2nd Nov" time="12:00">
                    { "You can start working on your projects in your teams! How you do this is completely up to you; you can meet up in person or do it fully remote. If you need any advice or technical support, the BCSS Committee will be at your disposal. Whether you're coding or not, we'll be here to help you succeed :)" }
                </ScheduleItem>
                <ScheduleItem event="Development time ends" day="Tue 12th Nov" time="20:00">
                    { "This is the deadline! You'll need to submit your projects to Devpost (we'll explain how to do this) by this time, and you won't be able to make any changes afterwards. We don't want to set the deadline too late to encourage you to get some sleep before the showcase!" }
                </ScheduleItem>
                <ScheduleItem
                    event="Showcase and prize-giving"
                    day="Wed 13th Nov"
                    time="TBA"
                    is_last=true
                >
                    { "The big moment you've been waiting for! You'll get assigned a table and you'll have some time to set your game up in our open fair. Then, you'll be able to go around and play everyone else's games, and they'll come play yours. Everyone from the Uni is welcome here, so bring all your friends! There'll be snacks and free pizza (of course). Finally, our judges will make their decisions and we'll announce the winners." }
                </ScheduleItem>
            </ul>
        </>
    }
}
