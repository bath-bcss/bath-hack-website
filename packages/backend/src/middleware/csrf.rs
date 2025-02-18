use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    HttpResponse, ResponseError,
};
use futures_util::future::LocalBoxFuture;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CsrfMiddlewareError {
    #[error("No `Origin` header provided")]
    NoOriginHeader,
    #[error("Failed to parse `Origin` header: {0}")]
    CannotParseOrigin(String),
    #[error("Origin not allowed")]
    OriginNotAllowed,
}

impl ResponseError for CsrfMiddlewareError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::BadRequest().body(self.to_string())
    }
}

pub struct Csrf {
    pub allowed_origin: String,
}

impl<S, B> Transform<S, ServiceRequest> for Csrf
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = CsrfMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CsrfMiddleware {
            service,
            allowed_origin: self.allowed_origin.clone(),
        }))
    }
}

pub struct CsrfMiddleware<S> {
    service: S,
    allowed_origin: String,
}

impl<S, B> Service<ServiceRequest> for CsrfMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if req.path() == "/healthz" {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        let req_headers = req.headers().clone();
        let origin_header = req_headers.get(header::ORIGIN);
        match origin_header {
            None => Box::pin(async { Err(CsrfMiddlewareError::NoOriginHeader.into()) }),
            Some(origin_header) => {
                let origin = origin_header.to_str();

                match origin {
                    Err(e) => Box::pin(async move {
                        Err(CsrfMiddlewareError::CannotParseOrigin(e.to_string()).into())
                    }),
                    Ok(origin) => {
                        if origin == self.allowed_origin {
                            let fut = self.service.call(req);
                            Box::pin(async move {
                                let res = fut.await?;
                                Ok(res)
                            })
                        } else {
                            Box::pin(async {
                                Err(CsrfMiddlewareError::OriginNotAllowed.into())
                            })
                        }
                    }
                }
            }
        }
    }
}
