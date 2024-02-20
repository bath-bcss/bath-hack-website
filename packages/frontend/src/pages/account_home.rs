use bhw_types::requests::{profile::ProfileResponse, update_profile::UpdateProfileRequest};
use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    components::{
        input::Input,
        page_container::PageContainer,
        page_title::PageTitle,
        profile_datapoint::{ProfileDatapoint, ProfileKey},
    },
    data::{auth::check_signed_in, profile::get_profile},
    redirect_if_not_authed,
    router::Route,
};

#[function_component(AccountHomePage)]
pub fn account_home_page() -> Html {
    let profile_handle = use_state_eq(|| None::<ProfileResponse>);
    let profile = (*profile_handle).clone();

    let error_handle = use_state_eq(|| None::<String>);
    let error = (*error_handle).clone();

    let loading_handle = use_state_eq(|| false);
    let loading = (*loading_handle).clone();

    let navigator = use_navigator().expect_throw("Navigator not found");
    {
        let navigator = navigator.clone();
        let profile_handle = profile_handle.clone();
        use_effect_with((), |_| {
            redirect_if_not_authed!(navigator);

            wasm_bindgen_futures::spawn_local(async move {
                loading_handle.set(true);
                let response = get_profile().await;
                loading_handle.set(false);

                match response {
                    Err(e) => error_handle.set(Some(e.to_string())),
                    Ok(d) => profile_handle.set(Some(d)),
                }
            });
        });
    }

    let on_datapoint_change = {
        let profile_handle = profile_handle.clone();
        use_callback(
            (profile.clone(),),
            move |req: UpdateProfileRequest, (profile,)| {
                let profile = (*profile).clone();
                if let None = profile {
                    return;
                }

                let mut profile = profile.expect_throw("bruh");

                match req {
                    UpdateProfileRequest::DisplayName(d) => profile.display_name = d,
                    UpdateProfileRequest::AccessibilityRequirements(a) => {
                        profile.accessibility_requirements = a
                    }
                }

                profile_handle.set(Some(profile));
            },
        )
    };

    html! {
    <PageContainer>
        <PageTitle page_description="Manage your group membership and profile details">
            {"Your details"}
        </PageTitle>

        if let Some(profile) = profile {
        <div class="space-y-4 mt-4">
            <Input static_value={profile.bath_username} readonly={true} input_label="Bath Username" />

            <ProfileDatapoint data_key={ProfileKey::DisplayName} current_value={profile.display_name}
                on_value_change={on_datapoint_change.clone()} />
            <ProfileDatapoint data_key={ProfileKey::AccessibilityRequirements}
                current_value={profile.accessibility_requirements} on_value_change={on_datapoint_change.clone()} />
        </div>
        }
    </PageContainer>
    }
}
