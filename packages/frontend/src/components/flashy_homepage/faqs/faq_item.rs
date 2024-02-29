use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum FAQItemIndex {
    WhatIsHackathon,
    WhatIsPrizeTrack,
    WhoCanParticipate,
    Where,
    TwentyFour,
    Food,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub current_open: Option<FAQItemIndex>,
    pub i: FAQItemIndex,
    pub on_click: Callback<FAQItemIndex>,
    pub question: String,
    pub children: Html,
}

#[function_component(FAQItem)]
pub fn faq_item(props: &Props) -> Html {
    let is_open = props.current_open == Some(props.i.clone());

    let link_classes = use_memo((is_open,), |(is_open,)| {
        let mut base_classes = classes!(
            "w-full",
            "block",
            "py-2",
            "px-4",
            "bg-bcss-100",
            "dark:bg-bcss-800",
            "transition-all",
            "rounded-xl",
            "text-bcss-900",
            "dark:text-bcss-200",
            "cursor-pointer",
            "hover:underline"
        );

        if is_open.clone() {
            base_classes.push("rounded-b-none");
        }

        base_classes
    });

    let expanded_div_classes = use_memo((is_open,), |(is_open,)| {
        let mut base_class = classes!(
            "bg-bcss-100",
            "dark:bg-bcss-800",
            "rounded-b-xl",
            "transition-[max-height,padding]",
            "origin-top",
            "max-h-0",
            "overflow-hidden",
            "space-y-2",
            "px-4",
        );

        if is_open.clone() {
            base_class.push(classes!("max-h-40", "py-4"));
        }

        base_class
    });

    let on_question_click = use_callback(
        (props.i.clone(), props.on_click.clone()),
        |_, (i, on_click)| {
            on_click.emit(i.clone());
        },
    );

    html! {
    <div>
        <p class="w-full text-lg">
            <a class={(*link_classes).clone()} onclick={on_question_click}>
                {props.question.clone()}
            </a>
        </p>
        <div class={(*expanded_div_classes).clone()}>
            {props.children.clone()}
        </div>
    </div>
    }
}
