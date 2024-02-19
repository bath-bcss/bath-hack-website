use actix_session::Session;
use actix_web::{get, post, web, Responder};
use bhw_types::requests::{
    check::CheckAuthResponse,
    sign_in::{SignInRequest, SignInResponseError, SignInResult},
};
use log::{error, warn};

use crate::{
    data::session::SessionUser, db::DbPool, models::users::User, util::passwords::PasswordManager,
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
    db: web::Data<DbPool>,
    session: Session,
) -> SignInResult {
    let user_id = web::block(move || -> Result<uuid::Uuid, SignInResponseError> {
        let mut conn = db.get().map_err(|e| {
            error!("getting db from pool: {}", e);
            SignInResponseError::DBError
        })?;

        conn.build_transaction().serializable().run(
            |mut tx| -> Result<uuid::Uuid, SignInResponseError> {
                let user = User::find_by_username(&mut tx, data.username.clone())
                    .map_err(|_| SignInResponseError::DBError)?
                    .ok_or_else(|| {
                        PasswordManager::dummy_verify(&data.password);
                        SignInResponseError::UsernameOrPasswordIncorrect
                    })?;

                let password_correct = user.verify_password(&data.password).map_err(|e| {
                    warn!("verifying user password: {}", e.to_string());
                    SignInResponseError::UsernameOrPasswordIncorrect
                })?;

                if !password_correct {
                    return Err(SignInResponseError::UsernameOrPasswordIncorrect);
                }

                Ok(user.id)
            },
        )
    })
    .await??;

    SessionUser::set_id(&session, &user_id.to_string()).map_err(|e| {
        error!("setting session on login: {}", e.to_string());
        SignInResponseError::SessionError
    })?;

    SignInResult::Ok(bhw_types::nothing::Nothing)
}
