use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyPageCategory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyPageCategory::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyPageCategory::Pid)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyPageCategory::Type)
                            .string_len(50)
                            .not_null()
                            .default("LINK"),
                    )
                    .col(
                        ColumnDef::new(TyPageCategory::Name)
                            .string_len(50)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyPageCategory::Sort)
                            .small_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyPageCategory::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyPageCategory::AddTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyPageCategory::Level)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyPageCategory::IsMer)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyPageCategory::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyPageCategory {
    #[iden = "ty_page_category"]
    Table,
    Id,
    Pid,
    Type,
    Name,
    Sort,
    Status,
    #[iden = "add_time"]
    AddTime,
    Level,
    #[iden = "is_mer"]
    IsMer,
}
