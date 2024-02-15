use actix_web::{post, web, Responder};
use bhw_types::requests::sign_up::{SignUpError, SignUpRequest, SignUpResponse};

use crate::{
    data::auth::check_username_exists, db::DbPool, models::signup_requests::SignupRequestObject,
};

#[post("/auth/signup")]
pub async fn sign_up_route(
    request: web::Json<SignUpRequest>,
    db: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    web::block(move || -> Result<(), SignUpError> {
        let mut conn = db.get().map_err(|_| SignUpError::DBError)?;

        conn.build_transaction()
            .serializable()
            .run(|mut tx| -> Result<(), SignUpError> {
                let exists = check_username_exists(&mut tx, request.bath_username.clone())
                    .map_err(|_| SignUpError::DBError)?;
                if exists {
                    return Err(SignUpError::UsernameAlreadyExists);
                }

                let new_sr = SignupRequestObject::create(&mut tx, request.bath_username.clone())
                    .map_err(|e| SignUpError::CreateError(e.to_string()))?;

                println!("new_sr: {} {}", new_sr.id, new_sr.secret);

                Ok(())
            })
    })
    .await??;

    Ok(SignUpResponse::default())
}
