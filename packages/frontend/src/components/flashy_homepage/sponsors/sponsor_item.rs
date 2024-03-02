use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Props {
    pub logo_url: String,
    pub name: String,
    pub children: Html,
}

#[function_component(SponsorItem)]
pub fn sponsor_item(props: &Props) -> Html {
    html! {
    <div class="flex items-center gap-x-8">
        <img src={props.logo_url.clone()} alt="Sponsor logo" class="h-20" />
        <div class="flex-1">
            <h3 class="text-bcss-900 dark:text-bcss-200 font-medium text-2xl">
                {props.name.clone()}
            </h3>
            <p class="text-bcss-800 dark:text-bcss-300">
                {props.children.clone()}
            </p>
        </div>
    </div>
    }
}
