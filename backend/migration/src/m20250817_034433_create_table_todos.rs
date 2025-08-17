use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Todos::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Todos::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Todos::Title).string_len(255).not_null())
                    .col(ColumnDef::new(Todos::Description).text())
                    .col(
                        ColumnDef::new(Todos::Completed)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Todos::CreatedAt)
                            .timestamp_with_time_zone()
                            .extra("DEFAULT (now() AT TIME ZONE 'Asia/Tokyo')")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Todos::UpdatedAt)
                            .timestamp_with_time_zone()
                            .extra("DEFAULT (now() AT TIME ZONE 'Asia/Tokyo')")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todos::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Todos {
    Table,
    Id,
    Title,
    Description,
    Completed,
    CreatedAt,
    UpdatedAt,
}
