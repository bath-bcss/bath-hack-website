use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::form::form_handle::FormHandle;

use super::form_handle::FormHandleChildProps;

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

    let child_renderer = use_callback(
        (
            props.input_class.clone(),
            props.placeholder.clone(),
            props.input_type.clone(),
            props.required,
            props.disabled,
            props.readonly,
            handle_value,
            on_change_handler,
        ),
        |child_props: FormHandleChildProps,
         (
            input_class_extra_props,
            placeholder,
            input_type,
            required,
            disabled,
            readonly,
            handle_value,
            on_change_handler,
        )| {
            let mut input_class = child_props.class;
            if let Some(input_class_extra_props) = input_class_extra_props {
                input_class.push(input_class_extra_props.clone());
            }

            html! {
            <input class={input_class} id={child_props.id} placeholder={placeholder.clone()}
                type={input_type.clone()} required={required.clone()} oninput={on_change_handler}
                value={handle_value.clone()} disabled={disabled.clone()}
                readonly={readonly.clone()} />
            }
        },
    );

    html! {
    <FormHandle child_renderer={child_renderer}
        container_class={props.container_class.clone()} label={props.input_label.clone()} />
    }
}
