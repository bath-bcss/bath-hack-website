use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    components::{page_container::PageContainer, page_title::PageTitle},
    redirect_if_not_authed,
};

#[function_component(AccountGroupPage)]
pub fn account_group_page() -> Html {
    let navigator = use_navigator().expect_throw("Navigator not found");
    use_effect_with((), move |_| {
        let navigator = navigator.clone();
        redirect_if_not_authed!(navigator);
    });

    html! {
    <PageContainer>
        <PageTitle page_description="Manage your group membership">
            {"Your group"}
        </PageTitle>
    </PageContainer>
    }
}
