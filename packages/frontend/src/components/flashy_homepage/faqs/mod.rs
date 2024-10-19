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
            <FAQItem
                current_open={currently_open_index.clone()}
                i={FAQItemIndex::InPerson}
                on_click={on_item_click.clone()}
                question="Is Game Jam in-person?"
            >
                <FAQItemParagraph>
                    { "Our kick-off and showcase sessions are both in-person. We'd like you to attend both of them if you're taking part in the competition. Don't worry, we'll make them super fun! We'll have a range of snacks at both, and pizza at the showcase. Other than these events, you'll do the development of your game in your own time. You can meet with your group in-person or online, however you'd like!" }
                </FAQItemParagraph>
            </FAQItem>
            <FAQItem
                current_open={currently_open_index.clone()}
                i={FAQItemIndex::WhatIsPrizeTrack}
                on_click={on_item_click.clone()}
                question="What is a track?"
            >
                <FAQItemParagraph>
                    { "A track is like a target your project should be aiming for. You can choose which tracks you'd like to aim for at the start. However, you don't need to actually tell us which you choose, as your project will automatically be considered for all of them as applicable." }
                </FAQItemParagraph>
            </FAQItem>
            <FAQItem
                current_open={currently_open_index.clone()}
                i={FAQItemIndex::WhoCanParticipate}
                on_click={on_item_click.clone()}
                question="Who can participate in this event?"
            >
                <FAQItemParagraph>{ "Any student at the University of Bath!" }</FAQItemParagraph>
                <FAQItemParagraph>
                    { "In fact, you don't even need experience with coding or tech in general. All you need are creativity
                and enthusiasm! There'll be lots of support available for those with less experience — but there will
                also be ways to participate without writing a single line of code." }
                </FAQItemParagraph>
                <FAQItemParagraph>
                    { "Plus, none of the tracks even require you to write code! A lot of groups will use digital approaches, but there's no requirement to do so. We're just looking for creative answers to our tracks." }
                </FAQItemParagraph>
            </FAQItem>
            <FAQItem
                current_open={currently_open_index.clone()}
                i={FAQItemIndex::Devpost}
                on_click={on_item_click.clone()}
                question="How do I submit my game?"
            >
                <FAQItemParagraph>
                    { "We use a website called " }
                    <strong>{ "Devpost" }</strong>
                    { ". This allows you to submit a cool profile of your project with all your teammates added to it. We'll need you to upload a short video showcasing your game, as well as a brief description of how you came up with the idea. Don't worry, this isn't like an assignment, and it's super easy to do! Future employers will be able to see your work here, so it's great for your career too." }
                </FAQItemParagraph>
            </FAQItem>
            <FAQItem
                current_open={currently_open_index.clone()}
                i={FAQItemIndex::DevelopmentPeriod}
                on_click={on_item_click.clone()}
                question="How long do I have to build my game?"
            >
                <FAQItemParagraph>
                    { "You'll have from 12:00 on Saturday 2nd November until 20:00 on Tuesday 12th November, so just over 10 days of project time! You can decide how to allocate your time and when to meet with your group. The deadline is a hard one, so we won't consider any submissions or changes made after that time." }
                </FAQItemParagraph>
                <FAQItemParagraph>{ "Good luck, use your time wisely!" }</FAQItemParagraph>
            </FAQItem>
            <FAQItem
                current_open={currently_open_index.clone()}
                i={FAQItemIndex::Food}
                on_click={on_item_click.clone()}
                question="Food?"
            >
                <FAQItemParagraph>
                    { "Food indeed! There will be free pizza provided to all
                participants at the showcase event, as well as a range of snacks. We'll try to cater to all dietary requirements, so please specify these when you create your account." }
                </FAQItemParagraph>
            </FAQItem>

        </div>
    }
}
