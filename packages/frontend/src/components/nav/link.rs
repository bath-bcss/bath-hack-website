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
        <a href={href} class="text-bcss-200 hover:text-white">
            {props.label.clone()}
        </a>
    </p>
    }
}
