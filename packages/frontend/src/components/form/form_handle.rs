use web_sys::js_sys::Math;
use yew::prelude::*;

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub container_class: Option<Classes>,
    #[prop_or_default]
    pub label: Option<String>,
    pub child_renderer: Callback<FormHandleChildProps, Html>,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct FormHandleChildProps {
    pub id: String,
    pub class: Classes,
}

#[function_component(FormHandle)]
pub fn form_handle(props: &Props) -> Html {
    let label_class = classes!("mb-1", "block", "text-bcss-900", "dark:text-bcss-300");

    let label_id = use_memo((), |_| {
        Math::random()
            .to_string()
            .trim_start_matches("0.")
            .to_string()
    });

    let child = use_memo(
        ((*label_id).clone(), props.child_renderer.clone()),
        |(label_id, renderer)| {
            renderer.emit(FormHandleChildProps {
                id: label_id.clone(),
                class: classes!(
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
                ),
            })
        },
    );

    html! {
        <div class={props.container_class.clone()}>
            if let Some(label) = props.label.clone() {
                <label for={(*label_id).clone()} class={label_class}>{ label }</label>
            }
            { (*child).clone() }
        </div>
    }
}
