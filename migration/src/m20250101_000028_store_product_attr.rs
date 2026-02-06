use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreProductAttr::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreProductAttr::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttr::ProductId)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttr::AttrName)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttr::AttrValues)
                            .string_len(1000)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttr::Type)
                            .tiny_integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttr::IsDel)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_attr_store_id")
                    .table(TyStoreProductAttr::Table)
                    .col(TyStoreProductAttr::ProductId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreProductAttr::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreProductAttr {
    #[iden = "ty_store_product_attr"]
    Table,
    Id,
    #[iden = "product_id"]
    ProductId,
    #[iden = "attr_name"]
    AttrName,
    #[iden = "attr_values"]
    AttrValues,
    Type,
    #[iden = "is_del"]
    IsDel,
}
