use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    pub dark_mode: bool,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent, ()>>,
    #[prop_or_default]
    pub class: Option<Classes>,
    #[prop_or("button".to_string())]
    pub button_type: String,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let class = use_memo((props.class.clone(),), move |(class_prop,)| {
        let mut class = classes!(
            "text-white",
            if props.dark_mode {
                "bg-transparent"
            } else {
                "bg-bcss-700"
            },
            "px-4",
            "py-2",
            "rounded-md",
            if props.dark_mode {
                "border-2"
            } else {
                "border-0"
            },
            "focus:ring-4",
            "focus:outline-0",
            "ring-bcss-300",
            "transition-all",
            if props.dark_mode {
                "hover:bg-white/20"
            } else {
                "hover:bg-bcss-800/90"
            },
            if props.dark_mode {
                "active:bg-white/10"
            } else {
                "active:bg-bcss-800"
            },
            "disabled:bg-gray-400",
            "disabled:cursor-not-allowed"
        );

        if let Some(class_prop) = class_prop {
            class.push((*class_prop).clone());
        }

        class
    });

    html! {
    <button class={(*class).clone()} onclick={props.onclick.clone()} type={props.button_type.clone()}
        disabled={props.disabled}>
        {props.children.clone()}
    </button>
    }
}
