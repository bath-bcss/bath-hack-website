use yew::prelude::*;

#[function_component(HomepageFooter)]
pub fn homepage_footer() -> Html {
    html! {
        <footer class="mt-24 bg-bcss-100 dark:bg-bcss-800 w-full rounded-t-3xl">
            <div class="max-w-7xl mx-auto py-14 px-2 md:px-4 lg:px-8">
                <p class="text-bcss-800 dark:text-bcss-200">
                    { "Website copyright © Pal Kerecsenyi 2024-25; licensed under GNU GPL 3.0." }
                </p>
                <p class="text-bcss-800 dark:text-bcss-200">
                    { "Images, some text, and other assets © WiT Committee 2024/25 and 2023/24." }
                </p>
                <p class="text-bcss-800 dark:text-bcss-200">
                    { "Source code and license details available on " }
                    <a
                        href="https://github.com/bath-bcss/bath-hack-website"
                        target="_blank"
                        class="underline"
                    >
                        { "GitHub" }
                    </a>
                    { "." }
                </p>
                <p class="text-bcss-800 dark:text-bcss-200">
                    { "Contact " }
                    <a
                        href="mailto:wit@bath.ac.uk"
                        class="underline"
                    >
                        { "wit@bath.ac.uk" }
                    </a>
                    { " for support." }
                </p>
            </div>
        </footer>
    }
}
