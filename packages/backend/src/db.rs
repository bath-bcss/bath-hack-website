use diesel::{r2d2, PgConnection};

use crate::app_config::AppConfig;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn init_db(config: AppConfig) -> DbPool {
    let manager = r2d2::ConnectionManager::<PgConnection>::new(config.database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("failed to build DB pool")
}
