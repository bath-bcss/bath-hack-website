use actix_session::Session;
use actix_web::{get, post, web, Responder};
use bhw_types::{
    nothing::Nothing,
    requests::{
        check::CheckAuthResponse,
        sign_in::{SignInRequest, SignInResponseError, SignInResult},
        sign_out::SignOutResult,
    },
};
use log::{error, warn};
use sea_orm::{DatabaseConnection, TransactionTrait, IsolationLevel, AccessMode};

use crate::{
    data::session::SessionUser, models::users::UserHelper, util::passwords::PasswordManager,
};

#[get("/auth/check")]
pub async fn check_signed_in_route(user: Option<SessionUser>) -> impl Responder {
    web::Json(CheckAuthResponse {
        signed_in: user.is_some(),
    })
}

#[post("/auth/signin")]
pub async fn sign_in_route(
    data: web::Json<SignInRequest>,
    db: web::Data<DatabaseConnection>,
    session: Session,
) -> SignInResult {
    let txn = db.begin_with_config(Some(IsolationLevel::Serializable), Some(AccessMode::ReadOnly)).await?;
    let user = UserHelper::find_by_username(&txn, data.username.clone())
        .await
        .map_err(|e| {
            error!("finding user by username: {}", e.to_string());
            SignInResponseError::DBError
        })?
        .ok_or_else(|| {
            PasswordManager::dummy_verify(&data.password);
            SignInResponseError::UsernameOrPasswordIncorrect
        })?;

    txn.commit().await?;

    let password_correct = UserHelper::verify_password(&user, &data.password).map_err(|e| {
        warn!("verifying user password: {}", e.to_string());
        SignInResponseError::UsernameOrPasswordIncorrect
    })?;

    if !password_correct {
        return Err(SignInResponseError::UsernameOrPasswordIncorrect);
    }

    SessionUser::set_id(&session, &user.id.to_string()).map_err(|e| {
        error!("setting session on login: {}", e.to_string());
        SignInResponseError::SessionError
    })?;

    SignInResult::Ok(Nothing)
}

#[post("/auth/signout")]
pub async fn sign_out_route(session: Session) -> SignOutResult {
    SessionUser::forget(&session);
    Ok(Nothing)
}
