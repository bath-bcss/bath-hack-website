use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast, UnwrapThrowExt},
    window,
};
use yew::prelude::*;

use crate::components::nav::link::{NavLink, NavLinkDestination};

mod link;

#[function_component(ScrollingNavbar)]
pub fn scrolling_navbar() -> Html {
    let is_at_top_handle = use_state(|| true);
    let is_at_top = *is_at_top_handle;

    use_effect_with((), move |_| {
        let window = window().expect_throw("Window was not defined");
        let document = window
            .document()
            .expect_throw("document within Window was not defined");

        let cb = Closure::<dyn FnMut()>::new(move || {
            let scroll_y = window.scroll_y().expect_throw("getting scroll_y");
            if scroll_y >= 10.0 {
                is_at_top_handle.set(false);
            } else {
                is_at_top_handle.set(true);
            }
        });

        document
            .add_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref())
            .expect_throw("adding scroll listener");

        move || {
            document
                .remove_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref())
                .expect_throw("removing scroll listener");
        }
    });

    let container_classes = use_memo((is_at_top,), |(is_at_top,)| {
        let mut base_classes = classes!(
            "w-full",
            "h-20",
            "bg-bcss-700",
            "dark:bg-bcss-900",
            "fixed",
            "top-0",
            "flex",
            "justify-between",
            "items-center",
            "px-8",
            "z-20",
            "transition-colors",
            "drop-shadow-md",
        );

        if *is_at_top {
            base_classes.push(classes!(
                "bg-transparent",
                "dark:bg-transparent",
                "drop-shadow-none"
            ));
        }

        base_classes
    });

    html! {
        <div class={(*container_classes).clone()}>
            <div class="flex items-center justify-start space-x-3">
                <NavLink
                    dest={NavLinkDestination::Anchor("welcome".to_string())}
                    label="About"
                    is_at_top={is_at_top}
                />
                <NavLink
                    dest={NavLinkDestination::Anchor("tracks".to_string())}
                    label="Tracks"
                    is_at_top={is_at_top}
                />
                <NavLink
                    dest={NavLinkDestination::Anchor("schedule".to_string())}
                    label="Schedule"
                    is_at_top={is_at_top}
                />
                <NavLink
                    dest={NavLinkDestination::Anchor("sponsor".to_string())}
                    label="Sponsors"
                    is_at_top={is_at_top}
                />
                <NavLink
                    dest={NavLinkDestination::Anchor("faqs".to_string())}
                    label="FAQs"
                    is_at_top={is_at_top}
                />
                <NavLink
                    dest={NavLinkDestination::Anchor("accessibility".to_string())}
                    label="Accessibility"
                    is_at_top={is_at_top}
                />
                <NavLink
                    dest={NavLinkDestination::Anchor("about".to_string())}
                    label="About WiT"
                    is_at_top={is_at_top}
                />
            </div>
            <div class="flex items-center justify-start space-x-3">
                <NavLink
                    dest={NavLinkDestination::Page("/login".to_string())}
                    label="Log in"
                    show_on_mobile=true
                    is_at_top={is_at_top}
                />
                <NavLink
                    dest={NavLinkDestination::Page("/signup".to_string())}
                    label="Sign up"
                    show_on_mobile=true
                    is_at_top={is_at_top}
                />
            </div>
        </div>
    }
}
