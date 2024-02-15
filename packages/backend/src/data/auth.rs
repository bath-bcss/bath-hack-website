use diesel::prelude::*;

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
