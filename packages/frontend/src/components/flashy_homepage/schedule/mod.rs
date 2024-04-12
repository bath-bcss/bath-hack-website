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
                <ScheduleItem event="Team building" day="Friday" time="16:15-18:05" is_first=true>
                    { "Don't have a team? Don't worry! Come along to our (optional) team-building session in CB3.1 on Friday
                afternoon where we'll be runing a series of fun activities to help you find a team, including a quiz! " }
                    { "There'll be plenty of opportunity to meet new people in a comfortable environment." }
                </ScheduleItem>
                <ScheduleItem event="Last-minute team building" day="Saturday" time="10:00-11:00">
                    { "Still don't have a team? That's okay! We'll have a final team-building event just before the hackathon
                starts, where we'll help get you into a team you're comfortable with. " }
                    { "If you can, please go along to the Friday session as it's likely to be more relaxed." }
                </ScheduleItem>
                <ScheduleItem event="Registration" time="10:00-11:00">
                    { "When you arrive at the hackathon, you'll need to register with one of our Committee Members at the main
                entrance of the Chancellors Building. " }
                    { "This will help us keep everyone safe at the event. " }
                    { "It's okay if you miss the registration time! We'll have someone at the desk throughout the event." }
                </ScheduleItem>
                <ScheduleItem event="XMOS presentation" time="10:30-11:30">
                    { "XMOS will be delivering some presentations and showcasing two demonstrations of their technology to you all. " }
                    { "You will be able to ask them questions are learn more about what they do." }
                </ScheduleItem>
                <ScheduleItem event="Introduction presentation" time="11:30-12:00">
                    { "Here's where everything gets started! We'll explain how the event works and go over the tracks and prizes
                you'll be competing for. " }
                    { "You'll also get a chance to hear about our sponsors, as well as some important safety-related
                information." }
                </ScheduleItem>
                <ScheduleItem event="Hacking starts!" time="12:00">
                    { "Hacking will start at 12:00 on the dot so we can keep to our 24 hour competition time. " }
                    { "You'll need to have finalised your group members and your group name by this point. " }
                    { "We'll have loads of rooms available for you to work in, including collaborative ones where you can
                check out what other teams are doing, as well as quieter ones." }
                </ScheduleItem>
                <ScheduleItem event="Pizza" time="~18:00">
                    { "The first meal of your culinary hackathon journey! " }
                    { " Make sure you've submitted your dietary requirements to avoid disappointment. " }
                    { "As mentioned, all meals are completely free of charge." }
                </ScheduleItem>
                <ScheduleItem event="Breakfast" day="Sunday" time="~06:00">
                    { "We'll have a buffet-style cold breakfast with a range of pastries, toast, snacks, and more." }
                </ScheduleItem>
                <ScheduleItem event="DevPost support" time="11:00-12:00">
                    { "You'll need to post your project on DevPost before hacking ends at 12:00. " }
                    { "We'll have a short session at 11:00 for anyone who hasn't done this before, and our Committee Members
                will be on hand to help sort out any issues." }
                </ScheduleItem>
                <ScheduleItem event="Hacking ends + Lunch" time="12:00">
                    { "You won't be able to make any changes to your project or its DevPost page after this time. " }
                    { "As the final meal, we'll serve lunch around this time: Subway!" }
                </ScheduleItem>
                <ScheduleItem event="Showcase Fair" time="12:30-14:30">
                    { "Along with all the other teams, you'll be able to demonstrate your project to our sponsors and judges at
                our Showcase Fair. " }
                    { "This is the best opportunity to impress the judges and secure your win! " }
                    { "You'll also be able to vote on your favourite projects for the Hackers' Choice prize." }
                </ScheduleItem>
                <ScheduleItem event="Closing presentation" time="14:30-15:15" is_last=true>
                    { "To finish off Bath Hack, we'll announce each of the winners and hand over their prizes. " }
                    { "We'll have some of our sponsors present here, so you can here more from them about their companies. " }
                    { "Now go home and get some sleep!" }
                </ScheduleItem>
            </ul>
        </>
    }
}
