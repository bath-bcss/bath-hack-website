pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240223_120013_ldap;
mod m20240227_135849_add_group_names;
mod m20240301_150415_add_password_reset;
mod m20240309_111540_add_tshirt_size;
mod m20240325_224148_add_committee_flag;
mod m20240406_074034_add_attendance;
mod m20240407_144512_multi_registration;
mod m20240412_105947_internal_notes;
mod m20240414_042102_group_number;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240223_120013_ldap::Migration),
            Box::new(m20240227_135849_add_group_names::Migration),
            Box::new(m20240301_150415_add_password_reset::Migration),
            Box::new(m20240309_111540_add_tshirt_size::Migration),
            Box::new(m20240325_224148_add_committee_flag::Migration),
            Box::new(m20240406_074034_add_attendance::Migration),
            Box::new(m20240407_144512_multi_registration::Migration),
            Box::new(m20240412_105947_internal_notes::Migration),
            Box::new(m20240414_042102_group_number::Migration),
        ]
    }
}

