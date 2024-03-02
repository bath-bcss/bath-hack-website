use actix_web::{post, web};
use bhw_types::{
    nothing::Nothing,
    requests::{
        forgot_password::{
            ForgotPasswordRequest, ForgotPasswordResponseError, ForgotPasswordResult,
        },
        forgot_password_pin::{
            ForgotPasswordPINRequest, ForgotPasswordPINResponseError, ForgotPasswordPINResult,
        },
    },
};
use log::{error, info, warn};
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    app_config::AppConfig,
    data::mail::Mailer,
    models::{
        password_reset::{CreatePasswordResetError, FindAndDeletePINError, PasswordResetHelper},
        users::{UpdateUserPasswordError, UserHelper},
    },
    util::passwords::PasswordManager,
};

#[post("/auth/reset/password")]
pub async fn forgot_password_route(
    db: web::Data<DatabaseConnection>,
    data: web::Json<ForgotPasswordRequest>,
    config: web::Data<AppConfig>,
) -> ForgotPasswordResult {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::Serializable),
            Some(AccessMode::ReadWrite),
        )
        .await?;

    if PasswordResetHelper::one_exists_for_username(&txn, data.bath_username.clone()).await? {
        info!("Not handling password reset request due to existing request for username; dropping silently.");
        Mailer::fake_send_email().await;
        return Ok(Nothing);
    }

    let new_password_reset_result =
        PasswordResetHelper::create(&txn, data.bath_username.clone()).await;

    if let Err(e) = new_password_reset_result {
        return match e {
            CreatePasswordResetError::TimeError => {
                error!("adding time failed");
                Err(ForgotPasswordResponseError::DBError)
            }
            CreatePasswordResetError::UserNotFound => {
                info!("requested user for password reset does not exist, dropping silently.");
                Mailer::fake_send_email().await;
                Ok(Nothing)
            }
            CreatePasswordResetError::DBError(e) => {
                error!("creating user password reset: {}", e.to_string());
                Err(ForgotPasswordResponseError::DBError)
            }
        };
    }

    let new_password_reset = new_password_reset_result.unwrap();

    let email_result = web::block(move || {
        PasswordResetHelper::send_email(
            config.as_ref(),
            data.bath_username.clone(),
            new_password_reset.pin,
        )
    })
    .await
    .map_err(|e| {
        error!("blocking: {}", e.to_string());
        ForgotPasswordResponseError::BlockingError
    })?;

    if let Err(email_result) = email_result {
        warn!("send reset email: {}", email_result.to_string());
        return Ok(Nothing);
    }

    txn.commit().await?;

    Ok(Nothing)
}

#[post("/auth/reset/password/pin")]
pub async fn forgot_password_pin_route(
    data: web::Json<ForgotPasswordPINRequest>,
    db: web::Data<DatabaseConnection>,
) -> ForgotPasswordPINResult {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::Serializable),
            Some(AccessMode::ReadWrite),
        )
        .await?;

    let user_id = PasswordResetHelper::find_and_delete_pin(&txn, data.pin.clone())
        .await
        .map_err(|e| match e {
            FindAndDeletePINError::DBError(e) => {
                error!("find and delete password reset: {}", e.to_string());
                ForgotPasswordPINResponseError::DBError
            }
            FindAndDeletePINError::PINNotFound => ForgotPasswordPINResponseError::PIN,
        })?;

    if let Err(e) = PasswordManager::check_security(&data.new_password) {
        return Err(ForgotPasswordPINResponseError::InsecurePassword(
            e.to_string(),
        ));
    }

    UserHelper::update_password(&txn, &user_id, &data.new_password)
        .await
        .map_err(|e| match e {
            UpdateUserPasswordError::DBError(e) => {
                error!("update user password: {}", e.to_string());
                ForgotPasswordPINResponseError::DBError
            }
            UpdateUserPasswordError::PasswordHash(e) => {
                error!("hash updated user password: {}", e.to_string());
                ForgotPasswordPINResponseError::DBError
            }
        })?;

    txn.commit().await?;

    Ok(Nothing)
}
