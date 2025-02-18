use bhw_models::{prelude::*, sea_orm_active_enums::TShirtSizeEnum, website_user};
use bhw_types::{models::website_user::TShirtSize, requests::update_profile::UpdateProfileRequest};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QuerySelect, Set,
};
use thiserror::Error;

use crate::util::passwords::PasswordManager;

#[derive(Debug, Error)]
pub enum CreateUserError {
    #[error("Hashing password: {0}")]
    PasswordHash(argon2::password_hash::Error),
    #[error("Inserting user: {0}")]
    DBError(#[from] DbErr),
}

#[derive(Debug, Error)]
pub enum UserFromIdError {
    #[error("Could not parse ID: {0}")]
    InvalidID(#[from] uuid::Error),
    #[error("Finding User failed: {0}")]
    DBError(#[from] DbErr),
}

#[derive(Debug, Error)]
pub enum UpdateUserPasswordError {
    #[error("Hashing password: {0}")]
    PasswordHash(argon2::password_hash::Error),
    #[error("Updating user password: {0}")]
    DBError(#[from] DbErr),
}

pub struct UserHelper;

impl UserHelper {
    pub fn validate_username(username: &str) -> bool {
        let username_regex = regex::Regex::new(r"^[a-z]{2,5}\d{2,5}$").expect("Username regex");
        username_regex.is_match(username)
    }

    pub async fn check_username_exists<C: ConnectionTrait>(
        conn: &C,
        username: &String,
    ) -> Result<bool, DbErr> {
        let result_count = WebsiteUser::find()
            .filter(website_user::Column::BathUsername.eq(username))
            .limit(1)
            .count(conn)
            .await?;
        Ok(result_count > 0)
    }

    pub async fn find_by_username<C: ConnectionTrait>(
        conn: &C,
        username: String,
    ) -> Result<Option<website_user::Model>, DbErr> {
        let response = WebsiteUser::find()
            .filter(website_user::Column::BathUsername.eq(username))
            .one(conn)
            .await?;

        Ok(response)
    }

    #[cfg(feature = "ldap")]
    pub async fn find_usernames_by_ldap_status<C: ConnectionTrait>(
        conn: &C,
        status: i16,
    ) -> Result<Vec<(uuid::Uuid, String)>, DbErr> {
        let response = WebsiteUser::find()
            .filter(website_user::Column::LdapCheckStatus.eq(status))
            .select_only()
            .column(website_user::Column::Id)
            .column(website_user::Column::BathUsername)
            .into_tuple()
            .all(conn)
            .await?;

        Ok(response)
    }

    pub async fn from_id<C: ConnectionTrait>(
        conn: &C,
        user_id: String,
    ) -> Result<Option<website_user::Model>, UserFromIdError> {
        let parsed_id = uuid::Uuid::parse_str(user_id.as_str())?;

        let response = WebsiteUser::find()
            .filter(website_user::Column::Id.eq(parsed_id))
            .one(conn)
            .await?;

        Ok(response)
    }

    pub async fn create<C: ConnectionTrait>(
        conn: &C,
        username: &str,
        password: &str,
        ldap_check_status: i16,
    ) -> Result<website_user::Model, CreateUserError> {
        let password_hash =
            PasswordManager::hash(password).map_err(CreateUserError::PasswordHash)?;

        let new_user = website_user::ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            bath_username: Set(username.to_owned()),
            password_hash: Set(password_hash),
            ldap_check_status: Set(ldap_check_status),
            ..Default::default()
        };

        let model = new_user.insert(conn).await?;
        Ok(model)
    }

    pub async fn update_property<C: ConnectionTrait>(
        conn: &C,
        user_id: uuid::Uuid,
        request: UpdateProfileRequest,
    ) -> Result<(), DbErr> {
        let mut updated_user = website_user::ActiveModel {
            id: Set(user_id),
            ..Default::default()
        };

        // i know this is horrible
        if let Some(display_name) = request.display_name {
            if display_name.is_empty() {
                updated_user.display_name = Set(None)
            } else {
                updated_user.display_name = Set(Some(display_name));
            }
        }
        if let Some(accessibility_requirements) = request.accessibility_requirements {
            if accessibility_requirements.is_empty() {
                updated_user.accessibility_requirements = Set(None)
            } else {
                updated_user.accessibility_requirements = Set(Some(accessibility_requirements));
            }
        }
        if let Some(dietary_requirements) = request.dietary_requirements {
            if dietary_requirements.is_empty() {
                updated_user.dietary_requirements = Set(None)
            } else {
                updated_user.dietary_requirements = Set(Some(dietary_requirements));
            }
        }
        if let Some(t_shirt_size) = request.t_shirt_size {
            updated_user.t_shirt_size = Set(Some(match t_shirt_size {
                TShirtSize::S => TShirtSizeEnum::S,
                TShirtSize::M => TShirtSizeEnum::M,
                TShirtSize::L => TShirtSizeEnum::L,
                TShirtSize::XL => TShirtSizeEnum::Xl,
                TShirtSize::XXL => TShirtSizeEnum::Xxl,
                TShirtSize::XXXL => TShirtSizeEnum::Xxxl,
                TShirtSize::XXXXL => TShirtSizeEnum::Xxxxl,
            }))
        }

        updated_user.save(conn).await?;
        Ok(())
    }

    pub async fn set_group_id<C: ConnectionTrait>(
        conn: &C,
        user_id: uuid::Uuid,
        group_id: Option<uuid::Uuid>,
    ) -> Result<(), DbErr> {
        let updated_user = website_user::ActiveModel {
            id: Set(user_id),
            group_id: Set(group_id),
            ..Default::default()
        };

        updated_user.save(conn).await?;
        Ok(())
    }

    #[cfg(feature = "ldap")]
    pub async fn set_ldap_status<C: ConnectionTrait>(
        conn: &C,
        id: &uuid::Uuid,
        new_status: i16,
    ) -> Result<(), DbErr> {
        let updated_user = website_user::ActiveModel {
            id: Set(*id),
            ldap_check_status: Set(new_status),
            ..Default::default()
        };

        updated_user.save(conn).await?;
        Ok(())
    }

    pub fn verify_password(
        user: &website_user::Model,
        password: &str,
    ) -> Result<bool, argon2::password_hash::Error> {
        PasswordManager::verify(password, &user.password_hash)
    }

    pub async fn update_password<C: ConnectionTrait>(
        conn: &C,
        id: &uuid::Uuid,
        new_password: &str,
    ) -> Result<(), UpdateUserPasswordError> {
        let password_hash = PasswordManager::hash(new_password)
            .map_err(UpdateUserPasswordError::PasswordHash)?;
        let updated_user = website_user::ActiveModel {
            id: Set(*id),
            password_hash: Set(password_hash),
            ..Default::default()
        };

        updated_user.save(conn).await?;
        Ok(())
    }
}
