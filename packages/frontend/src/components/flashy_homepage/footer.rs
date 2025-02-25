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
                    { "Photos by Joshua Dale (IG: " }
                    <a href="https://instagram.com/crafty_jj" target="_blank" class="underline">
                        { "@crafty_jj" }
                    </a>
                    { ") and Hayden Fernandes (IG: " }
                    <a href="https://instagram.com/heyhayden02" target="_blank" class="underline">
                        { "@heyhayden02" }
                    </a>
                    { ") from Bath Hack 2024." }
                </p>
                <p class="text-bcss-800 dark:text-bcss-200">
                    { "Some images, text, and other assets © BCSS Committee 2024/25, 2023/24, and 2022/23." }
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
                    { "Please contact " }
                    <a href="mailto:su-bcss@bath.ac.uk" class="underline">
                        { "su-bcss@bath.ac.uk" }
                    </a>
                    { " for support." }
                </p>
            </div>
        </footer>
    }
}
