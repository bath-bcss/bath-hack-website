use yew::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(TableHeading)]
pub fn table_heading(props: &Props) -> Html {
    html! {
    <th class="p-4 font-semibold text-bcss-900 border border-bcss-300">
        {props.children.clone()}
    </th>
    }
}
