use chrono::Duration;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use thiserror::Error;

use crate::util::passwords::PasswordManager;

#[derive(Queryable, Selectable, Insertable)]
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
    pub id: String,
    pub secret: String,
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
            id: new_signup_request.id.to_string(),
            secret: secret.random_password,
        })
    }
}
