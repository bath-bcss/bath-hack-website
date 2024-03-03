use yew::prelude::*;

use crate::components::{button::Button, page_control_paragraph::PageControlParagraph};

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

    html! {
    <>
        <PageControlParagraph>
            {"You aren't currently in a group. You'll need a group of between 1 and 4 people to take part in Bath Hack.
            It's
            really easy to create or join one!"}
        </PageControlParagraph>

    <div class="mt-4 flex md:flex-row flex-col items-stretch md:items-center md:gap-x-4">
        <Button dark_mode={false}>
            {"Create a group"}
        </Button>
        <PageControlParagraph classes={classes!("text-center")}>
            {"or"}
        </PageControlParagraph>
        <Button dark_mode={false}>
            {"Join an existing group"}
        </Button>
    </div>
    </>
    }
}
