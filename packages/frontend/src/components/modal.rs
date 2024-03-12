use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Html,
    pub open: bool,
}

#[function_component(Modal)]
pub fn modal_component(props: &Props) -> Html {
    let host = gloo_utils::document()
        .get_element_by_id("modal_host")
        .expect("Did not find #modal_host element");

    use_effect_with((props.open,), |(open,)| {
        let body = gloo_utils::document().body().expect("Did not find body");

        if *open {
            body.set_id("no-scroll-body");
        } else {
            body.set_id("");
        }
    });

    if !props.open {
        return html! {};
    }

    create_portal(
        html! {
            <div
                class="fixed top-0 left-0 right-0 bottom-0 bg-black/60 backdrop-blur-sm z-50 flex justify-center items-center p-4 md:p-8"
            >
                <div
                    class="bg-bcss-50 dark:bg-bcss-950 px-8 md:px-16 py-10 md:max-w-3xl max-h-full overflow-auto rounded-2xl shadow-bcss-100 dark:shadow-bcss-900/70 shadow-lg"
                >
                    { props.children.clone() }
                </div>
            </div>
        },
        host.into(),
    )
}
