pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240223_120013_ldap;
mod m20240227_135849_add_group_names;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240223_120013_ldap::Migration),
            Box::new(m20240227_135849_add_group_names::Migration),
        ]
    }
}
