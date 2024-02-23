use bhw_models::{prelude::*, user};
use bhw_types::requests::update_profile::UpdateProfileRequest;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set, ConnectionTrait};
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

pub struct UserHelper;

impl UserHelper {
    pub fn validate_username(username: &String) -> bool {
        let username_regex = regex::Regex::new(r"^[a-z]{2,4}\d{2,5}$").expect("Username regex");
        username_regex.is_match(username.as_str())
    }

    pub async fn check_username_exists<C: ConnectionTrait>(
        conn: &C,
        username: &String,
    ) -> Result<bool, DbErr> {
        let result_count = User::find()
            .filter(user::Column::BathUsername.eq(username))
            .limit(1)
            .count(conn)
            .await?;
        Ok(result_count > 0)
    }

    pub async fn find_by_username<C: ConnectionTrait>(
        conn: &C,
        username: String,
    ) -> Result<Option<user::Model>, DbErr> {
        let response = User::find()
            .filter(user::Column::BathUsername.eq(username))
            .one(conn)
            .await?;

        Ok(response)
    }

    #[cfg(feature = "ldap")]
    pub async fn find_usernames_by_ldap_status<C: ConnectionTrait>(
        conn: &C,
        status: i16,
    ) -> Result<Vec<String>, DbErr> {
        let response = User::find()
            .filter(user::Column::LdapCheckStatus.eq(status))
            .select_only()
            .column(user::Column::BathUsername)
            .into_tuple()
            .all(conn)
            .await?;

        Ok(response)
    }

    pub async fn from_id<C: ConnectionTrait>(
        conn: &C,
        user_id: String,
    ) -> Result<Option<user::Model>, UserFromIdError> {
        let parsed_id = uuid::Uuid::parse_str(user_id.as_str())?;

        let response = User::find()
            .filter(user::Column::Id.eq(parsed_id))
            .one(conn)
            .await?;

        Ok(response)
    }

    pub async fn create<C: ConnectionTrait>(
        conn: &C,
        username: &String,
        password: &String,
        ldap_check_status: i16,
    ) -> Result<user::Model, CreateUserError> {
        let password_hash =
            PasswordManager::hash(&password).map_err(|e| CreateUserError::PasswordHash(e))?;

        let new_user = user::ActiveModel {
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
        let mut updated_user = user::ActiveModel {
            id: Set(user_id),
            ..Default::default()
        };

        match request {
            UpdateProfileRequest::DisplayName(display_name) => {
                updated_user.display_name = Set(display_name);
            }
            UpdateProfileRequest::AccessibilityRequirements(accessibility_requirements) => {
                updated_user.accessibility_requirements = Set(accessibility_requirements);
            }
            UpdateProfileRequest::DietaryRequirements(dietary_requirements) => {
                updated_user.dietary_requirements = Set(dietary_requirements);
            }
        };

        updated_user.save(conn).await?;
        Ok(())
    }

    #[cfg(feature = "ldap")]
    pub async fn set_ldap_status<C: ConnectionTrait>(
        conn: &C,
        username: &String,
        new_status: i16,
    ) -> Result<u64, DbErr> {
        Ok(conn.execute(sea_orm::Statement::from_sql_and_values(sea_orm::DatabaseBackend::Postgres, "UPDATE website_user SET ldap_check_status = $1 WHERE bath_username = $2;", [sea_orm::Value::SmallInt(Some(new_status)), sea_orm::Value::String(Some(Box::from(username.to_owned())))])).await?.rows_affected())
    }

    pub fn verify_password(
        user: &user::Model,
        password: &String,
    ) -> Result<bool, argon2::password_hash::Error> {
        PasswordManager::verify(password, &user.password_hash)
    }
}
