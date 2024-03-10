use yew::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub children: Html,
    #[prop_or_default]
    pub class: Option<Classes>,
}

#[function_component(ResponsiveTableContainer)]
pub fn responsive_table_container(props: &Props) -> Html {
    html! {
    <div class={classes!("w-full", "overflow-x-auto", props.class.clone())}>
        {props.children.clone()}
    </div>
    }
}
