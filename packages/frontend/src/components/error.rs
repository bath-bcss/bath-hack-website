use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub message: Option<String>,
}

#[function_component(ErrorMessage)]
pub fn error_message(props: &Props) -> Html {
    let message = props.message.clone();

    html! {
        if let Some(message) = message {
            <p class="text-red-600 dark:text-red-300 mt-4">{ message }</p>
        }
    }
}
