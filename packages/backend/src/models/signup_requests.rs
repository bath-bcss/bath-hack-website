use std::collections::HashMap;

use bhw_models::prelude::*;
use bhw_models::signup_request;
use chrono::Duration;
use chrono::Utc;
use mailgun_rs::SendResponse;
use mailgun_rs::SendResult;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QuerySelect, Set
};
use thiserror::Error;
use crate::app_config::AppConfig;
use crate::data::mail::Mailer;
use crate::data::mail::SendInstruction;
use crate::util::passwords::PasswordManager;

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
            .filter(signup_request::Column::ExpiresAt.lte(Utc::now().naive_utc()))
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
        id: &String,
    ) -> Result<Option<signup_request::Model>, SignupRequestFromIdError> {
        let parsed_id =
            uuid::Uuid::parse_str(id).map_err(|e| SignupRequestFromIdError::InvalidID(e))?;

        let resp = SignupRequest::find()
            .filter(signup_request::Column::Id.eq(parsed_id))
            .one(conn)
            .await?;

        Ok(resp)
    }

    pub fn find_usernames_by_ldap_status<C: ConnectionTrait>(
        conn: &C,
        status: &i16,
    ) -> Result<Vec<String>, todo!()> {
        todo!()
    }
    pub fn verify_hash(
        signup_request: &signup_request::Model,
        secret: &String,
    ) -> Result<bool, argon2::password_hash::Error> {
        PasswordManager::verify(&secret, &signup_request.secret_hash)
    }

    pub async fn create<C: ConnectionTrait>(
        conn: &C,
        username: &String,
        status: i16,
    ) -> Result<NewSignupRequestSecret, SignupRequestCreateError> {
        let already_exists = Self::exists_for_username(conn, &username).await?;
        if already_exists {
            return Err(SignupRequestCreateError::AlreadyExists);
        }

        let secret = PasswordManager::hash_random()
            .map_err(|e| SignupRequestCreateError::PasswordError(e))?;

        let new_signup_request = signup_request::ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            bath_username: Set(username.clone()),
            expires_at: {
                let now = Utc::now();
                let new_time = now
                    .checked_add_signed(Duration::minutes(15))
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
        return (signup_request.bath_username.clone()) + "@bath.ac.uk";
    }

    pub fn expired(signup_request: &signup_request::Model) -> bool {
        signup_request.expires_at <= Utc::now().naive_utc()
    }

    pub fn send_email<'a>(
        signup_request: &'a signup_request::Model,
        config: &'a AppConfig,
        secret: &'a String,
    ) -> SendResult<SendResponse> {
        let mailer = Mailer::<'a>::client(config);
        let mut mail_vars = HashMap::new();
        mail_vars.insert("request_id".to_string(), signup_request.id.to_string());
        mail_vars.insert("secret".to_string(), secret.clone());

        let instruction = SendInstruction {
            to: &Self::email_address(signup_request),
            subject: &"Welcome to Bath Hack!".to_string(),
            template_key: &"bhw-welcome".to_string(),
            vars: &mail_vars,
        };
        mailer.send_template(&instruction)
    }

    pub fn set_ldap_status<C: ConnectionTrait>(
        conn: &C,
        username: &String,
        new_status: &i16,
    ) -> Result<(), todo!()> {
        todo!()
    }
}
