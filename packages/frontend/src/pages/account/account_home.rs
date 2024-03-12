use bhw_types::requests::{profile::ProfileResponse, update_profile::UpdateProfileRequest};
use gloo_console::error;
use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::hooks::use_location;

use crate::{
    components::{
        account::profile::{
            profile_datapoint::{ProfileDatapoint, ProfileKey},
            t_shirt_size_picker::TShirtSizePicker,
        },
        form::input::Input,
        loading_spinner::LoadingSpinner,
        page_container::PageContainer,
        page_control_paragraph::PageControlParagraph,
        page_title::PageTitle,
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

                if let Some(display_name) = req.display_name {
                    profile.display_name = Some(display_name);
                }
                if let Some(accessibility_requirements) = req.accessibility_requirements {
                    profile.accessibility_requirements = Some(accessibility_requirements);
                }
                if let Some(dietary_requirements) = req.dietary_requirements {
                    profile.dietary_requirements = Some(dietary_requirements);
                }
                if let Some(t_shirt_size) = req.t_shirt_size {
                    profile.t_shirt_size = Some(t_shirt_size)
                }

                profile_handle.set(Some(profile));
            },
        )
    };

    html! {
        <PageContainer>
            if is_initial_signup {
                <div
                    class="p-4 bg-green-200 dark:bg-green-700 shadow-lg dark:shadow-md shadow-green-100 dark:shadow-green-800 rounded-2xl mb-4"
                >
                    <h2 class="text-green-900 dark:text-green-200 font-bold text-xl">
                        { "You made it!" }
                    </h2>
                    <p class="text-green-800 dark:text-green-200">
                        { "That's it; you're officially going to Bath Hack! If you feel like it, you can fill out the rest of
                your profile, but that's all optional." }
                    </p>
                    <p class="text-green-800 dark:text-green-200">
                        { "When you're ready, check out the Group tab. Most people compete in groups of up to 4 people, and
                it's really easy to join or create them." }
                    </p>
                    <p class="text-green-800 dark:text-green-200">
                        { "Keep an eye on your inbox for more updates from our Committee as the event approaches :)" }
                    </p>
                </div>
            }
            <PageTitle page_description="View or edit your profile and requirements">
                { "Your profile" }
            </PageTitle>
            <PageControlParagraph>
                { "All of the below fields are completely optional. Entering your name will help your group members
                identify you, and adding your access or dietary requirements will help us provide everything you need at
                the event." }
            </PageControlParagraph>
            <PageControlParagraph>
                { "Your accessibility requirements will be stored securely and shared only with BCSS Committee members or
                members of University staff as needed. Unless necessary, please do not disclose sensitive or medical
                information." }
            </PageControlParagraph>
            if loading.clone() {
                <LoadingSpinner class={classes!("mt-4")} />
            }
            if let Some(profile) = profile {
                <div class="space-y-4 mt-4">
                    <Input
                        static_value={profile.bath_username}
                        readonly=true
                        input_label="Bath Username"
                    />
                    <ProfileDatapoint
                        data_key={ProfileKey::DisplayName}
                        current_value={profile.display_name}
                        on_value_change={on_datapoint_change.clone()}
                    />
                    <TShirtSizePicker
                        current_value={profile.t_shirt_size}
                        on_datapoint_change={on_datapoint_change.clone()}
                    />
                    <ProfileDatapoint
                        data_key={ProfileKey::AccessibilityRequirements}
                        current_value={profile.accessibility_requirements}
                        on_value_change={on_datapoint_change.clone()}
                    />
                    <ProfileDatapoint
                        data_key={ProfileKey::DietaryRequirements}
                        current_value={profile.dietary_requirements}
                        on_value_change={on_datapoint_change.clone()}
                    />
                </div>
            }
        </PageContainer>
    }
}
