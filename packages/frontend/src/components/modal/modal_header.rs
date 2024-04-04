use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::button::Button;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub title: String,
    #[prop_or_default]
    pub subtitle: Option<String>,
    #[prop_or_default]
    pub subtitle_link: Option<String>,
    pub onclose: Callback<MouseEvent>,
}

#[function_component(ModalHeader)]
pub fn modal_header(props: &Props) -> Html {
    html! {
        <div class="flex items-center justify-between space-x-6">
            <div>
                <h1 class="text-bcss-800 dark:text-bcss-200 font-bold text-3xl">
                    { props.title.clone() }
                </h1>
                if let Some(subtitle) = props.subtitle.clone() {
                    <h2 class="text-bcss-600 dark:text-bcss-300 text-xl">
                        if let Some(subtitle_link) = props.subtitle_link.clone() {
                            <a href={subtitle_link} target="_blank" class="underline">
                                { subtitle }
                            </a>
                        } else {
                            { subtitle }
                        }
                    </h2>
                }
            </div>
            <Button onclick={props.onclose.clone()} background_is_dark=false>
                <Icon icon_id={IconId::FontAwesomeSolidCircleXmark} />
            </Button>
        </div>
    }
}
