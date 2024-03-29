use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::{button::Button, modal::Modal};

#[derive(Debug, PartialEq, Clone)]
pub struct TrackCompany {
    pub name: String,
    pub link: String,
}

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
    pub track_name: String,
    pub track_caption: String,
    #[prop_or_default]
    pub track_prize: Option<String>,
    #[prop_or_default]
    pub track_company: Option<TrackCompany>,
    pub background_image: String,
}

#[function_component(TrackGridItem)]
pub fn track_grid_item(props: &Props) -> Html {
    let modal_handle = use_state(|| false);
    let show_modal = (*modal_handle).clone();

    let on_open_click = use_callback((modal_handle.clone(),), |_, (modal_handle,)| {
        modal_handle.set(true);
    });

    let on_close_click = use_callback((modal_handle,), |_, (modal_handle,)| {
        modal_handle.set(false);
    });

    html! {
        <>
            <li
                class="w-full h-40 relative rounded-2xl overflow-hidden shadow-bcss-900/40 shadow-xl hover:scale-105 transition-transform"
            >
                <div
                    style={format!("background-image: url('{}');", props.background_image.clone())}
                    class="absolute top-0 left-0 w-full h-full bg-bcss-200 bg-cover bg-center brightness-[0.6] blur-sm"
                />
                <a
                    class="absolute top-0 left-0 w-full h-full flex flex-col justify-center items-center text-center drop-shadow cursor-pointer px-2"
                    onclick={on_open_click}
                >
                    <span class="text-2xl text-white font-bold leading-none">
                        { props.track_name.clone() }
                    </span>
                    if let Some(track_company) = props.track_company.clone() {
                        <span class="text-gray-200">
                            { "from " }
                            <strong>{ track_company.name }</strong>
                        </span>
                    }
                </a>
            </li>
            <Modal open={show_modal}>
                <div class="flex items-center justify-between space-x-6">
                    <div>
                        <h1 class="text-bcss-800 dark:text-bcss-200 font-bold text-3xl">
                            { props.track_name.clone() }
                        </h1>
                        if let Some(track_company) = props.track_company.clone() {
                            <h2 class="text-bcss-600 dark:text-bcss-300 text-xl">
                                { "from " }
                                <a href={track_company.link} target="_blank" class="underline">
                                    { track_company.name }
                                </a>
                            </h2>
                        }
                    </div>
                    <Button onclick={on_close_click} dark_mode=false>
                        <Icon icon_id={IconId::FontAwesomeSolidCircleXmark} />
                    </Button>
                </div>
                <p class="text-bcss-700 dark:text-bcss-300 mt-4">{ props.track_caption.clone() }</p>
                if let Some(track_prize) = props.track_prize.clone() {
                    <h1
                        class="mt-4 text-bcss-800 dark:text-bcss-200 font-bold text-xl align-middle"
                    >
                        <Icon icon_id={IconId::FontAwesomeSolidTrophy} class="inline-block mr-2" />
                        { "Prize" }
                    </h1>
                    <p class="text-bcss-700 dark:text-bcss-300 mt-2">{ track_prize }</p>
                }
            </Modal>
        </>
    }
}
