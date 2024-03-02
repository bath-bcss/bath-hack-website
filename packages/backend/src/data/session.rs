use actix_session::{Session, SessionExt, SessionInsertError};
use actix_web::{http::header::ContentType, web, FromRequest, HttpResponse, ResponseError};
use bhw_models::{prelude::*, website_user};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect,
};
use serde::Serialize;
use thiserror::Error;

static USER_SESSION_KEY: &str = "authenticated_user";

/// A smaller subset of the User fields.
/// These are retrieved from the database for all authenticated user requests.
#[derive(Debug, Clone)]
pub struct SessionUser {
    pub id: uuid::Uuid,
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
        let db = req.app_data::<web::Data<DatabaseConnection>>().cloned();
        let session = req.get_session();

        Box::pin(async move {
            let user_id = session
                .get::<String>(USER_SESSION_KEY)
                .map_err(|e| AuthSessionError::ReadingSession(e.to_string()))?
                .ok_or(AuthSessionError::NotAuthenticated)?;

            let db = db.ok_or(AuthSessionError::NotConnected)?;

            let user = Self::from_id(db.get_ref(), &user_id)
                .await?
                .ok_or(AuthSessionError::NotAuthenticated)?;

            Ok(user)
        })
    }
}

impl SessionUser {
    async fn from_id<C: ConnectionTrait>(
        conn: &C,
        id: &String,
    ) -> Result<Option<Self>, AuthSessionError> {
        let parsed_id =
            uuid::Uuid::parse_str(id).map_err(|e| AuthSessionError::IDNotValid(e.to_string()))?;

        let user: Option<(uuid::Uuid, String)> = WebsiteUser::find()
            .select_only()
            .column(website_user::Column::Id)
            .column(website_user::Column::BathUsername)
            .filter(website_user::Column::Id.eq(parsed_id))
            .limit(1)
            .into_tuple()
            .one(conn)
            .await
            .map_err(|e| AuthSessionError::DBError(e.to_string()))?;

        Ok(user.map(|(id, bath_username)| SessionUser { id, bath_username }))
    }

    pub fn set_id(session: &Session, new_user_id: &String) -> Result<(), SessionInsertError> {
        session.insert(USER_SESSION_KEY, new_user_id)
    }

    pub fn forget(session: &Session) {
        session.remove(USER_SESSION_KEY);
    }
}
