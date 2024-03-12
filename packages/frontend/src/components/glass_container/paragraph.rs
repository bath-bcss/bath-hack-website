use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Html,
    #[prop_or_default]
    pub top_margin: bool,
}

#[function_component(GlassContainerParagraph)]
pub fn glass_container_paragraph(props: &Props) -> Html {
    let p_class = use_memo((props.top_margin,), |(top_margin,)| {
        let mut base_classes = classes!("dark:text-bcss-300", "text-bcss-900");
        if top_margin.clone() {
            base_classes.push("mt-4");
        }

        base_classes
    });

    html! { <p class={(*p_class).clone()}>{ props.children.clone() }</p> }
}
