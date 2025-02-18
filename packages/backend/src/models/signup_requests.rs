use std::collections::HashMap;

use crate::app_config::AppConfig;
use crate::data::mail::Mailer;
use crate::data::mail::SendInstruction;
use crate::util::email_address::email_address;
use crate::util::passwords::PasswordManager;
use bhw_models::prelude::*;
use bhw_models::signup_request;
use chrono::Duration;
use chrono::Utc;
use mailgun_rs::SendResponse;
use mailgun_rs::SendResult;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QuerySelect, Set,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignupRequestCreateError {
    #[error("Adding 15 minutes to current time failed")]
    Duration,
    #[error("A signup request is already in progress for that username")]
    AlreadyExists,
    #[error("Password hashing failed: {0}")]
    PasswordError(argon2::password_hash::Error),
    #[error("Inserting SignupRequest failed: {0}")]
    DBError(#[from] DbErr),
}

#[derive(Debug)]
pub struct NewSignupRequestSecret {
    pub secret: String,
    pub request: signup_request::Model,
}

#[derive(Debug, Error)]
pub enum SignupRequestFromIdError {
    #[error("Could not parse ID: {0}")]
    InvalidID(uuid::Error),
    #[error("Finding SignupRequest failed: {0}")]
    DBError(#[from] DbErr),
}

pub struct SignupRequestHelper;

impl SignupRequestHelper {
    pub async fn exists_for_username<C: ConnectionTrait>(
        conn: &C,
        username: &String,
    ) -> Result<bool, DbErr> {
        // delete existing expired signup requests in case user wants to start a new one
        SignupRequest::delete_many()
            .filter(signup_request::Column::BathUsername.eq(username))
            .filter(signup_request::Column::ExpiresAt.lt(Utc::now().naive_utc()))
            .exec(conn)
            .await?;

        let resp = SignupRequest::find()
            .filter(signup_request::Column::BathUsername.eq(username))
            .limit(1)
            .count(conn)
            .await?;

        Ok(resp > 0)
    }

    pub async fn from_id<C: ConnectionTrait>(
        conn: &C,
        id: &str,
    ) -> Result<Option<signup_request::Model>, SignupRequestFromIdError> {
        let parsed_id = uuid::Uuid::parse_str(id).map_err(SignupRequestFromIdError::InvalidID)?;

        let resp = SignupRequest::find()
            .filter(signup_request::Column::Id.eq(parsed_id))
            .one(conn)
            .await?;

        Ok(resp)
    }

    #[cfg(feature = "ldap")]
    pub async fn find_usernames_by_ldap_status<C: ConnectionTrait>(
        conn: &C,
        status: i16,
    ) -> Result<Vec<(uuid::Uuid, String)>, DbErr> {
        let response = SignupRequest::find()
            .filter(signup_request::Column::LdapCheckStatus.eq(status))
            .select_only()
            .column(signup_request::Column::Id)
            .column(signup_request::Column::BathUsername)
            .into_tuple()
            .all(conn)
            .await?;

        Ok(response)
    }
    pub fn verify_hash(
        signup_request: &signup_request::Model,
        secret: &str,
    ) -> Result<bool, argon2::password_hash::Error> {
        PasswordManager::verify(secret, &signup_request.secret_hash)
    }

    pub async fn create<C: ConnectionTrait>(
        conn: &C,
        username: &String,
        status: i16,
        skip_existence_check: bool,
    ) -> Result<NewSignupRequestSecret, SignupRequestCreateError> {
        if skip_existence_check {
            SignupRequest::delete_many()
                .filter(signup_request::Column::BathUsername.eq(username))
                .exec(conn)
                .await?;
        } else {
            let already_exists = Self::exists_for_username(conn, username).await?;
            if already_exists {
                return Err(SignupRequestCreateError::AlreadyExists);
            }
        }

        let secret =
            PasswordManager::hash_random().map_err(SignupRequestCreateError::PasswordError)?;

        let new_signup_request = signup_request::ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            bath_username: Set(username.clone()),
            expires_at: {
                let now = Utc::now();
                let new_time = now
                    .checked_add_signed(
                        Duration::try_minutes(30).ok_or(SignupRequestCreateError::Duration)?,
                    )
                    .ok_or(SignupRequestCreateError::Duration)?;
                Set(new_time.naive_utc())
            },
            secret_hash: Set(secret.hash),
            ldap_check_status: Set(status),
            ..Default::default()
        };

        let model = new_signup_request.insert(conn).await?;
        Ok(NewSignupRequestSecret {
            secret: secret.random_password,
            request: model,
        })
    }

    pub async fn delete<C: ConnectionTrait>(conn: &C, id: uuid::Uuid) -> Result<(), DbErr> {
        SignupRequest::delete_by_id(id).exec(conn).await?;
        Ok(())
    }

    fn email_address(signup_request: &signup_request::Model) -> String {
        email_address(signup_request.bath_username.clone())
    }

    pub fn expired(signup_request: &signup_request::Model) -> bool {
        signup_request.expires_at <= Utc::now().naive_utc()
    }
    pub async fn send_verification_email(
        signup_request: &signup_request::Model,
        config: &AppConfig,
        secret: &str,
    ) -> SendResult<SendResponse> {
        let mailer = Mailer::client(config);
        let mut mail_vars = HashMap::new();
        mail_vars.insert("request_id".to_string(), signup_request.id.to_string());
        mail_vars.insert("secret".to_string(), secret.to_owned());

        let instruction = SendInstruction {
            to: Self::email_address(signup_request),
            subject: "Welcome to WiTathon!".to_string(),
            template_key: "bhw-welcome".to_string(),
            vars: mail_vars,
        };
        mailer.send_template(instruction).await
    }

    pub async fn send_notification_email(
        signup_request: &signup_request::Model,
        config: &AppConfig,
    ) -> SendResult<SendResponse> {
        let mailer = Mailer::client(config);

        let instruction = SendInstruction {
            to: Self::email_address(signup_request),
            subject: "Welcome to WiTathon!".to_string(),
            template_key: "bhw-welcome-noverify".to_string(),
            vars: HashMap::new(),
        };
        mailer.send_template(instruction).await
    }

    #[cfg(feature = "ldap")]
    pub async fn set_ldap_status<C: ConnectionTrait>(
        conn: &C,
        id: &uuid::Uuid,
        new_status: i16,
    ) -> Result<(), DbErr> {
        let updated_signup_request = signup_request::ActiveModel {
            id: Set(*id),
            ldap_check_status: Set(new_status),
            ..Default::default()
        };

        updated_signup_request.save(conn).await?;
        Ok(())
    }
}
