use actix_session::Session;
use actix_web::{post, web};
use bhw_types::{
    nothing::Nothing,
    requests::{
        activate::{AccountActivateRequest, AccountActivateResponseError, AccountActivateResult},
        sign_up::{SignUpRequest, SignUpResponseError, SignUpResult},
    },
};
use log::error;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

#[cfg(feature = "ldap")]
use log::warn;

use crate::{
    app_config::AppConfig,
    data::session::SessionUser,
    models::{
        ldap_status::BathUserStatus,
        signup_requests::{SignupRequestFromIdError, SignupRequestHelper},
        users::UserHelper,
    },
    util::passwords::PasswordManager,
};

#[cfg(feature = "ldap")]
use crate::ldap::{connect_ldap, get_bath_user_details};

#[post("/auth/signup")]
pub async fn sign_up_route(
    request: web::Json<SignUpRequest>,
    db: web::Data<DatabaseConnection>,
    config: web::Data<AppConfig>,
) -> SignUpResult {
    if !UserHelper::validate_username(&request.bath_username) {
        return Err(SignUpResponseError::UsernameInvalid.into());
    }

    #[cfg(feature = "ldap")]
    let status = match connect_ldap(config.get_ref().clone()).await {
        Ok(ldap) => match get_bath_user_details(&request.bath_username, ldap).await {
            Ok(v) => match v {
                BathUserStatus::None => {
                    warn!("invalid return value from get_bath_user_details");
                    Ok(BathUserStatus::None as i16)
                }
                BathUserStatus::UserIsStudent => Ok(BathUserStatus::UserIsStudent as i16),
                BathUserStatus::UserNotExists => Err(SignUpResponseError::UsernameInvalid),
                BathUserStatus::UserIsNotStudent => Err(SignUpResponseError::UserIsNotStudent),
            },
            Err(e) => {
                error!("get user details from ldap: {}", e);
                Ok(0)
            }
        },
        Err(e) => {
            error!("connection to ldap: {}", e);
            Ok(0)
        }
    }?;
    #[cfg(not(feature = "ldap"))]
    let status = 0i16;

    let txn = db
        .begin_with_config(
            Some(IsolationLevel::Serializable),
            Some(AccessMode::ReadWrite),
        )
        .await?;
    let exists = UserHelper::check_username_exists(&txn, &request.bath_username)
        .await
        .map_err(|e| {
            error!("checking username existence: {}", e.to_string());
            SignUpResponseError::DBError
        })?;
    if exists {
        return Err(SignUpResponseError::UsernameAlreadyExists);
    }

    let new_sr = SignupRequestHelper::create(&txn, &request.bath_username.clone(), status)
        .await
        .map_err(|e| SignUpResponseError::CreateError(e.to_string()))?;

    web::block(move || -> Result<(), SignUpResponseError> {
        SignupRequestHelper::send_email(&new_sr.request, &config, &new_sr.secret)
            .map_err(|e| SignUpResponseError::EmailError(e.to_string()))?;
        Ok(())
    })
    .await
    .map_err(|_| SignUpResponseError::BlockingError)??;

    txn.commit().await?;

    Ok(Nothing)
}

#[post("/auth/activate")]
pub async fn account_activate_route(
    request: web::Json<AccountActivateRequest>,
    db: web::Data<DatabaseConnection>,
    session: Session,
    config: web::Data<AppConfig>,
) -> AccountActivateResult {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::Serializable),
            Some(AccessMode::ReadWrite),
        )
        .await?;

    let signup_request = SignupRequestHelper::from_id(&txn, &request.id)
        .await
        .map_err(|e| match e {
            SignupRequestFromIdError::InvalidID(_) => AccountActivateResponseError::IdOrSecretWrong,
            SignupRequestFromIdError::DBError(e) => {
                error!("finding signup_request: {}", e.to_string());
                AccountActivateResponseError::DBError
            }
        })?
        .ok_or_else(|| {
            PasswordManager::dummy_verify(&request.secret);
            AccountActivateResponseError::IdOrSecretWrong
        })?;

    let is_password_correct = SignupRequestHelper::verify_hash(&signup_request, &request.secret)
        .map_err(|_| AccountActivateResponseError::SecretError)?;

    if !is_password_correct {
        return Err(AccountActivateResponseError::IdOrSecretWrong);
    }

    if SignupRequestHelper::expired(&signup_request) {
        return Err(AccountActivateResponseError::RequestExpired);
    }

    match signup_request.ldap_check_status.try_into() {
        Ok(BathUserStatus::None) => Ok(()),
        Ok(BathUserStatus::UserIsStudent) => Ok(()),
        Ok(BathUserStatus::UserIsNotStudent) => {
            Err(AccountActivateResponseError::UserNotStudentError)
        }
        Ok(BathUserStatus::UserNotExists) => Err(AccountActivateResponseError::PhantomUserError),
        Err(_) => Ok(()),
    }?;

    if !config.dev_weak_passwords {
        if let Err(security_error) = PasswordManager::check_security(&request.password) {
            return Err(AccountActivateResponseError::InsecurePassword(
                security_error.to_string(),
            ));
        }
    }

    let new_user = UserHelper::create(
        &txn,
        &signup_request.bath_username,
        &request.password,
        signup_request.ldap_check_status,
    )
    .await
    .map_err(|e| AccountActivateResponseError::CreateUserError(e.to_string()))?;

    SignupRequestHelper::delete(&txn, signup_request.id)
        .await
        .map_err(|e| AccountActivateResponseError::DeleteRequestError(e.to_string()))?;

    SessionUser::set_id(&session, &new_user.id.to_string()).map_err(|e| {
        error!(
            "setting user ID after activating account: {}",
            e.to_string()
        );
        AccountActivateResponseError::SessionError
    })?;

    txn.commit().await?;

    Ok(Nothing)
}
