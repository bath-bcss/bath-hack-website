use actix_web::{post, web, Responder};
use bhw_types::requests::sign_up::{SignUpRequest, SignUpResponse};

use crate::db::DbPool;

#[post("/auth/signup")]
pub async fn sign_up_route(
    request: web::Json<SignUpRequest>,
    db: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    web::block(move || {

    });

    Ok(web::Json(SignUpResponse { success: false }))
}
