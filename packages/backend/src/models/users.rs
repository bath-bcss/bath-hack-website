use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use thiserror::Error;

use crate::util::passwords::PasswordManager;

#[derive(Queryable, Selectable, Identifiable, Associations, Insertable)]
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
    DBError(diesel::result::Error),
}

impl User {
    pub fn check_username_exists(
        conn: &mut PgConnection,
        username: String,
    ) -> Result<bool, diesel::result::Error> {
        use crate::schema::users;

        let result_count: i64 = users::table
            .count()
            .filter(users::bath_username.eq(username))
            .get_result(conn)?;

        Ok(result_count > 0)
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
            .execute(conn)
            .map_err(|e| CreateUserError::DBError(e))?;

        Ok(new_user)
    }
}
