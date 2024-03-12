use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Html,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

#[function_component(PageControlParagraph)]
pub fn page_control_paragraph(props: &Props) -> Html {
    let classes = use_memo((props.classes.clone(),), |(classes,)| {
        let mut base_classes = classes!("my-2", "text-bcss-900", "dark:text-bcss-300");

        if let Some(classes) = classes {
            base_classes.push(classes.clone());
        }

        base_classes
    });

    html! { <p class={(*classes).clone()}>{ props.children.clone() }</p> }
}
