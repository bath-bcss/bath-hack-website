#![cfg(ldap)]

use ldap3::{LdapConnAsync, LdapConnSettings, LdapError, Scope};
use log::error;
use r2d2::Error;
use thiserror::Error;

use crate::app_config::AppConfig;
use crate::db::DbPool;
use crate::models::signup_requests::SignupRequestObject;
use crate::models::users::User;

pub type Ldap = ldap3::Ldap;

use std::convert::TryFrom;

fn escape_ldap_input(input_val: String) -> String {
    input_val
        .replace("*", "\\2a")
        .replace("(", "\\28")
        .replace(")", "\\29")
        .replace("\\", "\\5c")
        .replace("\0", "\\00")
}

pub async fn connect_ldap(
    app_config: AppConfig
) -> Result<Ldap, LdapError> {
    let settings = LdapConnSettings::new()
        .set_starttls(true)
        .set_no_tls_verify(true);

    let connection_url = url::Url::parse(&app_config.ldap_url).expect("Failed to parse LDAP url");

    let (conn, ldap) = LdapConnAsync::from_url_with_settings(settings, &connection_url).await?;
    ldap3::drive!(conn);
    Ok(ldap)
}

pub async fn get_bath_user_details(
    username: String,
    mut ldap: Ldap,
) -> Result<BathUserStatus, LdapError> {
    let username = escape_ldap_input(username);

    let result = ldap
        .search(
            format!("uid={},ou=people,o=bath.ac.uk", username).as_str(),
            Scope::Subtree,
            format!(
                "(&(objectClass=BathStudentRole)(roleOccupant=uid={},ou=people,o=bath.ac.uk))",
                username
            )
            .as_str(),
            Vec::<&str>::new(),
        )
        .await?;
    match result.1.rc {
        32 =>  { Ok(BathUserStatus::UserNotExists) }
        0 => {
            if result.0.is_empty() {
                Ok(BathUserStatus::UserIsNotStudent)
            } else {
                Ok(BathUserStatus::UserIsStudent)
            }
        }
        _ => { Err(LdapError::LdapResult { result: result.1 }) }
    }
}

#[derive(Debug, Error)]
pub enum PendingUserCheckError {
    #[error("Database error")]
    DBError(diesel::result::Error),
    #[error("Database pool error")]
    DBPoolError(Error),
    #[error("LDAP error")]
    LdapError(LdapError),
}

impl From<diesel::result::Error> for PendingUserCheckError {
    fn from(error: diesel::result::Error) -> Self {
        PendingUserCheckError::DBError(error)
    }
}

impl From<LdapError> for PendingUserCheckError {
    fn from(error: LdapError) -> Self {
        PendingUserCheckError::LdapError(error)
    }
}

impl From<r2d2::Error> for PendingUserCheckError {
    fn from(error: r2d2::Error) -> Self {
        PendingUserCheckError::DBPoolError(error)
    }
}


pub async fn check_pending_users(
    ldap: Ldap,
    db_con: DbPool,
) -> Result<(), PendingUserCheckError> {
    let mut conn = db_con.get()?;

    let (user_usernames, signup_request_usernames) = conn.build_transaction().serializable().read_only().deferrable().run(
        |mut tx| -> Result<(Vec<String>, Vec<String>), PendingUserCheckError> {
            Ok((User::find_usernames_by_ldap_status(&mut tx, 0)?, SignupRequestObject::find_usernames_by_ldap_status(&mut tx, 0)?))
        }
    )?;
    for username in signup_request_usernames {
        let status = get_bath_user_details(username.clone(), ldap.clone()).await?;

        conn.build_transaction().serializable().run(
            |mut tx| -> Result<(), diesel::result::Error> {
                SignupRequestObject::set_ldap_status(&mut tx, username.clone(), status as i16).or_else(|_| {
                    User::set_ldap_status(&mut tx, username, status as i16)
                })
            }
        )?;
    }
    for username in user_usernames {
        let status = get_bath_user_details(username.clone(), ldap.clone()).await?;

        conn.build_transaction().serializable().run(
            |mut tx| -> Result<(), diesel::result::Error> {
                User::set_ldap_status(&mut tx, username, status as i16)
            }
        )?;
    }
    Ok(())
}