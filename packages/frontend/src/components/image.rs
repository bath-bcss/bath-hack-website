use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub src: String,
    #[prop_or_default]
    pub width: Option<String>,
}

#[function_component(Image)]
pub fn image(props: &Props) -> Html {
    html! {
        <img
            src={props.src.to_owned()}
            class="h-auto rounded-2xl shadow-xl shadow-bcss-900/40 dark:brightness-95"
            width={props.width.to_owned().unwrap_or("400".to_owned())}
            loading="lazy"
            role="presentation"
        />
    }
}
