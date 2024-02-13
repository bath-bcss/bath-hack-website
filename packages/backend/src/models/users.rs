use chrono::NaiveDateTime;
use diesel::{associations::{Associations, Identifiable}, deserialize::Queryable, Selectable};

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
