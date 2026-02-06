use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreProductLog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreProductLog::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductLog::Type)
                            .string_len(10)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductLog::ProductId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductLog::Uid)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductLog::VisitNum)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductLog::CartNum)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductLog::OrderNum)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductLog::PayNum)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductLog::PayPrice)
                            .decimal_len(10, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductLog::CostPrice)
                            .decimal_len(10, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductLog::PayUid)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductLog::RefundNum)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductLog::RefundPrice)
                            .decimal_len(10, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductLog::CollectNum)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductLog::AddTime)
                            .big_integer()
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
            .drop_table(Table::drop().table(TyStoreProductLog::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreProductLog {
    #[iden = "ty_store_product_log"]
    Table,
    Id,
    Type,
    #[iden = "product_id"]
    ProductId,
    Uid,
    #[iden = "visit_num"]
    VisitNum,
    #[iden = "cart_num"]
    CartNum,
    #[iden = "order_num"]
    OrderNum,
    #[iden = "pay_num"]
    PayNum,
    #[iden = "pay_price"]
    PayPrice,
    #[iden = "cost_price"]
    CostPrice,
    #[iden = "pay_uid"]
    PayUid,
    #[iden = "refund_num"]
    RefundNum,
    #[iden = "refund_price"]
    RefundPrice,
    #[iden = "collect_num"]
    CollectNum,
    #[iden = "add_time"]
    AddTime,
}
