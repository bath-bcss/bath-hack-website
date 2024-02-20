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
}

#[function_component(NavLink)]
pub fn nav_link(props: &Props) -> Html {
    let href = {
        let dest = props.dest.clone();
        match dest {
            NavLinkDestination::Anchor(mut anchor) => {
                anchor.insert_str(0, "#");
                anchor
            }
            NavLinkDestination::Page(route) => route,
        }
    };

    html! {
    <p>
        <a href={href}
            class="text-bcss-200 hover:text-white hover:bg-bcss-800 px-2 py-2 rounded-md active:bg-bcss-900/80 active:ring-4 focus:ring-4 ring-bcss-500 transition-all">
            {props.label.clone()}
        </a>
    </p>
    }
}
