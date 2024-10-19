use yew::prelude::*;

use crate::components::modal::{modal_header::ModalHeader, Modal};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub event: String,
    pub time: String,
    #[prop_or_default]
    pub is_first: bool,
    #[prop_or_default]
    pub is_last: bool,
    #[prop_or_default]
    pub day: Option<String>,
    pub children: Html,
}

#[function_component(ScheduleItem)]
pub fn schedule_item(props: &Props) -> Html {
    let modal_open_handle = use_state(|| false);
    let modal_open = *modal_open_handle;

    let on_open_modal_click = use_callback(
        (modal_open_handle.clone(),),
        |e: MouseEvent, (modal_open_handle,)| {
            e.prevent_default();
            modal_open_handle.set(true);
        },
    );

    let on_close_modal_click = use_callback(
        (modal_open_handle.clone(),),
        |e: MouseEvent, (modal_open_handle,)| {
            e.prevent_default();
            modal_open_handle.set(false);
        },
    );

    html! {
        <>
            <li class="flex items-start flex-col sm:flex-row">
                <div class="w-36 pt-1">
                    if let Some(day) = props.day.clone() {
                        <p class="font-regular text-bcss-800 dark:text-bcss-300 mb-1 sm:mb-0">{ day }</p>
                    }
                </div>
                <button
                    class="w-full bg-bcss-100 dark:bg-bcss-800 rounded-2xl hover:-translate-y-[2px] transition-all motion-reduce:hover:translate-y-0 hover:shadow-md hover:shadow-bcss-400/50 dark:hover:shadow-bcss-700/50 text-left flex overflow-hidden"
                    onclick={on_open_modal_click}
                >
                    <div class="ml-4">
                        <div class="h-full relative">
                            if !props.is_first {
                                <div
                                    class="absolute top-0 h-1/2 bg-bcss-300 dark:bg-bcss-400 w-[1px]"
                                />
                            }
                            <div class="absolute top-0 -left-1 h-full flex items-center z-10">
                                <div class="w-2 h-2 bg-bcss-400 dark:bg-bcss-600 rounded-full" />
                            </div>
                            if !props.is_last {
                                <div
                                    class="absolute top-1/2 h-1/2 bg-bcss-300 dark:bg-bcss-400 w-[1px]"
                                />
                            }
                        </div>
                    </div>
                    <div class="ml-4 py-2 pr-4 flex-1">
                        <h3
                            class="text-lg text-bcss-900 dark:text-bcss-200 font-medium leading-none"
                        >
                            { props.event.to_string() }
                        </h3>
                        <p
                            class="font-light text-bcss-800 dark:text-bcss-200 text-sm leading-none"
                        >
                            { props.time.clone() }
                        </p>
                    </div>
                </button>
            </li>
            <Modal open={modal_open}>
                <ModalHeader
                    title={props.event.clone()}
                    subtitle={props.time.clone()}
                    onclose={on_close_modal_click}
                />
                <p class="mt-4 text-bcss-700 dark:text-bcss-300">{ props.children.clone() }</p>
            </Modal>
        </>
    }
}
