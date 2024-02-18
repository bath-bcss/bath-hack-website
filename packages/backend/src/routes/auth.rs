use actix_session::Session;
use actix_web::{get, post, web, Responder};
use bhw_types::requests::{
    check::CheckAuthResponse,
    sign_in::{SignInRequest, SignInResponse, SignInResponseError},
};
use log::warn;

use crate::{data::session::SessionUser, db::DbPool, models::users::User};

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
) -> actix_web::Result<impl Responder> {
    let user_id = web::block(move || -> Result<uuid::Uuid, SignInResponseError> {
        let mut conn = db.get().map_err(|_| SignInResponseError::DBError)?;

        conn.build_transaction().serializable().run(
            |mut tx| -> Result<uuid::Uuid, SignInResponseError> {
                let user = User::find_by_username(&mut tx, data.username.clone())
                    .map_err(|_| SignInResponseError::DBError)?
                    .ok_or(SignInResponseError::UsernameOrPasswordIncorrect)?;

                let password_correct = user
                    .verify_password(&data.password)
                    .map_err(|e| {
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

    SessionUser::set_id(&session, &user_id.to_string())
        .map_err(|_| SignInResponseError::SessionError)?;

    Ok(SignInResponse::default())
}
