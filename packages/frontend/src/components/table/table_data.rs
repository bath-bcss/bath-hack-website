use yew::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(TableData)]
pub fn table_data(props: &Props) -> Html {
    html! {
    <td class="text-slate-700 p-4 text-center border border-bcss-300">
        {props.children.clone()}
    </td>
    }
}
