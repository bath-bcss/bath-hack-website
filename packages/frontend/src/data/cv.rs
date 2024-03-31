use bhw_types::{
    nothing::Nothing,
    requests::{
        cv_exists::{CVExistsResponse, CVExistsResponseError},
        delete_cv::DeleteCVResponseError,
        get_cv_download_url::{GetCVDownloadURLResponse, GetCVDownloadURLResponseError},
        upload_cv::UploadCVResponseError,
    },
};
use web_sys::File;

use super::api::{send_file, send_get, FrontendRequestError};

pub async fn upload_cv(file: File) -> Result<Nothing, FrontendRequestError<UploadCVResponseError>> {
    send_file("/cv/upload".to_string(), file, "cv_pdf").await
}

pub async fn cv_exists() -> Result<CVExistsResponse, FrontendRequestError<CVExistsResponseError>> {
    send_get("/cv/exists".to_string()).await
}

pub async fn get_cv_download_url(
) -> Result<GetCVDownloadURLResponse, FrontendRequestError<GetCVDownloadURLResponseError>> {
    send_get("/cv/url".to_string()).await
}

pub async fn delete_cv() -> Result<Nothing, FrontendRequestError<DeleteCVResponseError>> {
    send_get("/cv/delete".to_string()).await
}
