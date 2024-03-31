use std::path::Path;

use s3::{creds::Credentials, error::S3Error, Bucket, Region};
use thiserror::Error;
use tokio::fs::File;

use crate::app_config::AppConfig;

#[derive(Debug, Clone)]
pub struct CVManager {
    bucket: Bucket,
}

#[derive(Debug, Error)]
pub enum UploadFileError {
    #[error("open file: {0}")]
    OpenFile(#[from] std::io::Error),
    #[error("s3: {0}")]
    Upload(#[from] S3Error),
}

#[derive(Debug, Error)]
pub enum UserHasCVError {
    #[error("s3: {0}")]
    Head(#[from] S3Error),
}

#[derive(Debug, Error)]
pub enum SignedDownloadError {
    #[error("s3: {0}")]
    Presign(#[from] S3Error),
}

#[derive(Debug, Error)]
pub enum DeleteCVError {
    #[error("s3: {0}")]
    Delete(#[from] S3Error),
}

impl CVManager {
    pub fn client(app_config: &AppConfig) -> CVManager {
        let region = Region::Custom {
            region: app_config.s3_region.clone(),
            endpoint: app_config.s3_endpoint.clone(),
        };

        let mut bucket = Bucket::new(
            app_config.s3_bucket.clone().as_str(),
            region,
            Credentials::default().expect("Parse credentials"),
        )
        .expect("init bucket");

        bucket.set_path_style();

        CVManager { bucket }
    }

    fn user_id_to_path(user_id: &uuid::Uuid) -> String {
        let user_id_string = user_id.to_string();
        return user_id_string + ".pdf";
    }

    pub async fn upload_file_by_path(
        &self,
        file_path: &Path,
        user_id: &uuid::Uuid,
    ) -> Result<(), UploadFileError> {
        let mut file = File::open(file_path).await?;

        self.bucket
            .put_object_stream(&mut file, Self::user_id_to_path(user_id))
            .await?;

        Ok(())
    }

    pub async fn user_has_cv(&self, user_id: &uuid::Uuid) -> Result<bool, UserHasCVError> {
        let resp = self
            .bucket
            .head_object(Self::user_id_to_path(user_id))
            .await;
        if let Err(e) = resp {
            if let S3Error::HttpFailWithBody(404, _) = e {
                return Ok(false);
            }
            return Err(e.into());
        }
        Ok(true)
    }

    pub async fn signed_download(
        &self,
        user_id: &uuid::Uuid,
    ) -> Result<String, SignedDownloadError> {
        let url = self
            .bucket
            .presign_get(Self::user_id_to_path(user_id), 600, None)
            .await?;
        Ok(url)
    }

    pub async fn delete_cv(&self, user_id: &uuid::Uuid) -> Result<(), DeleteCVError> {
        self.bucket
            .delete_object(Self::user_id_to_path(user_id))
            .await?;

        Ok(())
    }
}
