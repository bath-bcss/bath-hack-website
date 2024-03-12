use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::{
    components::{button::Button, page_control_paragraph::PageControlParagraph},
    pages::account::account_group::joining::{create_group::CreateGroup, join_group::JoinGroup},
};

use super::types::AccountGroupSubpageProps;

#[derive(Debug, Clone, PartialEq)]
enum GroupMethod {
    Join,
    Create,
}

#[function_component(AccountGroupJoining)]
pub fn account_group_joining(props: &AccountGroupSubpageProps) -> Html {
    let group_method_handle = use_state_eq(|| None::<GroupMethod>);
    let group_method = (*group_method_handle).clone();

    let on_create_click = use_callback(
        (group_method_handle.clone(),),
        |_, (group_method_handle,)| group_method_handle.set(Some(GroupMethod::Create)),
    );
    let on_join_click = use_callback(
        (group_method_handle.clone(),),
        |_, (group_method_handle,)| group_method_handle.set(Some(GroupMethod::Join)),
    );
    let on_method_deselect = use_callback((group_method_handle,), |_, (group_method_handle,)| {
        group_method_handle.set(None);
    });

    html! {
        <>
            if let Some(group_method) = group_method {
                <Button onclick={on_method_deselect} dark_mode=false class={classes!("mt-4")}>
                    <Icon
                        icon_id={IconId::FontAwesomeSolidArrowLeft}
                        class={classes!("inline-block")}
                    />
                    { " Back" }
                </Button>
                if group_method == GroupMethod::Create {
                    <CreateGroup group_handle={props.group_handle.clone()} />
                } else if group_method == GroupMethod::Join {
                    <JoinGroup group_handle={props.group_handle.clone()} />
                }
            } else {
                <PageControlParagraph>
                    { "You aren't currently in a group. You'll need a group of between 1 and 4 people to take part in Bath Hack.
            It's
            really easy to create or join one!" }
                </PageControlParagraph>
                <div
                    class="mt-4 flex md:flex-row flex-col items-stretch md:items-center md:gap-x-4"
                >
                    <Button dark_mode=false onclick={on_create_click}>{ "Create a group" }</Button>
                    <PageControlParagraph classes={classes!("text-center")}>
                        { "or" }
                    </PageControlParagraph>
                    <Button dark_mode=false onclick={on_join_click}>
                        { "Join an existing group" }
                    </Button>
                </div>
            }
        </>
    }
}
