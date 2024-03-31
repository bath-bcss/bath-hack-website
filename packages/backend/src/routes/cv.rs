use crate::data::{cv::CVManager, session::SessionUser};
use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web};
use bhw_types::{
    nothing::Nothing,
    requests::{
        cv_exists::{CVExistsResponse, CVExistsResponseError, CVExistsResult},
        delete_cv::{DeleteCVResponseError, DeleteCVResult},
        get_cv_download_url::{
            GetCVDownloadURLResponse, GetCVDownloadURLResponseError, GetCVDownloadURLResult,
        },
        upload_cv::{UploadCVRequest, UploadCVResponseError, UploadCVResult},
    },
};
use log::warn;

#[post("/cv/upload")]
pub async fn upload_cv_route(
    user: SessionUser,
    cv_pdf: MultipartForm<UploadCVRequest>,
    cv_manager: web::Data<CVManager>,
) -> UploadCVResult {
    if cv_pdf.cv_pdf.content_type != Some(mime::APPLICATION_PDF) {
        return Err(UploadCVResponseError::FileTypeOrSize);
    }

    let path = cv_pdf.cv_pdf.file.path();
    cv_manager
        .upload_file_by_path(path, &user.id)
        .await
        .map_err(|e| {
            warn!("upload file: {}", e.to_string());
            UploadCVResponseError::FileError
        })?;

    Ok(Nothing)
}

#[get("/cv/exists")]
pub async fn cv_exists_route(
    user: SessionUser,
    cv_manager: web::Data<CVManager>,
) -> CVExistsResult {
    let exists = cv_manager.user_has_cv(&user.id).await.map_err(|e| {
        warn!("check file exists: {}", e.to_string());
        CVExistsResponseError::FileError
    })?;

    Ok(CVExistsResponse { exists })
}

#[get("/cv/url")]
pub async fn get_cv_download_url_route(
    user: SessionUser,
    cv_manager: web::Data<CVManager>,
) -> GetCVDownloadURLResult {
    let url = cv_manager.signed_download(&user.id).await.map_err(|e| {
        warn!("presign download url for file: {}", e.to_string());
        GetCVDownloadURLResponseError::FileError
    })?;

    Ok(GetCVDownloadURLResponse { url })
}

#[get("/cv/delete")]
pub async fn delete_cv_route(
    user: SessionUser,
    cv_manager: web::Data<CVManager>,
) -> DeleteCVResult {
    cv_manager.delete_cv(&user.id).await.map_err(|e| {
        warn!("delete file: {}", e.to_string());
        DeleteCVResponseError::FileError
    })?;

    Ok(Nothing)
}
