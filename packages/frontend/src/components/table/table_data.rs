use yew::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(TableData)]
pub fn table_data(props: &Props) -> Html {
    html! {
    <td class="text-slate-700 dark:text-slate-200 p-4 text-center border border-bcss-300 dark:border-bcss-700">
        {props.children.clone()}
    </td>
    }
}
