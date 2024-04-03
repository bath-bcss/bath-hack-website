use actix_web::{get, Responder};

#[get("/healthz")]
pub async fn healthz_route() -> impl Responder {
    "ok :)"
}
