use yew::prelude::*;

use crate::components::flashy_homepage::faqs::faq_item::FAQItem;

mod faq_item;

#[derive(Clone, PartialEq)]
enum FAQItemIndex {
    WhatIsHackathon,
}

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
        <FAQItem on_open_click={Callback::from(move |_| { on_item_click.emit(FAQItemIndex::WhatIsHackathon) })}
            is_open={currently_open_index==Some(FAQItemIndex::WhatIsHackathon)} question="What is a hackathon?">
            {"A hackathon is when pal pal pal"}
        </FAQItem>
    </div>
    }
}
