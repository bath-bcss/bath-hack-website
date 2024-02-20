use yew::prelude::*;
use yew_icons::IconId;

use crate::{components::sidebar::sidebar_element::SidebarElement, router::AccountRoute};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Html,
}

#[function_component(AccountSidebar)]
pub fn account_sidebar(props: &Props) -> Html {
    html! {
    <div class="flex w-full">
        <div class="w-80 h-screen min-h-full bg-bcss-600 py-8 px-4 rounded-r-2xl">
            <h1 class="text-3xl text-white font-bold tracking-tighter">
                {"Bath Hack 24"}
            </h1>

            <div class="space-y-2 mt-4">
                <SidebarElement link_to={AccountRoute::Profile} label="Profile" icon={IconId::FontAwesomeSolidUser} />
                <SidebarElement link_to={AccountRoute::Groups} label="Group" icon={IconId::FontAwesomeSolidPeopleGroup} />
            </div>
        </div>

        <div class="flex-1">
            {props.children.clone()}
        </div>
    </div>
    }
}
