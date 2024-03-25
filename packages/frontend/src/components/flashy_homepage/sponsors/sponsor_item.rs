use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Props {
    pub logo_url: String,
    pub name: String,
    #[prop_or_default]
    pub url: Option<String>,
}

#[function_component(SponsorItem)]
pub fn sponsor_item(props: &Props) -> Html {
    html! {
        <a href={props.url.clone()} target="_blank" class="block w-48">
            <img src={props.logo_url.clone()} alt={props.name.clone()} loading="lazy" />
        </a>
    }
}
