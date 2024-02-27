use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub is_open: bool,
    pub on_open_click: Callback<MouseEvent>,
    pub question: String,
    pub children: Html,
}

#[function_component(FAQItem)]
pub fn faq_item(props: &Props) -> Html {
    let link_classes = use_memo((props.is_open,), |(is_open,)| {
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

    let expanded_div_classes = use_memo((props.is_open,), |(is_open,)| {
        let mut base_class = classes!(
            "bg-bcss-100",
            "dark:bg-bcss-800",
            "rounded-b-xl",
            "p-4",
            "transition-transform",
            "origin-top",
            "scale-y-0",
        );

        if is_open.clone() {
            base_class.push("scale-y-100");
        }

        base_class
    });

    html! {
    <div>
        <p class="w-full">
            <a class={(*link_classes).clone()} onclick={props.on_open_click.clone()}>
                {props.question.clone()}
            </a>
        </p>
        <div class={(*expanded_div_classes).clone()}>
            <p class="text-bcss-900 dark:text-bcss-200">
                {props.children.clone()}
            </p>
        </div>
    </div>
    }
}
