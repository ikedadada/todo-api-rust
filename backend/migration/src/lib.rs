pub use sea_orm_migration::prelude::*;

mod m20250817_034433_create_table_todos;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250817_034433_create_table_todos::Migration)]
    }
}
