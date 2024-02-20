use bhw_types::requests::update_profile::UpdateProfileRequest;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use thiserror::Error;

use crate::util::passwords::PasswordManager;

#[derive(Debug, Queryable, Selectable, Identifiable, Associations, Insertable, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(crate::models::groups::Group))]
pub struct User {
    pub id: uuid::Uuid,
    pub display_name: Option<String>,
    pub bath_username: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub dietary_requirements: Option<String>,
    pub accessibility_requirements: Option<String>,
    pub group_id: Option<uuid::Uuid>,
}

#[derive(Debug, Error)]
pub enum CreateUserError {
    #[error("Hashing password: {0}")]
    PasswordHash(argon2::password_hash::Error),
    #[error("Inserting user: {0}")]
    DBError(#[from] diesel::result::Error),
}

#[derive(Debug, Error)]
pub enum UserFromIdError {
    #[error("Could not parse ID: {0}")]
    InvalidID(#[from] uuid::Error),
    #[error("Finding User failed: {0}")]
    DBError(#[from] diesel::result::Error),
}

impl User {
    pub fn validate_username(username: &String) -> bool {
        let username_regex = regex::Regex::new(r"^[a-z]{2,4}\d{2,5}$").expect("Username regex");
        username_regex.is_match(username.as_str())
    }

    pub fn check_username_exists(
        conn: &mut PgConnection,
        username: String,
    ) -> Result<bool, diesel::result::Error> {
        use crate::schema::users;

        let result_count: i64 = users::table
            .count()
            .filter(users::bath_username.eq(username))
            .limit(1)
            .get_result(conn)?;

        Ok(result_count > 0)
    }

    pub fn find_by_username(
        conn: &mut PgConnection,
        username: String,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::schema::users;

        let response: Vec<Self> = users::table
            .select(Self::as_select())
            .filter(users::bath_username.eq(username))
            .limit(1)
            .load(conn)?;

        let first = response.first().cloned();
        Ok(first)
    }

    pub fn from_id(
        conn: &mut PgConnection,
        user_id: String,
    ) -> Result<Option<Self>, UserFromIdError> {
        use crate::schema::users;

        let parsed_id = uuid::Uuid::parse_str(user_id.as_str())?;

        let response: Vec<Self> = users::table
            .select(Self::as_select())
            .filter(users::id.eq(parsed_id))
            .limit(1)
            .load(conn)?;

        let first = response.first().cloned();
        Ok(first)
    }

    pub fn create(
        conn: &mut PgConnection,
        username: &String,
        password: &String,
    ) -> Result<User, CreateUserError> {
        use crate::schema::users;

        let password_hash =
            PasswordManager::hash(&password).map_err(|e| CreateUserError::PasswordHash(e))?;

        let new_user = User {
            id: uuid::Uuid::new_v4(),
            bath_username: username.to_owned(),
            password_hash,
            created_at: Utc::now().naive_utc(),
            display_name: None,
            dietary_requirements: None,
            accessibility_requirements: None,
            group_id: None,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;

        Ok(new_user)
    }

    pub fn update_property(
        conn: &mut PgConnection,
        user_id: uuid::Uuid,
        request: UpdateProfileRequest,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::users;

        let update_query = diesel::update(users::table).filter(users::id.eq(user_id));

        match request {
            UpdateProfileRequest::DisplayName(display_name) => {
                update_query
                    .set(users::display_name.eq(display_name))
                    .execute(conn)?;
            }
            UpdateProfileRequest::AccessibilityRequirements(accessibility_requirements) => {
                update_query
                    .set(users::accessibility_requirements.eq(accessibility_requirements))
                    .execute(conn)?;
            }
            UpdateProfileRequest::DietaryRequirements(dietary_requirements) => {
                update_query
                    .set(users::dietary_requirements.eq(dietary_requirements))
                    .execute(conn)?;
            }
        };

        Ok(())
    }

    pub fn verify_password(&self, password: &String) -> Result<bool, argon2::password_hash::Error> {
        PasswordManager::verify(password, &self.password_hash)
    }
}
