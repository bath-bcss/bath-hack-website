use bhw_types::requests::update_profile::UpdateProfileRequest;
use yew::prelude::*;

use crate::{
    components::{button::Button, input::Input},
    data::profile::update_profile,
};

#[derive(PartialEq, Clone)]
pub enum ProfileKey {
    DisplayName,
    AccessibilityRequirements,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data_key: ProfileKey,
    pub current_value: Option<String>,
    pub on_value_change: Callback<UpdateProfileRequest>,
}

#[function_component(ProfileDatapoint)]
pub fn profile_datapoint(props: &Props) -> Html {
    let input_label = use_memo((props.data_key.clone(),), |(data_key,)| match data_key {
        ProfileKey::DisplayName => "Display name",
        ProfileKey::AccessibilityRequirements => "Accessibility requirements",
    });

    let local_input_state = use_state_eq(|| match props.current_value.clone() {
        Some(cv) => cv,
        None => String::default(),
    });
    let local_input_value = (*local_input_state).clone();

    let value_has_changed = use_memo(
        (local_input_value.clone(), props.current_value.clone()),
        |(current_value, original_value)| {
            let current_value = (*current_value).clone();
            match original_value {
                Some(original_value) => current_value != *original_value,
                None => current_value != String::default(),
            }
        },
    );

    let error_handle = use_state(|| None::<String>);
    let loading_handle = use_state(|| false);
    let loading = (*loading_handle).clone();

    let on_save_click = use_callback(
        (
            local_input_value,
            props.data_key.clone(),
            props.on_value_change.clone(),
        ),
        move |e: SubmitEvent, (local_input_value, data_key, on_value_change)| {
            e.prevent_default();

            let local_input_value = (*local_input_value).clone();
            let request = match data_key.clone() {
                ProfileKey::DisplayName => {
                    UpdateProfileRequest::DisplayName(Some(local_input_value))
                }
                ProfileKey::AccessibilityRequirements => {
                    UpdateProfileRequest::AccessibilityRequirements(Some(local_input_value))
                }
            };

            let error_handle = error_handle.clone();
            let loading_handle = loading_handle.clone();
            let on_value_change = on_value_change.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading_handle.set(true);
                let result = update_profile(&request).await;
                loading_handle.set(false);

                match result {
                    Err(e) => error_handle.set(Some(e.to_string())),
                    Ok(_) => {
                        on_value_change.emit(request.clone());
                    }
                }
            });
        },
    );

    html! {
    <form onsubmit={on_save_click}>
        <Input input_label={input_label.to_string()} handle={local_input_state} disabled={loading} />
        if *value_has_changed {
        <Button dark_mode={false} disabled={loading} button_type="submit" class={classes!("mt-4")}>{"Save"}</Button>
        }
    </form>
    }
}
