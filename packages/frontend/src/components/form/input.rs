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
    pub onchange: Option<Callback<InputEvent>>,

    #[prop_or_default]
    pub readonly: bool,

    #[prop_or_default]
    pub input_type: Option<String>,
    #[prop_or_default]
    pub accept_file: Option<String>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub input_class: Option<Classes>,
    #[prop_or_default]
    pub container_class: Option<Classes>,

    #[prop_or_default]
    pub children: Html,
}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let handle_value = {
        if props.input_type == Some("file".to_string()) {
            None
        } else {
            match props.handle.clone() {
                Some(h) => Some((*h).clone()),
                None => props.static_value.clone(),
            }
        }
    };
    let on_change_handler = {
        let handle = props.handle.to_owned();

        use_callback(
            (props.onchange.clone(),),
            move |value: InputEvent, (on_change_prop,)| {
                if let Some(on_change_prop) = on_change_prop {
                    on_change_prop.emit(value.clone());
                }

                if let Some(handle) = handle.clone() {
                    let target = value.target_dyn_into::<HtmlInputElement>();
                    if let Some(target) = target {
                        handle.set(target.value());
                    }
                }
            },
        )
    };

    let child_renderer = use_callback(
        (
            props.input_class.clone(),
            props.placeholder.clone(),
            props.input_type.clone(),
            props.accept_file.clone(),
            props.required,
            props.disabled,
            props.readonly,
            props.children.clone(),
            handle_value,
            on_change_handler,
        ),
        |child_props: FormHandleChildProps,
         (
            input_class_extra_props,
            placeholder,
            input_type,
            accept_file,
            required,
            disabled,
            readonly,
            children,
            handle_value,
            on_change_handler,
        )| {
            let mut input_class = child_props.class;
            if let Some(input_class_extra_props) = input_class_extra_props {
                input_class.push(input_class_extra_props.clone());
            }

            let mut children = children.clone();
            html! {
                <>
                    <input
                        class={input_class}
                        id={child_props.id}
                        placeholder={placeholder.clone()}
                        type={input_type.clone()}
                        required={required.clone()}
                        oninput={on_change_handler}
                        value={handle_value.clone()}
                        disabled={disabled.clone()}
                        readonly={readonly.clone()}
                        accept={accept_file.clone()}
                    />
                    if children.to_vlist_mut().len() > 0 {
                        <p class="text-gray-600 dark:text-gray-200 text-sm mt-1">
                            { children.clone() }
                        </p>
                    }
                </>
            }
        },
    );

    html! {
        <FormHandle
            child_renderer={child_renderer}
            container_class={props.container_class.clone()}
            label={props.input_label.clone()}
        />
    }
}
