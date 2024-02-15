use actix_web::{get, post, web, Responder};
use bhw_types::requests::{
    activate::{AccountActivateRequest, AccountActivateResponse, AccountActivateResponseError},
    sign_up::{SignUpRequest, SignUpResponse, SignUpResponseError},
};

use crate::{
    app_config::AppConfig,
    db::DbPool,
    models::{signup_requests::SignupRequestObject, users::User},
};

#[post("/auth/signup")]
pub async fn sign_up_route(
    request: web::Json<SignUpRequest>,
    db: web::Data<DbPool>,
    config: web::Data<AppConfig>,
) -> actix_web::Result<impl Responder> {
    web::block(move || -> Result<(), SignUpResponseError> {
        let mut conn = db.get().map_err(|_| SignUpResponseError::DBError)?;

        conn.build_transaction()
            .serializable()
            .run(|mut tx| -> Result<(), SignUpResponseError> {
                let exists = User::check_username_exists(&mut tx, request.bath_username.clone())
                    .map_err(|_| SignUpResponseError::DBError)?;
                if exists {
                    return Err(SignUpResponseError::UsernameAlreadyExists);
                }

                let new_sr = SignupRequestObject::create(&mut tx, request.bath_username.clone())
                    .map_err(|e| SignUpResponseError::CreateError(e.to_string()))?;

                new_sr
                    .request
                    .send_email(&config, &new_sr.secret)
                    .map_err(|e| SignUpResponseError::EmailError(e.to_string()))?;

                Ok(())
            })
    })
    .await??;

    Ok(SignUpResponse::default())
}

#[get("/auth/activate")]
pub async fn user_sign_up_activate_route(
    request: web::Json<AccountActivateRequest>,
    db: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    web::block(move || -> Result<(), AccountActivateResponseError> {
        let mut conn = db
            .get()
            .map_err(|_| AccountActivateResponseError::DBError)?;

        conn.build_transaction().serializable().run(
            |mut tx| -> Result<(), AccountActivateResponseError> {
                let signup_request = SignupRequestObject::from_id(&mut tx, &request.id)?
                    .ok_or(AccountActivateResponseError::IdOrSecretWrong)?;

                let is_password_correct = signup_request
                    .verify_hash(&request.secret)
                    .map_err(|_| AccountActivateResponseError::SecretError)?;

                if !is_password_correct {
                    return Err(AccountActivateResponseError::IdOrSecretWrong);
                }

                Ok(())
            },
        )
    })
    .await??;

    Ok(AccountActivateResponse::default())
}
