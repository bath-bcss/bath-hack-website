use yew::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(TableRow)]
pub fn table_row(props: &Props) -> Html {
    html! {
    <tr>
        {props.children.clone()}
    </tr>
    }
}
