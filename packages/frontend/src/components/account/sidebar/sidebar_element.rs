use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::{components::Link, hooks::use_route};

use crate::router::AccountRoute;

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub link_to: AccountRoute,
    pub icon: IconId,
    pub label: String,
}

#[function_component(SidebarElement)]
pub fn sidebar_element(props: &Props) -> Html {
    let route = use_route::<AccountRoute>();

    let link_classes = use_memo((route, props.link_to.clone()), |(route, link_to)| {
        let mut base_classes = classes!(
            "block",
            "w-full",
            "h-10",
            "flex",
            "items-center",
            "px-4",
            "rounded-md",
            "font-medium",
            "hover:bg-bcss-800",
            "dark:hover:bg-bcss-800",
            "text-white",
        );

        if let Some(route) = route {
            if route == link_to {
                base_classes.push(classes!("bg-bcss-800", "dark:bg-bcss-800"));
            }
        }

        base_classes
    });

    html! {
        <p>
            <Link<AccountRoute> to={props.link_to.clone()} classes={(*link_classes).clone()}>
                <Icon
                    icon_id={props.icon}
                    class={classes!("h-full", "w-5" , "mr-3" )}
                    role="presentation"
                />
                { props.label.clone() }
            </Link<AccountRoute>>
        </p>
    }
}
