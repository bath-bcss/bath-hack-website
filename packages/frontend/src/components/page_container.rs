use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(PageContainer)]
pub fn page_container(props: &Props) -> Html {
    html! {
        <div class="flex flex-col items-center mt-16 md:mt-12">
            <div class="max-w-7xl mx-auto w-full md:px-8 px-4">{ props.children.clone() }</div>
        </div>
    }
}
