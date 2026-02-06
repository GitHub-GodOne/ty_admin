use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreProductAttrResult::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreProductAttrResult::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrResult::ProductId)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrResult::Result)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrResult::ChangeTime)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrResult::Type)
                            .tiny_integer()
                            .null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_attr_result_product_id")
                    .table(TyStoreProductAttrResult::Table)
                    .col(TyStoreProductAttrResult::ProductId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreProductAttrResult::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreProductAttrResult {
    #[iden = "ty_store_product_attr_result"]
    Table,
    Id,
    #[iden = "product_id"]
    ProductId,
    Result,
    #[iden = "change_time"]
    ChangeTime,
    Type,
}
