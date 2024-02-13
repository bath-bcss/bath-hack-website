use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub input_label: Option<String>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    pub value: String,
    pub onchange: Callback<InputEvent, ()>,
}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let input_class = classes!(
        "bg-transparent",
        "px-4",
        "py-3",
        "rounded-md",
        "border",
        "border-bcss-700",
        "focus:ring-2",
        "focus:border-bcss-800",
        "ring-bcss-800",
        "transition-all",
        "outline-none",
        "text-md",
        "placeholder:text-bcss-800",
        "w-full",
    );

    let label_class = classes!("mb-1", "block", "text-bcss-900");

    let label_id = use_memo((), |_| Uuid::new_v4().to_string());

    html! {
    <>
        if props.input_label.clone().is_some() {
        <label for={(*label_id).clone()} class={label_class}>
            { props.input_label.clone().unwrap() }
        </label>
        }
        <input class={input_class} id={(*label_id).clone()} placeholder={props.placeholder.clone()}
            oninput={props.onchange.clone()} />
    </>
    }
}
