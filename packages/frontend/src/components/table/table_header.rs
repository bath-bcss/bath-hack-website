use yew::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(TableHeader)]
pub fn table_header(props: &Props) -> Html {
    html! {
    <thead class="bg-bcss-100">
        {props.children.clone()}
    </thead>
    }
}
