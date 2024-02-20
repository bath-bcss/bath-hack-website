use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub category_label: Option<String>,
    pub children: Html,
    #[prop_or_default]
    pub page_description: Option<String>,
}

#[function_component(PageTitle)]
pub fn page_title(props: &Props) -> Html {
    html! {
    <>
        if let Some(category_label) = props.category_label.clone() {
        <p class="mb-3 text-bcss-900 tracking-wide">
            {category_label}
        </p>
        }
        <h1 class="font-bold text-3xl dark:text-bcss-200 tracking-tighter">
            {props.children.clone()}
        </h1>
        if let Some(page_description) = props.page_description.clone() {
        <h2 class="text-gray-500 dark:text-bcss-400 font-medium text-xl">
            {page_description}
        </h2>
        }
    </>
    }
}
