use yew::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(TableHeading)]
pub fn table_heading(props: &Props) -> Html {
    html! {
    <th class="p-4 font-semibold text-bcss-900 dark:text-bcss-200 border border-bcss-300 dark:border-bcss-700">
        {props.children.clone()}
    </th>
    }
}
