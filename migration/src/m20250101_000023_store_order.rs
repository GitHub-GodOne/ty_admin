use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreOrder::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreOrder::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::OrderId)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::Uid)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::RealName)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::UserPhone)
                            .string_len(18)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::UserAddress)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::FreightPrice)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::TotalNum)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::TotalPrice)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::TotalPostage)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::PayPrice)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::PayPostage)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::DeductionPrice)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::CouponId)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::CouponPrice)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::Paid)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::PayTime)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::PayType)
                            .string_len(32)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::Status)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::RefundStatus)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::RefundReasonWapImg)
                            .string_len(5000)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::RefundReasonWapExplain)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::RefundReasonWap)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::RefundReason)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::RefundReasonTime)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::RefundPrice)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::DeliveryName)
                            .string_len(64)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::DeliveryType)
                            .string_len(32)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::DeliveryId)
                            .string_len(64)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::GainIntegral)
                            .integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::UseIntegral)
                            .integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::BackIntegral)
                            .integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::Mark)
                            .string_len(512)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::IsDel)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::Remark)
                            .string_len(512)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::MerId)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::IsMerCheck)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::CombinationId)
                            .unsigned()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::PinkId)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::Cost)
                            .decimal_len(8, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::SeckillId)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::BargainId)
                            .unsigned()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::VerifyCode)
                            .string_len(12)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::StoreId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::ShippingType)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::ClerkId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::IsChannel)
                            .tiny_unsigned()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::IsRemind)
                            .tiny_unsigned()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::IsSystemDel)
                            .tiny_integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::UpdateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::DeliveryCode)
                            .string_len(50)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::BargainUserId)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::Type)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::ProTotalPrice)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::BeforePayPrice)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::IsAlterPrice)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::OutTradeNo)
                            .string_len(32)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::ShipmentPic)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::ShipmentTaskId)
                            .string_len(64)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::ShipmentOrderId)
                            .string_len(64)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::ShipmentNum)
                            .string_len(64)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrder::ExpressRecordType)
                            .integer()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_order_uid")
                    .table(TyStoreOrder::Table)
                    .col(TyStoreOrder::Uid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_order_add_time")
                    .table(TyStoreOrder::Table)
                    .col(TyStoreOrder::CreateTime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_order_pay_price")
                    .table(TyStoreOrder::Table)
                    .col(TyStoreOrder::PayPrice)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_order_paid")
                    .table(TyStoreOrder::Table)
                    .col(TyStoreOrder::Paid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_order_pay_time")
                    .table(TyStoreOrder::Table)
                    .col(TyStoreOrder::PayTime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_order_pay_type")
                    .table(TyStoreOrder::Table)
                    .col(TyStoreOrder::PayType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_order_status")
                    .table(TyStoreOrder::Table)
                    .col(TyStoreOrder::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_order_is_del")
                    .table(TyStoreOrder::Table)
                    .col(TyStoreOrder::IsDel)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_order_coupon_id")
                    .table(TyStoreOrder::Table)
                    .col(TyStoreOrder::CouponId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_order_order_id_2")
                    .table(TyStoreOrder::Table)
                    .col(TyStoreOrder::OrderId)
                    .col(TyStoreOrder::Uid)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreOrder::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreOrder {
    #[iden = "ty_store_order"]
    Table,
    Id,
    #[iden = "order_id"]
    OrderId,
    Uid,
    #[iden = "real_name"]
    RealName,
    #[iden = "user_phone"]
    UserPhone,
    #[iden = "user_address"]
    UserAddress,
    #[iden = "freight_price"]
    FreightPrice,
    #[iden = "total_num"]
    TotalNum,
    #[iden = "total_price"]
    TotalPrice,
    #[iden = "total_postage"]
    TotalPostage,
    #[iden = "pay_price"]
    PayPrice,
    #[iden = "pay_postage"]
    PayPostage,
    #[iden = "deduction_price"]
    DeductionPrice,
    #[iden = "coupon_id"]
    CouponId,
    #[iden = "coupon_price"]
    CouponPrice,
    Paid,
    #[iden = "pay_time"]
    PayTime,
    #[iden = "pay_type"]
    PayType,
    #[iden = "create_time"]
    CreateTime,
    Status,
    #[iden = "refund_status"]
    RefundStatus,
    #[iden = "refund_reason_wap_img"]
    RefundReasonWapImg,
    #[iden = "refund_reason_wap_explain"]
    RefundReasonWapExplain,
    #[iden = "refund_reason_wap"]
    RefundReasonWap,
    #[iden = "refund_reason"]
    RefundReason,
    #[iden = "refund_reason_time"]
    RefundReasonTime,
    #[iden = "refund_price"]
    RefundPrice,
    #[iden = "delivery_name"]
    DeliveryName,
    #[iden = "delivery_type"]
    DeliveryType,
    #[iden = "delivery_id"]
    DeliveryId,
    #[iden = "gain_integral"]
    GainIntegral,
    #[iden = "use_integral"]
    UseIntegral,
    #[iden = "back_integral"]
    BackIntegral,
    Mark,
    #[iden = "is_del"]
    IsDel,
    Remark,
    #[iden = "mer_id"]
    MerId,
    #[iden = "is_mer_check"]
    IsMerCheck,
    #[iden = "combination_id"]
    CombinationId,
    #[iden = "pink_id"]
    PinkId,
    Cost,
    #[iden = "seckill_id"]
    SeckillId,
    #[iden = "bargain_id"]
    BargainId,
    #[iden = "verify_code"]
    VerifyCode,
    #[iden = "store_id"]
    StoreId,
    #[iden = "shipping_type"]
    ShippingType,
    #[iden = "clerk_id"]
    ClerkId,
    #[iden = "is_channel"]
    IsChannel,
    #[iden = "is_remind"]
    IsRemind,
    #[iden = "is_system_del"]
    IsSystemDel,
    #[iden = "update_time"]
    UpdateTime,
    #[iden = "delivery_code"]
    DeliveryCode,
    #[iden = "bargain_user_id"]
    BargainUserId,
    Type,
    #[iden = "pro_total_price"]
    ProTotalPrice,
    #[iden = "before_pay_price"]
    BeforePayPrice,
    #[iden = "is_alter_price"]
    IsAlterPrice,
    #[iden = "out_trade_no"]
    OutTradeNo,
    #[iden = "shipment_pic"]
    ShipmentPic,
    #[iden = "shipment_task_id"]
    ShipmentTaskId,
    #[iden = "shipment_order_id"]
    ShipmentOrderId,
    #[iden = "shipment_num"]
    ShipmentNum,
    #[iden = "express_record_type"]
    ExpressRecordType,
}
