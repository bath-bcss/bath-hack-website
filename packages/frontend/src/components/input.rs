use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub input_label: Option<String>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    pub handle: UseStateHandle<String>,

    #[prop_or_default]
    pub input_type: Option<String>,
    #[prop_or_default]
    pub required: bool,
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

    let handle_value = (*props.handle).clone();
    let on_change_handler = {
        let handle = props.handle.to_owned();

        use_callback((), move |value: InputEvent, _| {
            let target = value.target_dyn_into::<HtmlInputElement>();
            if let Some(target) = target {
                handle.set(target.value());
            }
        })
    };

    html! {
    <>
        if props.input_label.clone().is_some() {
        <label for={(*label_id).clone()} class={label_class}>
            { props.input_label.clone().unwrap() }
        </label>
        }
        <input class={input_class} id={(*label_id).clone()} placeholder={props.placeholder.clone()}
            type={props.input_type.clone()} required={props.required.clone()}
            oninput={on_change_handler} value={handle_value} />
    </>
    }
}
