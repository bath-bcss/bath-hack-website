use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

use crate::{
    components::{
        button::Button, page_control_heading::PageControlHeading,
        page_control_paragraph::PageControlParagraph,
    },
    pages::account::account_group::member_card::member_card,
};

use super::types::AccountGroupSubpageProps;

#[function_component(AccountGroupManage)]
pub fn account_group_manage(props: &AccountGroupSubpageProps) -> Html {
    let current_group = (*props.group_handle).clone().expect_throw(
        "Cannot use AccountGroupManage with a None value for
group_handle.",
    );

    html! {
    <>
        <PageControlHeading>
            {"Current group"}
        </PageControlHeading>
        <PageControlParagraph>
            {"You're currently a member of a group, which you either created or joined with a code."}
        </PageControlParagraph>

        <div class="mt-4 py-6 px-8 bg-bcss-50 shadow-md shadow-bcss-200 rounded-2xl inline-block">
            <p class="text-xl font-bold text-bcss-800">
                {current_group.name}
            </p>
            <p class="mt-1 text-bcss-900">
                {"Join code: "}
                <code>{current_group.join_code}</code>
            </p>

            <div class="mt-4 flex items-start gap-x-4">
                {current_group.members.into_iter().map(|m| member_card(m)).collect::<Html>()}
            </div>

            <Button class={classes!("mt-4")} dark_mode={false}>
                {"Leave group"}
            </Button>
        </div>
    </>
    }
}
