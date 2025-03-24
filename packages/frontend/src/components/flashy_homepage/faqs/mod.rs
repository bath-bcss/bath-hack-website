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
                i={FAQItemIndex::SignupsFull}
                on_click={on_item_click.clone()}
                question="Signups are full?!"
            >
                <FAQItemParagraph>
                    { "We know it's annoying! We have a record-breaking 220 participants this year, and we aren't financially able to provide a good experience for any more than this. New spaces are very unlikely to become available, but if they do you'll be able to sign up through this website." }
                </FAQItemParagraph>
                <FAQItemParagraph>
                    { "If you or someone you know can no longer take part, please email su-bcss@bath.ac.uk to free up the space." }
                </FAQItemParagraph>
                <FAQItemParagraph>
                    { "If someone who was supposed to be in your group didn't get to sign up, please email us and we'll see if we can sort out an additional spot." }
                </FAQItemParagraph>
            </FAQItem>
            <FAQItem
                current_open={currently_open_index.clone()}
                i={FAQItemIndex::NoCode}
                on_click={on_item_click.clone()}
                question="I don't know how to code!"
            >
                <FAQItemParagraph>
                    { "Don't worry! There's loads of ways to take part and win prizes at Bath Hack without any coding whatsoever. We'll give a tutorial at the start of the event, and there'll also be opportunities to learn simple coding skills. Many tracks assess the idea and design of a project, not the complexity of its implementation." }
                </FAQItemParagraph>
            </FAQItem>
            <FAQItem
                current_open={currently_open_index.clone()}
                i={FAQItemIndex::Duration}
                on_click={on_item_click.clone()}
                question="Do I have to stay 24 hours?
    "
            >
                <FAQItemParagraph>
                    { "No! You can leave and return whenever you like. But if you want a once-in-a-lifetime sleepover with all your besties, this is the perfect opportunity! At least two Committee Members will be on-site 24/7 to help with any needs or give coding advice." }
                </FAQItemParagraph>
            </FAQItem>
            <FAQItem
                current_open={currently_open_index.clone()}
                i={FAQItemIndex::WhoCanParticipate}
                on_click={on_item_click.clone()}
                question="Who can take part?"
            >
                <FAQItemParagraph>
                    { "Any student at the University of Bath! You don't have to do computer science, and we get loads of participants from a range of departments every year. There's many ways to take part without even writing code." }
                </FAQItemParagraph>
            </FAQItem>
            <FAQItem
                current_open={currently_open_index.clone()}
                i={FAQItemIndex::Food}
                on_click={on_item_click.clone()}
                question="FOOD?!"
            >
                <FAQItemParagraph>
                    { "YES!! Free dinner (pizza), breakfast, and lunch will be provided as well as a range of snacks and drinks to keep you going through the long night." }
                </FAQItemParagraph>
            </FAQItem>
            <FAQItem
                current_open={currently_open_index.clone()}
                i={FAQItemIndex::Prizes}
                on_click={on_item_click.clone()}
                question="What are the prizes?"
            >
                <FAQItemParagraph>
                    { "Each track has a large and exciting prize associated with it. These will be high-value and sought-after, so the competition will be strong! We'll announce the specific prizes soon." }
                </FAQItemParagraph>
            </FAQItem>
            <FAQItem
                current_open={currently_open_index.clone()}
                i={FAQItemIndex::Equipment}
                on_click={on_item_click.clone()}
                question="What equipment can I use?"
            >
                <FAQItemParagraph>
                    { "We recommend using your own laptop for coding (some people even bring a full PC setup)! Don't worry if you can't though - you'll be able to use the computer labs in the building! We'll also (hopefully) have a full electronics lab with soldering and 3D printing facilities." }
                </FAQItemParagraph>
            </FAQItem>
            <FAQItem
                current_open={currently_open_index.clone()}
                i={FAQItemIndex::ExistingCode}
                on_click={on_item_click.clone()}
                question="Can I use existing code or ChatGPT?"
            >
                <FAQItemParagraph>
                    { "It's forbidden to write any code prior to the hackathon for the purpose of your project. However, you're welcome to use any open-source libraries and generative AI tools, as long as you give proper credit and follow licensing requirements." }
                </FAQItemParagraph>
            </FAQItem>
            <FAQItem
                current_open={currently_open_index.clone()}
                i={FAQItemIndex::Games}
                on_click={on_item_click.clone()}
                question="Will there be games?"
            >
                <FAQItemParagraph>
                    { "Yes! We'll have a fun mystery midnight game, with some more exciting prizes available!" }
                </FAQItemParagraph>
            </FAQItem>
            <FAQItem
                current_open={currently_open_index.clone()}
                i={FAQItemIndex::Recruiters}
                on_click={on_item_click.clone()}
                question="Can I network with recruiters?"
            >
                <FAQItemParagraph>
                    { "Yes! Many of our sponsors will have representatives at our career fair near the start of the event! You'll be able to talk with them about placements and graduate opportunities, and they'll have lots of goodies!" }
                </FAQItemParagraph>
            </FAQItem>
        </div>
    }
}
