pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_users_table;
mod m20231105_182316_create_authors_table;
mod m20231105_183108_create_books_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users_table::Migration),
            Box::new(m20231105_182316_create_authors_table::Migration),
            Box::new(m20231105_183108_create_books_table::Migration),
        ]
    }
}
