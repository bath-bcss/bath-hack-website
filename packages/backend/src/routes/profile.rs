use actix_web::{get, post, web};
use bhw_types::{
    nothing::Nothing,
    requests::{
        profile::{ProfileResponse, ProfileResponseError, ProfileResult},
        update_profile::{UpdateProfileRequest, UpdateProfileResponseError, UpdateProfileResult},
    },
};
use log::{error, warn};

use crate::{
    data::session::SessionUser,
    db::DbPool,
    models::users::{User, UserFromIdError},
};

#[get("/profile")]
pub async fn get_profile_route(user: SessionUser, db: web::Data<DbPool>) -> ProfileResult {
    let profile = web::block(move || -> ProfileResult {
        let mut conn = db.get().map_err(|e| {
            error!("get db from pool: {}", e.to_string());
            ProfileResponseError::DBError
        })?;

        conn.build_transaction()
            .serializable()
            .run(|mut tx| -> ProfileResult {
                let user = User::from_id(&mut tx, user.id.to_string())
                    .map_err(|e| match e {
                        UserFromIdError::DBError(e) => {
                            warn!("finding user by ID: {}", e.to_string());
                            ProfileResponseError::DBError
                        }
                        UserFromIdError::InvalidID(e) => {
                            ProfileResponseError::InvalidID(e.to_string())
                        }
                    })?
                    .ok_or(ProfileResponseError::NotFound)?;

                let profile = ProfileResponse {
                    display_name: user.display_name,
                    bath_username: user.bath_username,
                    accessibility_requirements: user.accessibility_requirements,
                    dietary_requirements: user.dietary_requirements,
                };

                Ok(profile)
            })
    })
    .await??;

    Ok(profile)
}

#[post("/profile")]
pub async fn update_profile_route(
    user: SessionUser,
    db: web::Data<DbPool>,
    data: web::Json<UpdateProfileRequest>,
) -> UpdateProfileResult {
    web::block(move || -> UpdateProfileResult {
        let mut conn = db.get().map_err(|e| {
            error!("get db from pool: {}", e.to_string());
            UpdateProfileResponseError::DBError
        })?;

        conn.build_transaction()
            .serializable()
            .run(|mut tx| -> UpdateProfileResult {
                User::update_property(&mut tx, user.id, data.into_inner())?;
                Ok(Nothing)
            })
    })
    .await??;

    Ok(Nothing)
}
