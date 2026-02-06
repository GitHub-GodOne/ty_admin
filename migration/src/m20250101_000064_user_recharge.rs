use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyUserRecharge::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyUserRecharge::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyUserRecharge::Uid)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUserRecharge::OrderId)
                            .string_len(32)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUserRecharge::Price)
                            .decimal_len(8, 2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUserRecharge::GivePrice)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyUserRecharge::RechargeType)
                            .string_len(32)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUserRecharge::Paid)
                            .tiny_integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserRecharge::PayTime)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUserRecharge::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyUserRecharge::RefundPrice)
                            .decimal_len(10, 2)
                            .null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyUserRecharge::IsWechatShipping)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserRecharge::OutTradeNo)
                            .string_len(32)
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_recharge_uid")
                    .table(TyUserRecharge::Table)
                    .col(TyUserRecharge::Uid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_recharge_recharge_type")
                    .table(TyUserRecharge::Table)
                    .col(TyUserRecharge::RechargeType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_recharge_paid")
                    .table(TyUserRecharge::Table)
                    .col(TyUserRecharge::Paid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_recharge_order_id")
                    .table(TyUserRecharge::Table)
                    .col(TyUserRecharge::OrderId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyUserRecharge::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyUserRecharge {
    #[iden = "ty_user_recharge"]
    Table,
    Id,
    Uid,
    #[iden = "order_id"]
    OrderId,
    Price,
    #[iden = "give_price"]
    GivePrice,
    #[iden = "recharge_type"]
    RechargeType,
    Paid,
    #[iden = "pay_time"]
    PayTime,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "refund_price"]
    RefundPrice,
    #[iden = "is_wechat_shipping"]
    IsWechatShipping,
    #[iden = "out_trade_no"]
    OutTradeNo,
}
