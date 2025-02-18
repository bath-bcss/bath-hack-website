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
                i={FAQItemIndex::Duration}
                on_click={on_item_click.clone()}
                question="Do I have to stay 24 hours?
    "
            >
                <FAQItemParagraph>
                    { "No! You can leave and return to the 1 West building as you like. Members of our committee will be on shifts at all times of the event so if you would like to stay the full time you are free to do so. But there is no expectation for you to stay awake 24 hours. Please put your health first." }
                </FAQItemParagraph>
            </FAQItem>
            <FAQItem
                current_open={currently_open_index.clone()}
                i={FAQItemIndex::WhoCanParticipate}
                on_click={on_item_click.clone()}
                question="Who is invited to participate?"
            >
                <FAQItemParagraph>
                    { "Any student of Bath University who identifies as a woman or gender minority can take part in a group of 1 to 4. While some prize tracks are focused on rewarding impressive code solutions, you are free to join no matter your skill level." }
                </FAQItemParagraph>
            </FAQItem>
            <FAQItem
                current_open={currently_open_index.clone()}
                i={FAQItemIndex::Food}
                on_click={on_item_click.clone()}
                question="What food and drink is provided?"
            >
                <FAQItemParagraph>
                    { "All food and drink provided is free, we will provide pizza Saturday at 6pm and lunch on Sunday, if you have any dietary restrictions please specify in your sign up or email wit@bath.ac.uk to let us know. We will also provide hot drinks and snacks Sunday morning similar to what we provide in our weekly coffee meets." }
                </FAQItemParagraph>
            </FAQItem>
            <FAQItem
                current_open={currently_open_index.clone()}
                i={FAQItemIndex::ExistingCode}
                on_click={on_item_click.clone()}
                question="Can I use existing code / Gen AI?"
            >
                <FAQItemParagraph>
                    { "We allow the use of any and all online resources. You only have 24 hours so you should be resourscful in how you work. However, you should start a fresh project Saturday at 12pm, no work should be done prior to the event." }
                </FAQItemParagraph>
            </FAQItem>
        </div>
    }
}
