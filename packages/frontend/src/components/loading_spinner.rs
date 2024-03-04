use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Option<Classes>,
}

#[function_component(LoadingSpinner)]
pub fn loading_spinner(props: &Props) -> Html {
    let classes = use_memo((props.class.clone(),), |(classes,)| {
        let mut base_classes = classes!(
            "animate-spin",
            "text-bcss-700",
            "dark:text-bcss-300",
            "h-12",
            "w-12"
        );

        if let Some(classes) = classes {
            base_classes.push(classes.clone())
        }

        base_classes
    });

    html! {
    <Icon icon_id={IconId::FontAwesomeSolidSpinner} class={(*classes).clone()} />
    }
}
