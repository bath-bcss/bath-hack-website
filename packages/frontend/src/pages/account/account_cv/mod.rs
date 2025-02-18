use yew::prelude::*;

use crate::{
    components::{
        error::ErrorMessage, loading_spinner::LoadingSpinner, page_container::PageContainer,
        page_title::PageTitle,
    },
    data::cv::cv_exists,
    pages::account::account_cv::{manage_cv::AccountCVPageManage, upload_cv::AccountCVPageUpload},
};

mod manage_cv;
mod types;
mod upload_cv;

#[function_component(AccountCVPage)]
pub fn account_cv_page() -> Html {
    let cv_exists_handle = use_state_eq(|| false);
    let cv_exists_val = *cv_exists_handle;
    let cv_exists_loading_handle = use_state_eq(|| true);
    let cv_exists_loading = *cv_exists_loading_handle;
    let cv_exists_error_handle = use_state_eq(|| None::<String>);
    let cv_exists_error = (*cv_exists_error_handle).clone();

    {
        let cv_exists_handle = cv_exists_handle.clone();
        use_effect_with((), move |()| {
            let cv_exists_handle = cv_exists_handle.clone();
            let cv_exists_loading_handle = cv_exists_loading_handle.clone();
            let cv_exists_error_handle = cv_exists_error_handle.clone();

            wasm_bindgen_futures::spawn_local(async move {
                cv_exists_error_handle.set(None);
                cv_exists_loading_handle.set(true);
                let resp = cv_exists().await;
                cv_exists_loading_handle.set(false);

                match resp {
                    Err(e) => cv_exists_error_handle.set(Some(e.to_string())),
                    Ok(d) => cv_exists_handle.set(d.exists),
                }
            })
        });
    }

    html! {
        <PageContainer>
            <PageTitle page_description="Optionally upload a CV to share with sponsors">
                { "Your CV" }
            </PageTitle>
            <ErrorMessage message={cv_exists_error} />
            if cv_exists_loading {
                <LoadingSpinner class={classes!("mt-4")} />
            } else {
                if cv_exists_val {
                    <AccountCVPageManage cv_exists_handle={cv_exists_handle} />
                } else {
                    <AccountCVPageUpload cv_exists_handle={cv_exists_handle} />
                }
            }
        </PageContainer>
    }
}
