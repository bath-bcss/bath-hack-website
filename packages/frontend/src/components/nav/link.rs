use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum NavLinkDestination {
    Anchor(String),
    Page(String),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub dest: NavLinkDestination,
    pub label: String,

    #[prop_or_default]
    pub show_on_mobile: bool,

    pub is_at_top: bool,
}

#[function_component(NavLink)]
pub fn nav_link(props: &Props) -> Html {
    let href = {
        let dest = props.dest.clone();
        match dest {
            NavLinkDestination::Anchor(mut anchor) => {
                anchor.insert(0, '#');
                anchor
            }
            NavLinkDestination::Page(route) => route,
        }
    };

    let classes = use_memo((props.is_at_top,), |(is_at_top,)| {
        let mut base_class = classes!(
            "text-bcss-100",
            "font-medium",
            "hover:text-white",
            "hover:bg-bcss-700",
            "px-2",
            "py-2",
            "rounded-md",
            "active:bg-bcss-900/80",
            "active:ring-4",
            "focus:ring-4",
            "ring-bcss-500",
            "transition-all",
        );

        if *is_at_top {
            base_class.push(classes!("bg-bcss-600/70", "backdrop-blur"));
        }

        base_class
    });

    let paragraph_classes = use_memo((props.show_on_mobile,), |(show_on_mobile,)| {
        if *show_on_mobile {
            classes!("block")
        } else {
            classes!("hidden", "xl:block")
        }
    });

    html! {
        <p class={(*paragraph_classes).clone()}>
            <a href={href} class={(*classes).clone()}>{ props.label.clone() }</a>
        </p>
    }
}
