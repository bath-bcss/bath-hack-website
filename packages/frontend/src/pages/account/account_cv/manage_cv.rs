use gloo_utils::window;
use yew::prelude::*;

use crate::{
    components::{
        button::Button, error::ErrorMessage, page_control_paragraph::PageControlParagraph,
    },
    data::cv::{delete_cv, get_cv_download_url},
};

use super::types::AccountCVSubpageProps;

#[function_component(AccountCVPageManage)]
pub fn manage_cv_page(props: &AccountCVSubpageProps) -> Html {
    let loading_handle = use_state_eq(|| false);
    let loading = (*loading_handle).clone();
    let download_error_handle = use_state_eq(|| None::<String>);
    let download_error = (*download_error_handle).clone();
    let delete_error_handle = use_state_eq(|| None::<String>);
    let delete_error = (*delete_error_handle).clone();

    let on_download_click = use_callback(
        (loading_handle.clone(), download_error_handle.clone()),
        |_: MouseEvent, (loading_handle, download_error_handle)| {
            let loading_handle = loading_handle.clone();
            let download_error_handle = download_error_handle.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading_handle.set(true);
                let url_resp = get_cv_download_url().await;

                match url_resp {
                    Err(e) => {
                        loading_handle.set(false);
                        download_error_handle.set(Some(e.to_string()))
                    }
                    Ok(d) => {
                        window()
                            .location()
                            .set_href(&d.url)
                            .expect("Failed to set window href");
                    }
                }
            })
        },
    );

    let on_delete_click = use_callback(
        (
            loading_handle.clone(),
            delete_error_handle.clone(),
            props.cv_exists_handle.clone(),
        ),
        |_: MouseEvent, (loading_handle, delete_error_handle, cv_exists_handle)| {
            let loading_handle = loading_handle.clone();
            let delete_error_handle = delete_error_handle.clone();
            let cv_exists_handle = cv_exists_handle.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading_handle.set(true);
                let resp = delete_cv().await;
                loading_handle.set(false);

                match resp {
                    Err(e) => delete_error_handle.set(Some(e.to_string())),
                    Ok(_) => cv_exists_handle.set(false),
                }
            });
        },
    );

    html! {
        <>
            <PageControlParagraph>
                { "Thanks for uploading your CV! This will be (or already has been) shared with all the
            sponsors of the event, as listed on our home page." }
            </PageControlParagraph>
            <PageControlParagraph>
                { "You can remove your CV (and thereby revoke consent to share your data with our sponsors) at any time
                before
                the end of the event. After this time, please contact each of the sponsors directly." }
            </PageControlParagraph>
            <div>
                <Button
                    background_is_dark=false
                    onclick={on_download_click}
                    class={classes!("mt-4")}
                    disabled={loading}
                >
                    { "Download my CV" }
                </Button>
                <ErrorMessage message={download_error} />
            </div>
            <div>
                <Button
                    background_is_dark=false
                    onclick={on_delete_click}
                    class={classes!("mt-4")}
                    disabled={loading}
                >
                    { "Delete my CV" }
                </Button>
                <ErrorMessage message={delete_error} />
            </div>
        </>
    }
}
