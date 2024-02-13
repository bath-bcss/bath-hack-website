use diesel::{associations::Identifiable, deserialize::Queryable, Selectable};

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
    pub id: uuid::Uuid,
    pub join_code: Option<String>,
}
