use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreProductCate::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreProductCate::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductCate::ProductId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductCate::CateId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductCate::AddTime)
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
            .drop_table(Table::drop().table(TyStoreProductCate::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreProductCate {
    #[iden = "ty_store_product_cate"]
    Table,
    Id,
    #[iden = "product_id"]
    ProductId,
    #[iden = "cate_id"]
    CateId,
    #[iden = "add_time"]
    AddTime,
}
