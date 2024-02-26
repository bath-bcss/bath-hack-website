use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::{button::Button, modal::Modal};

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
    pub track_name: String,
    pub track_caption: String,
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
        <div
            class="w-full h-40 relative rounded-2xl overflow-hidden shadow-bcss-900/40 shadow-xl hover:scale-105 transition-transform">
            <div style={format!("background-image: url('{}');", props.background_image.clone())}
                class="absolute top-0 left-0 w-full h-full bg-bcss-200 brightness-75 blur-sm" />
            <a class="absolute top-0 left-0 w-full h-full flex justify-center items-center text-center text-2xl text-white font-bold drop-shadow cursor-pointer"
                onclick={on_open_click}>
                {props.track_name.clone()}
            </a>
        </div>

        <Modal open={show_modal}>
            <div class="flex items-center justify-between">
                <h1 class="text-bcss-800 dark:text-bcss-200 font-bold text-3xl">
                    {props.track_name.clone()}
                </h1>
                <Button onclick={on_close_click} dark_mode={false}>
                    <Icon icon_id={IconId::FontAwesomeSolidCircleXmark} />
                </Button>
            </div>

            <p class="text-bcss-700 dark:text-bcss-300 mt-4">
                {props.track_caption.clone()}
            </p>
        </Modal>
    </>
    }
}
