use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreProductCoupon::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreProductCoupon::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductCoupon::ProductId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductCoupon::IssueCouponId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductCoupon::AddTime)
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
            .drop_table(Table::drop().table(TyStoreProductCoupon::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreProductCoupon {
    #[iden = "ty_store_product_coupon"]
    Table,
    Id,
    #[iden = "product_id"]
    ProductId,
    #[iden = "issue_coupon_id"]
    IssueCouponId,
    #[iden = "add_time"]
    AddTime,
}
