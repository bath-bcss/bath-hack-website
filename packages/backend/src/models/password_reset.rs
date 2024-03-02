use std::collections::HashMap;

use bhw_models::{password_reset, prelude::*, website_user};
use chrono::{Duration, Utc};
use mailgun_rs::{SendResponse, SendResult};
use rand::{
    distributions::{Alphanumeric, DistString},
    rngs::OsRng,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, ModelTrait, PaginatorTrait,
    QueryFilter, QuerySelect, SelectColumns, Set,
};
use thiserror::Error;

use crate::{
    app_config::AppConfig,
    data::mail::{Mailer, SendInstruction},
    util::email_address::email_address,
};

#[derive(Debug, Error)]
pub enum CreatePasswordResetError {
    #[error("database: {0}")]
    DBError(#[from] DbErr),
    #[error("Failed to do time")]
    TimeError,
    #[error("Username not found")]
    UserNotFound,
}

#[derive(Debug, Error)]
pub enum FindAndDeletePINError {
    #[error("database: {0}")]
    DBError(#[from] DbErr),
    #[error("PIN not found")]
    PINNotFound,
}

pub struct PasswordResetHelper;
impl PasswordResetHelper {
    pub async fn one_exists_for_username<T: ConnectionTrait>(
        conn: &T,
        username: String,
    ) -> Result<bool, DbErr> {
        let res = PasswordReset::find()
            .find_also_related(WebsiteUser)
            .filter(website_user::Column::BathUsername.eq(username))
            // not yet expired
            .filter(password_reset::Column::ExpiresAt.gt(Utc::now().naive_utc()))
            .count(conn)
            .await?;

        Ok(res > 0)
    }

    fn generate_pin() -> String {
        Alphanumeric.sample_string(&mut OsRng, 10)
    }

    pub async fn create<T: ConnectionTrait>(
        conn: &T,
        username: String,
    ) -> Result<password_reset::Model, CreatePasswordResetError> {
        let (user_id,): (uuid::Uuid,) = WebsiteUser::find()
            .select_only()
            .select_column(website_user::Column::Id)
            .filter(website_user::Column::BathUsername.eq(username))
            .into_tuple()
            .one(conn)
            .await?
            .ok_or(CreatePasswordResetError::UserNotFound)?;

        // delete any expired password resets
        PasswordReset::delete_many()
            .filter(password_reset::Column::UserId.eq(user_id))
            .filter(password_reset::Column::ExpiresAt.lt(Utc::now().naive_utc()))
            .exec(conn)
            .await?;

        let new_password_reset = password_reset::ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            user_id: Set(user_id),
            pin: Set(Self::generate_pin()),
            expires_at: {
                let now = Utc::now();
                let new_time = now
                    .checked_add_signed(Duration::minutes(15))
                    .ok_or(CreatePasswordResetError::TimeError)?;
                Set(new_time.naive_utc())
            },
            ..Default::default()
        };

        let new_password_reset: password_reset::Model = new_password_reset.insert(conn).await?;

        Ok(new_password_reset)
    }

    pub fn send_email(
        app_config: &AppConfig,
        to_username: String,
        pin: String,
    ) -> SendResult<SendResponse> {
        let mailer = Mailer::client(app_config);
        let mut mail_vars = HashMap::new();
        mail_vars.insert("pin".to_string(), pin);
        mailer.send_template(SendInstruction {
            to: email_address(to_username),
            vars: mail_vars,
            template_key: "bhw-password-reset".to_string(),
            subject: "Reset your Bath Hack password".to_string(),
        })
    }

    pub async fn find_and_delete_pin<T: ConnectionTrait>(
        conn: &T,
        pin: String,
    ) -> Result<uuid::Uuid, FindAndDeletePINError> {
        let matching = PasswordReset::find()
            .filter(password_reset::Column::Pin.eq(pin))
            .filter(password_reset::Column::ExpiresAt.gt(Utc::now().naive_utc()))
            .one(conn)
            .await?
            .ok_or(FindAndDeletePINError::PINNotFound)?;

        matching.clone().delete(conn).await?;
        Ok(matching.user_id)
    }
}
