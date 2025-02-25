use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::{
    components::Link,
    hooks::{use_navigator, use_route},
};

use crate::{
    components::{account::sidebar::sidebar_element::SidebarElement, button::Button},
    redirect_if_not_authed,
    router::{AccountRoute, Route},
};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Html,
}

#[function_component(AccountSidebar)]
pub fn account_sidebar(props: &Props) -> Html {
    let navigator = use_navigator().expect_throw("Navigator not found");
    use_effect_with((), move |_| {
        redirect_if_not_authed!(navigator);
    });

    let small_screen_show_handle = use_state_eq(|| false);
    let small_screen_show = *small_screen_show_handle;

    let sidebar_container_classes = use_memo((small_screen_show,), |(small_screen_show,)| {
        let mut base_classes = classes!(
            "w-full",
            "md:w-60",
            "lg:w-80",
            "h-screen",
            "py-8",
            "px-4",
            "rounded-r-2xl",
            "z-10",
            "md:!transform-none",
            "transition-transform",
            "motion-reduce:transition-none",
            "md:transition-none",
            "fixed",
            "md:static",
            "md:mr-0",
            "backdrop-blur",
            "md:backdrop-blur-none",
            "bg-bcss-700/70",
            "md:bg-bcss-700",
            "dark:bg-bcss-900/70",
            "dark:md:bg-bcss-900"
        );

        if *small_screen_show {
            base_classes.push(classes!("!mr-4"));
        } else {
            base_classes.push("-translate-x-full");
        }

        base_classes
    });

    let on_small_screen_close_click = use_callback(
        (small_screen_show_handle.clone(),),
        move |_: MouseEvent, (small_screen_show_handle,)| {
            small_screen_show_handle.set(false);
        },
    );

    let on_small_screen_show_click = use_callback(
        (small_screen_show_handle.clone(),),
        move |_: MouseEvent, (small_screen_show_handle,)| {
            small_screen_show_handle.set(true);
        },
    );

    let route = use_route::<AccountRoute>();
    use_effect_with((route,), move |_| {
        small_screen_show_handle.set(false);
    });

    html! {
        <div class="flex w-full">
            <div class={(*sidebar_container_classes).clone()}>
                <div class="flex items-center justify-between mb-2 md:mb-0">
                    <h1 class="text-3xl text-white dark:text-bcss-200 font-bold tracking-tighter">
                        <Link<Route> to={Route::Home} classes={classes!("hover:underline")}>
                            { "Bath Hack 25" }
                        </Link<Route>>
                    </h1>
                    <Button
                        background_is_dark=true
                        onclick={on_small_screen_close_click}
                        class={classes!("md:hidden")}
                    >
                        <Icon icon_id={IconId::FontAwesomeSolidCircleXmark} />
                    </Button>
                </div>
                <p>
                    <Link<Route>
                        to={Route::Logout}
                        classes={classes!("text-white", "hover:underline")}
                    >
                        { "Log out" }
                    </Link<Route>>
                </p>
                <div class="space-y-2 mt-4">
                    <SidebarElement
                        link_to={AccountRoute::Profile}
                        label="Profile"
                        icon={IconId::FontAwesomeSolidUser}
                    />
                    <SidebarElement
                        link_to={AccountRoute::Groups}
                        label="Group"
                        icon={IconId::FontAwesomeSolidPeopleGroup}
                    />
                    <SidebarElement
                        link_to={AccountRoute::CV}
                        label="CV"
                        icon={IconId::FontAwesomeSolidFileContract}
                    />
                </div>
            </div>
            <div class="flex-1 pb-10">
                if !small_screen_show {
                    <div class="md:hidden fixed pt-4 pl-4 z-0">
                        <Button background_is_dark=false onclick={on_small_screen_show_click}>
                            <Icon icon_id={IconId::FontAwesomeSolidBars} />
                        </Button>
                    </div>
                }
                { props.children.clone() }
            </div>
        </div>
    }
}
