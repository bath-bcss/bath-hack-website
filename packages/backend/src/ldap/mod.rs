use ldap3::{LdapConnAsync, LdapConnSettings, Scope};
use thiserror::Error;

use crate::app_config::AppConfig;

#[derive(Error, Debug)]
pub enum BathUserError {
    #[error("LDAP error: {0}")]
    LDAPError(#[from] ldap3::result::LdapError),
    #[error("User does not exist")]
    UserNotExists,
    #[error("User is not a student")]
    UserIsNotStudent,
}

fn escape_ldap_input(input_val: String) -> String {
    input_val
        .replace("*", "\\2a")
        .replace("(", "\\28")
        .replace(")", "\\29")
        .replace("\\", "\\5c")
        .replace("\0", "\\00")
}

pub async fn get_bath_user_details(
    username: String,
    app_config: &AppConfig,
) -> Result<(), BathUserError> {
    let settings = LdapConnSettings::new()
        .set_starttls(true)
        .set_no_tls_verify(true);

    let connection_url = url::Url::parse(&app_config.ldap_url).expect("Failed to parse LDAP url");

    let (conn, mut ldap) = LdapConnAsync::from_url_with_settings(settings, &connection_url).await?;
    ldap3::drive!(conn);

    let username = escape_ldap_input(username);

    let (rs, _res) = ldap
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
        .await?
        .success()
        .map_err(|e| match &e {
            ldap3::LdapError::LdapResult { result } => {
                if result.rc == 32 {
                    BathUserError::UserNotExists
                } else {
                    BathUserError::LDAPError(e)
                }
            }
            _ => BathUserError::LDAPError(e),
        })?;

    if rs.is_empty() {
        return Err(BathUserError::UserIsNotStudent);
    }

    ldap.unbind().await?;
    Ok(())
}
