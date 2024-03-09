use actix_web::{get, post, web};
use bhw_models::sea_orm_active_enums::TShirtSizeEnum;
use bhw_types::{
    models::website_user::TShirtSize,
    nothing::Nothing,
    requests::{
        profile::{ProfileResponse, ProfileResponseError, ProfileResult},
        update_profile::{UpdateProfileRequest, UpdateProfileResult},
    },
};
use log::warn;
use sea_orm::DatabaseConnection;

use crate::{
    data::session::SessionUser,
    models::users::{UserFromIdError, UserHelper},
};

#[get("/profile")]
pub async fn get_profile_route(
    user: SessionUser,
    db: web::Data<DatabaseConnection>,
) -> ProfileResult {
    let user = UserHelper::from_id(db.get_ref(), user.id.to_string())
        .await
        .map_err(|e| match e {
            UserFromIdError::DBError(e) => {
                warn!("finding user by ID: {}", e.to_string());
                ProfileResponseError::DBError
            }
            UserFromIdError::InvalidID(e) => ProfileResponseError::InvalidID(e.to_string()),
        })?
        .ok_or(ProfileResponseError::NotFound)?;

    let profile = ProfileResponse {
        display_name: user.display_name,
        bath_username: user.bath_username,
        accessibility_requirements: user.accessibility_requirements,
        dietary_requirements: user.dietary_requirements,
        t_shirt_size: user.t_shirt_size.map(|e| match e {
            TShirtSizeEnum::S => TShirtSize::S,
            TShirtSizeEnum::M => TShirtSize::M,
            TShirtSizeEnum::L => TShirtSize::L,
            TShirtSizeEnum::Xl => TShirtSize::XL,
            TShirtSizeEnum::Xxl => TShirtSize::XXL,
            TShirtSizeEnum::Xxxl => TShirtSize::XXXL,
            TShirtSizeEnum::Xxxxl => TShirtSize::XXXXL,
        }),
    };

    Ok(profile)
}

#[post("/profile")]
pub async fn update_profile_route(
    user: SessionUser,
    db: web::Data<DatabaseConnection>,
    data: bhw_types::actix_web_validator::Json<UpdateProfileRequest>,
) -> UpdateProfileResult {
    UserHelper::update_property(db.get_ref(), user.id, data.into_inner()).await?;
    Ok(Nothing)
}
