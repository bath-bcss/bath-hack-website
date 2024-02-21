use yew::prelude::*;

#[function_component(HomepageFooter)]
pub fn homepage_footer() -> Html {
    html! {
    <footer class="mt-24 bg-bcss-100 w-full rounded-t-3xl">
        <div class="max-w-7xl mx-auto py-14 px-2 md:px-4 lg:px-8">

            <p class="text-bcss-800">
                {"Website copyright © Pal Kerecsenyi 2024; licensed under GNU GPL 3.0."}
            </p>
            <p class="text-bcss-800">
                {"Images and other assets © BCSS Committee 2023-2024."}
            </p>
            <p class="text-bcss-800">
                {"Source code and license details available on "}
                <a href="https://github.com/bath-bcss/bath-hack-website" target="_blank" class="underline">
                    {"GitHub"}
                </a>{"."}
            </p>
            <p class="text-bcss-800">
                {"Contact su-bcss@bath.ac.uk for support."}
            </p>
        </div>
    </footer>
    }
}
