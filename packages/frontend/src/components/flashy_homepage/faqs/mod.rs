use yew::prelude::*;

use crate::components::flashy_homepage::faqs::{
    faq_item::{FAQItem, FAQItemIndex},
    faq_item_paragraph::FAQItemParagraph,
};

mod faq_item;
mod faq_item_paragraph;

#[function_component(FAQs)]
pub fn faqs_component() -> Html {
    let currently_open_handle = use_state_eq(|| None::<FAQItemIndex>);
    let currently_open_index = (*currently_open_handle).clone();

    let on_item_click = use_callback(
        (currently_open_handle, currently_open_index.clone()),
        |i: FAQItemIndex, (currently_open_handle, currently_open_index)| {
            let currently_open_index = (*currently_open_index).clone();
            if Some(i.clone()) == currently_open_index {
                currently_open_handle.set(None);
                return;
            }

            currently_open_handle.set(Some(i));
        },
    );

    html! {
    <div class="mt-4 space-y-4">
        <FAQItem current_open={currently_open_index.clone()} i={FAQItemIndex::WhatIsHackathon}
            on_click={on_item_click.clone()} question="What is a hackathon?">
            <FAQItemParagraph>
                {"In short, it is the ultimate technological group work task. You have 24hrs and a team to plan, design,
                develop and pitch a system from scratch. Whether you sacrifice functionality for ambition or strive for
                a
                fully working system the options are boundless. Pick a prize track and creatively solve the problem and
                you'll be real competition. As cliche as it sounds, the most important thing is have fun and meet new
                people."}
            </FAQItemParagraph>
        </FAQItem>
        <FAQItem current_open={currently_open_index.clone()} i={FAQItemIndex::WhatIsPrizeTrack}
            on_click={on_item_click.clone()} question="What is a track?">
            <FAQItemParagraph>
                {"A track is a problem that needs to be solved or a question that needs to be answered. It's a
                specific
                goal your solution should fulfil."}
            </FAQItemParagraph>
            <FAQItemParagraph>
                {"Tracks can be vague or specific and your project can be eligible for more than one."}
            </FAQItemParagraph>
        </FAQItem>
        <FAQItem current_open={currently_open_index.clone()} i={FAQItemIndex::WhoCanParticipate}
            on_click={on_item_click.clone()} question="Who can participate in this event?">
            <FAQItemParagraph>
                {"Any student at the University of Bath! Not just Computer Scientists."}
            </FAQItemParagraph>
            <FAQItemParagraph>
                {"In fact, you don't even need experience with coding or tech in general. All you need are creativity
                and enthusiasm! There'll be lots of support available for those with less experience — but there will
                also be ways to participate without writing a single line of code."}
            </FAQItemParagraph>
        </FAQItem>
        <FAQItem current_open={currently_open_index.clone()} i={FAQItemIndex::Where} on_click={on_item_click.clone()}
            question="Where is Bath Hack?">
            <FAQItemParagraph>
                {"Bath Hack 2024 will take place on-campus in the Chancellor's Building. The entire building will be
                reserved for the event, with various rooms in use for talks, workshops, etc."}
            </FAQItemParagraph>
            <FAQItemParagraph>
                {"Lifts, double-width doors, hearing loop support and water fountains are available in the building. If
                you have any specific access requirements that we haven't thought of, please let us know!"}
            </FAQItemParagraph>
        </FAQItem>
        <FAQItem current_open={currently_open_index.clone()} i={FAQItemIndex::TwentyFour}
            on_click={on_item_click.clone()} question="24 hours‽">
            <FAQItemParagraph>
                {"You don't have to use the full time and we encourage you to take a break and/or sleep. But yes, 24hrs
                of potential project time."}
            </FAQItemParagraph>
            <FAQItemParagraph>
                {"Good luck, use it wisely!"}
            </FAQItemParagraph>
        </FAQItem>
        <FAQItem current_open={currently_open_index.clone()} i={FAQItemIndex::Food} on_click={on_item_click.clone()}
            question="Food?">
            <FAQItemParagraph>
                {"Food indeed! There will be 3 meals provided free of charge (thanks to our lovely sponsors!) to all
                participants, including dinner, breakfast,
                and lunch. We'll try to cater to all dietary requirements, so please specify these when you create your
                account."}
            </FAQItemParagraph>
        </FAQItem>
    </div>
    }
}
