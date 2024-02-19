use actix_session::Session;
use actix_web::{post, web, Responder};
use bhw_types::requests::{
    activate::{AccountActivateRequest, AccountActivateResponse, AccountActivateResponseError},
    sign_up::{SignUpRequest, SignUpResponse, SignUpResponseError},
};

use crate::{
    app_config::AppConfig,
    data::session::SessionUser,
    db::DbPool,
    models::{
        signup_requests::{SignupRequestFromIdError, SignupRequestObject},
        users::User,
    },
    util::passwords::PasswordManager,
};

#[post("/auth/signup")]
pub async fn sign_up_route(
    request: web::Json<SignUpRequest>,
    db: web::Data<DbPool>,
    config: web::Data<AppConfig>,
) -> actix_web::Result<impl Responder> {
    if !User::validate_username(&request.bath_username) {
        return Err(SignUpResponseError::UsernameInvalid.into());
    }

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

#[post("/auth/activate")]
pub async fn account_activate_route(
    request: web::Json<AccountActivateRequest>,
    db: web::Data<DbPool>,
    session: Session,
) -> actix_web::Result<impl Responder> {
    let new_user_id = web::block(
        move || -> Result<uuid::Uuid, AccountActivateResponseError> {
            let mut conn = db
                .get()
                .map_err(|_| AccountActivateResponseError::DBError)?;

            let new_user_id = conn.build_transaction().serializable().run(
                |mut tx| -> Result<uuid::Uuid, AccountActivateResponseError> {
                    let signup_request = SignupRequestObject::from_id(&mut tx, &request.id)
                        .map_err(|e| match e {
                            SignupRequestFromIdError::InvalidID(_) => {
                                AccountActivateResponseError::IdOrSecretWrong
                            }
                            SignupRequestFromIdError::DBError(_) => {
                                AccountActivateResponseError::DBError
                            }
                        })?
                        .ok_or_else(|| {
                            PasswordManager::dummy_verify(&request.secret);
                            AccountActivateResponseError::IdOrSecretWrong
                        })?;

                    let is_password_correct = signup_request
                        .verify_hash(&request.secret)
                        .map_err(|_| AccountActivateResponseError::SecretError)?;

                    if !is_password_correct {
                        return Err(AccountActivateResponseError::IdOrSecretWrong);
                    }

                    let new_user =
                        User::create(&mut tx, &signup_request.bath_username, &request.password)
                            .map_err(|e| {
                                AccountActivateResponseError::CreateUserError(e.to_string())
                            })?;

                    signup_request.delete(&mut tx).map_err(|e| {
                        AccountActivateResponseError::DeleteRequestError(e.to_string())
                    })?;

                    Ok(new_user.id)
                },
            )?;

            Ok(new_user_id)
        },
    )
    .await??;

    SessionUser::set_id(&session, &new_user_id.to_string())
        .map_err(|_| AccountActivateResponseError::SessionError)?;

    Ok(AccountActivateResponse::default())
}
