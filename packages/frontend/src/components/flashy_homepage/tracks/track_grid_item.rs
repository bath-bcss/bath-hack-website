use yew::prelude::*;

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
    pub track_name: String,
    pub background_image: String,
}

#[function_component(TrackGridItem)]
pub fn track_grid_item(props: &Props) -> Html {
    html! {
    <div class="w-full h-40 relative rounded-2xl overflow-hidden shadow-bcss-900/40 shadow-xl">
        <div style={format!("background-image: url('{}');", props.background_image.clone())}
            class="absolute top-0 left-0 w-full h-full bg-bcss-200 brightness-75 blur-sm" />
        <a
            class="absolute top-0 left-0 w-full h-full flex justify-center items-center text-center text-2xl text-white font-bold drop-shadow">
            {props.track_name.clone()}
        </a>
    </div>
    }
}
