use bhw_types::requests::{profile::ProfileResponse, update_profile::UpdateProfileRequest};
use gloo_console::error;
use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::hooks::use_location;

use crate::{
    components::{
        input::Input,
        loading_spinner::LoadingSpinner,
        page_container::PageContainer,
        page_title::PageTitle,
        profile_datapoint::{ProfileDatapoint, ProfileKey},
    },
    data::profile::get_profile,
};

pub struct InitialSignupState;

#[function_component(AccountHomePage)]
pub fn account_home_page() -> Html {
    let profile_handle = use_state_eq(|| None::<ProfileResponse>);
    let profile = (*profile_handle).clone();

    let loading_handle = use_state_eq(|| false);
    let loading = (*loading_handle).clone();

    let is_initial_signup = use_location()
        .expect_throw("Location not found")
        .state::<InitialSignupState>()
        .is_some();

    {
        let profile_handle = profile_handle.clone();
        use_effect_with((), |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading_handle.set(true);
                let response = get_profile().await;
                loading_handle.set(false);

                match response {
                    Err(e) => error!("getting profile: ", e.to_string()),
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
                    UpdateProfileRequest::AccessibilityRequirements(d) => {
                        profile.accessibility_requirements = d
                    }
                    UpdateProfileRequest::DietaryRequirements(d) => {
                        profile.dietary_requirements = d
                    }
                }

                profile_handle.set(Some(profile));
            },
        )
    };

    html! {
    <PageContainer>
        if is_initial_signup {
        <div
            class="p-4 bg-green-200 dark:bg-green-700 shadow-lg dark:shadow-md shadow-green-100 dark:shadow-green-800 rounded-2xl mb-4">
            <h2 class="text-green-900 dark:text-green-200 font-bold text-xl">
                {"You made it!"}
            </h2>
            <p class="text-green-800 dark:text-green-200">
                {"That's it; you're officially going to Bath Hack! If you feel like it, you can fill out the rest of
                your profile, but that's all optional."}
            </p>
            <p class="text-green-800 dark:text-green-200">
                {"When you're ready, check out the Group tab. Most people compete in groups of up to 4 people, and
                it's really easy to join or create them."}
            </p>
            <p class="text-green-800 dark:text-green-200">
                {"Keep an eye on your inbox for more updates from our Committee as the event approaches :)"}
            </p>
        </div>
        }

        <PageTitle page_description="View or edit your profile and requirements">
            {"Your profile"}
        </PageTitle>

        if loading.clone() {
        <LoadingSpinner />
        }

        if let Some(profile) = profile {
        <div class="space-y-4 mt-4">
            <Input static_value={profile.bath_username} readonly={true} input_label="Bath Username" />

            <ProfileDatapoint data_key={ProfileKey::DisplayName} current_value={profile.display_name}
                on_value_change={on_datapoint_change.clone()} />
            <ProfileDatapoint data_key={ProfileKey::AccessibilityRequirements}
                current_value={profile.accessibility_requirements} on_value_change={on_datapoint_change.clone()} />
            <ProfileDatapoint data_key={ProfileKey::DietaryRequirements}
                current_value={profile.dietary_requirements} on_value_change={on_datapoint_change.clone()} />
        </div>
        }
    </PageContainer>
    }
}
