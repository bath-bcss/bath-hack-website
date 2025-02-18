use web_sys::{File, HtmlInputElement};
use yew::prelude::*;

use crate::{
    components::{
        button::Button, error::ErrorMessage, form::input::Input,
        page_control_paragraph::PageControlParagraph,
    },
    data::cv::upload_cv,
};

use super::types::AccountCVSubpageProps;

#[function_component(AccountCVPageUpload)]
pub fn upload_cv_page(props: &AccountCVSubpageProps) -> Html {
    let current_file_handle = use_state_eq(|| None::<File>);
    let current_file = (*current_file_handle).clone();
    let on_file_change_handler = use_callback(
        (current_file_handle,),
        |e: InputEvent, (current_file_handle,)| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                if let Some(file_list) = input.files() {
                    current_file_handle.set(file_list.get(0));
                    return;
                }
            }

            current_file_handle.set(None)
        },
    );

    let error_handle = use_state_eq(|| None::<String>);
    let error = (*error_handle).clone();
    let loading_handle = use_state_eq(|| false);
    let loading = *loading_handle;
    let on_cv_submit = use_callback(
        (
            current_file.clone(),
            error_handle.clone(),
            loading_handle.clone(),
            props.cv_exists_handle.clone(),
        ),
        |e: SubmitEvent, (current_file, error_handle, loading_handle, cv_exists_handle)| {
            e.prevent_default();
            if let Some(current_file) = current_file {
                let current_file = (*current_file).clone();
                let error_handle = error_handle.clone();
                let loading_handle = loading_handle.clone();
                let cv_exists_handle = cv_exists_handle.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    error_handle.set(None);
                    loading_handle.set(true);
                    let res = upload_cv(current_file).await;
                    loading_handle.set(false);

                    match res {
                        Err(e) => error_handle.set(Some(e.to_string())),
                        Ok(_) => cv_exists_handle.set(true),
                    }
                });
            }
        },
    );

    html! {
        <>
            <PageControlParagraph>
                { "Some of our sponsors may be recruiting interns and are therefore interested in seeing the CVs of
                    any
                    potential applicants. This is completely optional, and it only takes a few seconds!" }
            </PageControlParagraph>
            <PageControlParagraph>
                { "If you upload a CV, it will be shared with some of our sponsors, along with your provided display
            name (if any), group name, and University username (so they can get in touch with you). This does " }
                <strong>{ "not" }</strong>
                { " constitute a formal job application of any kind." }
            </PageControlParagraph>
            <form onsubmit={on_cv_submit.clone()}>
                <Input
                    input_type="file"
                    input_label="Select your CV"
                    accept_file="application/pdf"
                    onchange={on_file_change_handler}
                    required=true
                    disabled={loading}
                >
                    { "Only PDFs are accepted (up to 10MB)" }
                </Input>
                <Button
                    button_type="submit"
                    background_is_dark=false
                    class={classes!("mt-4")}
                    disabled={current_file.is_none() || loading}
                >
                    { "Upload and share with sponsors" }
                </Button>
            </form>
            <ErrorMessage message={error} />
        </>
    }
}
