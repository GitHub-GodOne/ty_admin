use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyPageLink::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyPageLink::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyPageLink::CateId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyPageLink::Type)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyPageLink::Name)
                            .string_len(50)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyPageLink::Url)
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyPageLink::Param)
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyPageLink::Example)
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyPageLink::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyPageLink::Sort)
                            .small_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyPageLink::AddTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyPageLink::IsMer)
                            .tiny_integer()
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
            .drop_table(Table::drop().table(TyPageLink::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyPageLink {
    #[iden = "ty_page_link"]
    Table,
    Id,
    #[iden = "cate_id"]
    CateId,
    Type,
    Name,
    Url,
    Param,
    Example,
    Status,
    Sort,
    #[iden = "add_time"]
    AddTime,
    #[iden = "is_mer"]
    IsMer,
}
