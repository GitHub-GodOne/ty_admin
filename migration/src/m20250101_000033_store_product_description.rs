use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreProductDescription::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreProductDescription::ProductId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductDescription::Description)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductDescription::Type)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductDescription::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_description_product_id")
                    .table(TyStoreProductDescription::Table)
                    .col(TyStoreProductDescription::ProductId)
                    .col(TyStoreProductDescription::Type)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreProductDescription::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreProductDescription {
    #[iden = "ty_store_product_description"]
    Table,
    #[iden = "product_id"]
    ProductId,
    Description,
    Type,
    Id,
}
