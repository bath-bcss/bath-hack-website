use bhw_migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

use crate::app_config::AppConfig;

pub async fn init_db(config: &AppConfig) -> DatabaseConnection {
    let manager = Database::connect(&config.database_url)
        .await
        .expect("Connecting to DB");
    Migrator::up(&manager, None)
        .await
        .expect("Performing migrations");
    manager
}
