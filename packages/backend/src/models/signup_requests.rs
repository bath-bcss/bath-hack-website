use std::collections::HashMap;

use chrono::Duration;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use mailgun_rs::SendResponse;
use mailgun_rs::SendResult;
use thiserror::Error;

use crate::app_config::AppConfig;
use crate::data::mail::Mailer;
use crate::data::mail::SendInstruction;
use crate::util::passwords::PasswordManager;

#[derive(Debug, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = crate::schema::signup_request)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SignupRequestObject {
    pub id: uuid::Uuid,
    pub bath_username: String,
    pub created_at: NaiveDateTime,
    pub expires: NaiveDateTime,
    secret_hash: String,
}

#[derive(Debug, Error)]
pub enum SignupRequestCreateError {
    #[error("Adding 15 minutes to current time failed")]
    Duration,
    #[error("A signup request is already in progress for that username")]
    AlreadyExists,
    #[error("Password hashing failed: {0}")]
    PasswordError(argon2::password_hash::Error),
    #[error("Inserting SignupRequest failed: {0}")]
    DBError(diesel::result::Error),
}

#[derive(Debug)]
pub struct NewSignupRequestSecret {
    pub secret: String,
    pub request: SignupRequestObject,
}

impl SignupRequestObject {
    pub fn exists_for_username(
        conn: &mut PgConnection,
        username: &String,
    ) -> Result<bool, diesel::result::Error> {
        use crate::schema::signup_request;
        let resp: i64 = signup_request::table
            .count()
            .filter(signup_request::bath_username.eq(username))
            .get_result(conn)?;
        Ok(resp > 0)
    }

    pub fn from_id(
        conn: &mut PgConnection,
        id: &String,
    ) -> Result<Option<SignupRequestObject>, diesel::result::Error> {
        use crate::schema::signup_request;
        let parsed_id = uuid::Uuid::parse_str(id).unwrap();

        let resp: Vec<SignupRequestObject> = signup_request::table
            .select(SignupRequestObject::as_select())
            .filter(signup_request::id.eq(parsed_id))
            .load(conn)?;

        let first = resp.first().cloned();
        Ok(first)
    }

    pub fn verify_hash(&self, secret: &String) -> Result<bool, argon2::password_hash::Error> {
        PasswordManager::verify(&secret, &self.secret_hash)
    }

    pub fn create(
        conn: &mut PgConnection,
        username: String,
    ) -> Result<NewSignupRequestSecret, SignupRequestCreateError> {
        use crate::schema::signup_request;

        let already_exists = SignupRequestObject::exists_for_username(conn, &username)
            .map_err(|e| SignupRequestCreateError::DBError(e))?;
        if already_exists {
            return Err(SignupRequestCreateError::AlreadyExists);
        }

        let secret = PasswordManager::hash_random()
            .map_err(|e| SignupRequestCreateError::PasswordError(e))?;

        let new_signup_request = SignupRequestObject {
            id: uuid::Uuid::new_v4(),
            bath_username: username,
            created_at: Utc::now().naive_utc(),
            expires: {
                let now = Utc::now();
                let new_time = now
                    .checked_add_signed(Duration::minutes(15))
                    .ok_or(SignupRequestCreateError::Duration)?;
                new_time.naive_utc()
            },
            secret_hash: secret.hash,
        };

        diesel::insert_into(signup_request::table)
            .values(&new_signup_request)
            .execute(conn)
            .map_err(|e| SignupRequestCreateError::DBError(e))?;

        Ok(NewSignupRequestSecret {
            secret: secret.random_password,
            request: new_signup_request,
        })
    }

    fn email_address(&self) -> String {
        return (self.bath_username.clone()) + "@bath.ac.uk";
    }

    pub fn send_email<'a>(
        &self,
        config: &'a AppConfig,
        secret: &'a String,
    ) -> SendResult<SendResponse> {
        let mailer = Mailer::<'a>::client(config);
        let mut mail_vars = HashMap::new();
        mail_vars.insert("request_id".to_string(), self.id.to_string());
        mail_vars.insert("secret".to_string(), secret.clone());

        let instruction = SendInstruction {
            to: &self.email_address(),
            subject: &"Welcome to Bath Hack!".to_string(),
            template_key: &"bhw-welcome".to_string(),
            vars: &mail_vars,
        };
        mailer.send_template(&instruction)
    }
}
