use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(HeroHeader)]
pub fn hero_header(props: &Props) -> Html {
    html! {
    <div class="h-screen w-full bg-gradient-to-br from-purple-800 dark:from-purple-950 to-bcss-700 dark:to-bcss-900 flex justify-center items-center p-8">
        <div class="flex flex-col justify-center space-y-4">
            { props.children.clone() }
        </div>
    </div>
    }
}
