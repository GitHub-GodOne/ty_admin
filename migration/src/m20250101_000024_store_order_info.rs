use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreOrderInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::OrderId)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::ProductId)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::Info)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::Unique)
                            .char_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::UpdateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::OrderNo)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::ProductName)
                            .string_len(128)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::AttrValueId)
                            .unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::Image)
                            .string_len(256)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::Sku)
                            .string_len(128)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::Price)
                            .decimal_len(8, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::PayNum)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::Weight)
                            .decimal_len(8, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::Volume)
                            .decimal_len(8, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::GiveIntegral)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::IsReply)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::IsSub)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::VipPrice)
                            .decimal_len(8, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderInfo::ProductType)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_order_info_product_id")
                    .table(TyStoreOrderInfo::Table)
                    .col(TyStoreOrderInfo::ProductId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_order_info_oid")
                    .table(TyStoreOrderInfo::Table)
                    .col(TyStoreOrderInfo::OrderId)
                    .col(TyStoreOrderInfo::Unique)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreOrderInfo::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreOrderInfo {
    #[iden = "ty_store_order_info"]
    Table,
    Id,
    #[iden = "order_id"]
    OrderId,
    #[iden = "product_id"]
    ProductId,
    Info,
    Unique,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
    #[iden = "order_no"]
    OrderNo,
    #[iden = "product_name"]
    ProductName,
    #[iden = "attr_value_id"]
    AttrValueId,
    Image,
    Sku,
    Price,
    #[iden = "pay_num"]
    PayNum,
    Weight,
    Volume,
    #[iden = "give_integral"]
    GiveIntegral,
    #[iden = "is_reply"]
    IsReply,
    #[iden = "is_sub"]
    IsSub,
    #[iden = "vip_price"]
    VipPrice,
    #[iden = "product_type"]
    ProductType,
}
