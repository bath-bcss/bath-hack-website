use yew::prelude::*;

// use crate::components::flashy_homepage::sponsors::sponsor_item::SponsorItem;

mod sponsor_item;

#[function_component(SponsorsGrid)]
pub fn sponsors_grid() -> Html {
    html! {
        <div class="mt-8 grid grid-cols-1 md:grid-cols-2 gap-x-6 gap-y-12" role="list">
            /*<SponsorItem name="PalSponsor" logo_url="img/sponsor_pal.svg">
            {"Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat."}
        </SponsorItem>*/<p class="text-bcss-700 dark:text-bcss-300" role="listitem">
                { "List of sponsors coming soon..." }
            </p>
        </div>
    }
}
