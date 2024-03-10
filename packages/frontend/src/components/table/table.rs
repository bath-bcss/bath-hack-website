use yew::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(Table)]
pub fn table_root(props: &Props) -> Html {
    html! {
    <table class="w-full border border-bcss-300 dark:border-bcss-700">
        {props.children.clone()}
    </table>
    }
}
