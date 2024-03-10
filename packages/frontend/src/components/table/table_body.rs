use yew::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(TableBody)]
pub fn table_body(props: &Props) -> Html {
    html! {
    <tbody>
        {props.children.clone()}
    </tbody>
    }
}
