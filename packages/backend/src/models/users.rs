use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(crate::models::groups::Group))]
pub struct User {
    pub id: uuid::Uuid,
    pub display_name: String,
    pub bath_username: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub dietary_requirements: String,
    pub accessibility_requirements: String,
    pub group_id: Option<uuid::Uuid>,
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

    pub fn create(conn: &mut PgConnection, username: String) {
        use crate::schema::users;

        let new_user = User {

        };

        diesel::insert_into(users::table).values(records)
    }
}
