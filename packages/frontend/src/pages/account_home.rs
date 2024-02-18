use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    components::{page_container::PageContainer, page_title::PageTitle},
    data::auth::check_signed_in,
    redirect_if_not_authed,
    router::Route,
};

#[function_component(AccountHomePage)]
pub fn account_home_page() -> Html {
    let navigator = use_navigator().expect_throw("Navigator not found");
    {
        let navigator = navigator.clone();
        use_effect_with((), |_| {
            redirect_if_not_authed!(navigator);
        });
    }

    html! {
    <PageContainer>
        <PageTitle page_description="Manage your group membership and profile details">
            {"Your details"}
        </PageTitle>
    </PageContainer>
    }
}
