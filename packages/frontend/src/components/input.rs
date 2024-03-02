use rand::{distributions::{Alphanumeric, DistString}, rngs::OsRng};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub input_label: Option<String>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub handle: Option<UseStateHandle<String>>,
    #[prop_or_default]
    pub static_value: Option<String>,

    #[prop_or_default]
    pub readonly: bool,

    #[prop_or_default]
    pub input_type: Option<String>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub input_class: Option<Classes>,
    #[prop_or_default]
    pub container_class: Option<Classes>,
}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let input_class = use_memo((props.input_class.clone(),), |(button_class_prop,)| {
        let mut input_class = classes!(
            "bg-transparent",
            "px-4",
            "py-3",
            "rounded-md",
            "border",
            "border-bcss-700",
            "focus:ring-2",
            "focus:border-bcss-800",
            "ring-bcss-800",
            "dark:ring-bcss-400",
            "transition-all",
            "outline-none",
            "text-md",
            "placeholder:text-bcss-800",
            "dark:placeholder:text-bcss-400",
            "dark:text-bcss-100",
            "w-full",
        );

        if let Some(button_class_prop) = button_class_prop {
            input_class.push((*button_class_prop).clone());
        }

        input_class
    });

    let label_class = classes!("mb-1", "block", "text-bcss-900", "dark:text-bcss-300");

    let label_id = use_memo((), |_| {
        Alphanumeric.sample_string(&mut OsRng, 5)
    });

    let handle_value = match props.handle.clone() {
        Some(h) => (*h).clone(),
        None => match props.static_value.clone() {
            Some(s) => s,
            None => String::default(),
        },
    };
    let on_change_handler = {
        let handle = props.handle.to_owned();

        use_callback((), move |value: InputEvent, _| {
            if let Some(handle) = handle.clone() {
                let target = value.target_dyn_into::<HtmlInputElement>();
                if let Some(target) = target {
                    handle.set(target.value());
                }
            }
        })
    };

    html! {
    <div class={props.container_class.clone()}>
        if props.input_label.clone().is_some() {
        <label for={(*label_id).clone()} class={label_class}>
            { props.input_label.clone().unwrap() }
        </label>
        }
        <input class={(*input_class).clone()} id={(*label_id).clone()}
            placeholder={props.placeholder.clone()} type={props.input_type.clone()}
            required={props.required.clone()} oninput={on_change_handler}
            value={handle_value} disabled={props.disabled.clone()}
            readonly={props.readonly.clone()} />
    </div>
    }
}
