#![cfg(feature = "ldap")]

use ldap3::{LdapConnAsync, LdapConnSettings, LdapError, Scope};
use log::error;
use thiserror::Error;

use crate::app_config::AppConfig;
use crate::models::users::UserHelper;

pub type Ldap = ldap3::Ldap;

use sea_orm::{AccessMode, DatabaseConnection, DbErr, IsolationLevel, TransactionTrait};
use crate::models::ldap_status::BathUserStatus;
use crate::models::signup_requests::SignupRequestHelper;

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
    DBError(DbErr),
    #[error("LDAP error")]
    LdapError(LdapError),
}

impl From<LdapError> for PendingUserCheckError {
    fn from(error: LdapError) -> Self {
        PendingUserCheckError::LdapError(error)
    }
}

impl From<DbErr> for PendingUserCheckError {
    fn from(error: DbErr) -> Self {
        PendingUserCheckError::DBError(error)
    }
}



pub async fn check_pending_users(
    ldap: Ldap,
    db: DatabaseConnection,
) -> Result<(), PendingUserCheckError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::Serializable),
            Some(AccessMode::ReadOnly),
        )
        .await?;
    let signup_request_usernames = SignupRequestHelper::find_usernames_by_ldap_status(&txn, 0).await?;
    let user_usernames = UserHelper::find_usernames_by_ldap_status(&txn, 0).await?;

    txn.commit().await?;

    for username in signup_request_usernames {
        let status = get_bath_user_details(username.clone(), ldap.clone()).await?;
        let txn = db
            .begin_with_config(
                Some(IsolationLevel::Serializable),
                Some(AccessMode::ReadWrite),
            )
            .await?;

        if SignupRequestHelper::set_ldap_status(&txn, &username, status as i16).await? == 0 {
            UserHelper::set_ldap_status(&txn, &username, status as i16).await?;
        }


        txn.commit().await?;
    }
    for username in user_usernames {
        let status = get_bath_user_details(username.clone(), ldap.clone()).await?;
        let txn = db
            .begin_with_config(
                Some(IsolationLevel::Serializable),
                Some(AccessMode::ReadWrite),
            )
            .await?;

        UserHelper::set_ldap_status(&txn, &username, status as i16)
            .await?;

        txn.commit().await?;
    }
    Ok(())
}