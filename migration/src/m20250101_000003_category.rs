use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyCategory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyCategory::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyCategory::Pid)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyCategory::Path)
                            .string_len(255)
                            .not_null()
                            .default("/0/"),
                    )
                    .col(
                        ColumnDef::new(TyCategory::Name)
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyCategory::Type)
                            .small_integer()
                            .null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyCategory::Url)
                            .string_len(255)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyCategory::Extra)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyCategory::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyCategory::Sort)
                            .integer()
                            .not_null()
                            .default(99999),
                    )
                    .col(
                        ColumnDef::new(TyCategory::CreateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyCategory::UpdateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_category_status+pid")
                    .table(TyCategory::Table)
                    .col(TyCategory::Pid)
                    .col(TyCategory::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_category_id+status+url")
                    .table(TyCategory::Table)
                    .col(TyCategory::Path)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyCategory::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyCategory {
    #[iden = "ty_category"]
    Table,
    Id,
    Pid,
    Path,
    Name,
    Type,
    Url,
    Extra,
    Status,
    Sort,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
