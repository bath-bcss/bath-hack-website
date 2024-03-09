use bhw_types::{
    models::website_user::TShirtSize, requests::update_profile::UpdateProfileRequest,
    strum::IntoEnumIterator,
};
use web_sys::{wasm_bindgen::UnwrapThrowExt, HtmlInputElement};
use yew::prelude::*;

use crate::{
    components::{
        button::Button,
        error::ErrorMessage,
        form::form_handle::{FormHandle, FormHandleChildProps},
    },
    data::profile::update_profile,
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub current_value: Option<TShirtSize>,
    pub on_datapoint_change: Callback<UpdateProfileRequest>,
}

#[function_component(TShirtSizePicker)]
pub fn t_shirt_size_picker(props: &Props) -> Html {
    let t_shirt_size_handle = use_state_eq(|| props.current_value.clone());
    let t_shirt_size = (*t_shirt_size_handle).clone();

    let select_value = use_memo(
        (t_shirt_size.clone(),),
        |(t_shirt_size,)| match t_shirt_size {
            None => "NONE".to_string(),
            Some(t_shirt_size) => {
                serde_json::to_string(t_shirt_size).expect_throw("Serializing T-Shirt size")
            }
        },
    );

    let on_select_change = use_callback(
        (t_shirt_size_handle.clone(),),
        |e: Event, (t_shirt_size_handle,)| {
            let select_el: HtmlInputElement = e.target_unchecked_into();
            if select_el.value() == "NONE" {
                return;
            }

            let parsed_val: TShirtSize = serde_json::from_str(select_el.value().as_str())
                .expect_throw("Parsing T-Shirt size");
            t_shirt_size_handle.set(Some(parsed_val));
        },
    );

    let loading_handle = use_state_eq(|| false);
    let loading = (*loading_handle).clone();
    let error_handle = use_state_eq(|| None::<String>);
    let error = (*error_handle).clone();

    let on_submit = use_callback(
        (
            t_shirt_size.clone(),
            loading_handle,
            error_handle,
            props.on_datapoint_change.clone(),
        ),
        |e: SubmitEvent, (t_shirt_size, loading_handle, error_handle, on_datapoint_change)| {
            e.prevent_default();

            let t_shirt_size = t_shirt_size.clone();
            let loading_handle = loading_handle.clone();
            let error_handle = error_handle.clone();
            let on_datapoint_change = on_datapoint_change.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading_handle.set(true);
                let req = UpdateProfileRequest {
                    t_shirt_size,
                    ..Default::default()
                };
                let resp = update_profile(&req).await;
                loading_handle.set(false);

                match resp {
                    Err(e) => error_handle.set(Some(e.to_string())),
                    Ok(_) => {
                        on_datapoint_change.emit(req);
                    }
                }
            });
        },
    );

    let child_renderer = use_callback(
        (
            (*select_value).clone(),
            on_select_change,
            loading,
            t_shirt_size.clone(),
        ),
        |child_props: FormHandleChildProps,
         (select_value, on_select_change, loading, t_shirt_size)| {
            html! {
            <select class={child_props.class.clone()} value={select_value.clone()} onchange={on_select_change}
                disabled={loading.clone()}>
                <option hidden={true} selected={t_shirt_size.clone().is_none()}>{"Please select"}</option>

                {TShirtSize::iter().map(|size| {
                let label = match size {
                TShirtSize::S => "S",
                TShirtSize::M => "M",
                TShirtSize::L => "L",
                TShirtSize::XL => "XL",
                TShirtSize::XXL => "2XL",
                TShirtSize::XXXL => "3XL",
                TShirtSize::XXXXL => "4XL"
                };

                let opt_value = serde_json::to_string(&size).expect_throw("Serialize T-Shirt size");

                html! {
                <option value={opt_value.clone()} key={opt_value.clone()} selected={t_shirt_size.clone()==Some(size)}>
                    {label}
                </option>
                }
                }).collect::<Html>()}
            </select>
            }
        },
    );

    html! {
    <>
        <form onsubmit={on_submit}>
            <FormHandle child_renderer={child_renderer} label={"T-Shirt size"} />
            if t_shirt_size != props.current_value {
            <Button dark_mode={false} button_type={"submit"} class={classes!("mt-4")} disabled={loading}>
                {"Save"}
            </Button>
            }
            <ErrorMessage message={error} />
        </form>
    </>
    }
}
