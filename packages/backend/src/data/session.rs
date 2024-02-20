use actix_session::{Session, SessionExt, SessionInsertError};
use actix_web::{http::header::ContentType, web, FromRequest, HttpResponse, ResponseError};
use diesel::prelude::*;
use serde::Serialize;
use thiserror::Error;

use crate::db::DbPool;

static USER_SESSION_KEY: &str = "authenticated_user";

/// A smaller subset of the User fields.
/// These are retrieved from the database for all authenticated user requests.
#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SessionUser {
    pub id: uuid::Uuid,
    pub display_name: Option<String>,
    pub bath_username: String,
}

#[derive(Debug, Error, Serialize)]
pub enum AuthSessionError {
    #[error("Reading session: {0}")]
    ReadingSession(String),
    #[error("Not signed in")]
    NotAuthenticated,
    #[error("Database not connected")]
    NotConnected,
    #[error("User ID was not valid because: {0}")]
    IDNotValid(String),
    #[error("From DB: {0}")]
    DBError(String),
    #[error("Blocking: {0}")]
    BlockingError(String),
}

impl ResponseError for AuthSessionError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::Unauthorized()
            .content_type(ContentType::json())
            .json(self)
    }
}

impl FromRequest for SessionUser {
    type Error = AuthSessionError;
    type Future = futures_util::future::LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let db_pool = req.app_data::<web::Data<DbPool>>().cloned();
        let session = req.get_session();

        Box::pin(async move {
            let user_id = session
                .get::<String>(USER_SESSION_KEY)
                .map_err(|e| AuthSessionError::ReadingSession(e.to_string()))?
                .ok_or(AuthSessionError::NotAuthenticated)?;

            let db = db_pool.ok_or(AuthSessionError::NotConnected)?;

            let user = web::block(move || -> Result<Self, Self::Error> {
                let mut conn = db.get().map_err(|_| AuthSessionError::NotConnected)?;
                let user = Self::from_id(&mut conn, &user_id)?
                    .ok_or(AuthSessionError::NotAuthenticated)?;

                Ok(user)
            })
            .await
            .map_err(|e| AuthSessionError::BlockingError(e.to_string()))??;

            Ok(user)
        })
    }
}

impl SessionUser {
    fn from_id(conn: &mut PgConnection, id: &String) -> Result<Option<Self>, AuthSessionError> {
        use crate::schema::users;

        let parsed_id =
            uuid::Uuid::parse_str(id).map_err(|e| AuthSessionError::IDNotValid(e.to_string()))?;

        let users: Vec<SessionUser> = users::table
            .select(SessionUser::as_select())
            .filter(users::id.eq(parsed_id))
            .limit(1)
            .load(conn)
            .map_err(|e| AuthSessionError::DBError(e.to_string()))?;

        Ok(users.first().cloned())
    }

    pub fn set_id(session: &Session, new_user_id: &String) -> Result<(), SessionInsertError> {
        session.insert(USER_SESSION_KEY, new_user_id)
    }

    pub fn forget(session: &Session) {
        session.remove(USER_SESSION_KEY);
    }
}
