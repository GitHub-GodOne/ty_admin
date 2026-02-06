use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyWechatPayInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyWechatPayInfo::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::AppId)
                            .string_len(50)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::MchId)
                            .string_len(50)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::DeviceInfo)
                            .string_len(50)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::OpenId)
                            .string_len(50)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::NonceStr)
                            .string_len(50)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::Sign)
                            .string_len(70)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::SignType)
                            .string_len(20)
                            .null()
                            .default("MD5"),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::Body)
                            .string_len(500)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::Detail)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::Attach)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::OutTradeNo)
                            .string_len(50)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::FeeType)
                            .string_len(50)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::TotalFee)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::SpbillCreateIp)
                            .string_len(64)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::TimeStart)
                            .string_len(20)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::TimeExpire)
                            .string_len(20)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::NotifyUrl)
                            .string_len(300)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::TradeType)
                            .string_len(20)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::ProductId)
                            .string_len(32)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::SceneInfo)
                            .string_len(256)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::ErrCode)
                            .string_len(50)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::PrepayId)
                            .string_len(100)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::CodeUrl)
                            .string_len(64)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::IsSubscribe)
                            .string_len(2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::TradeState)
                            .string_len(32)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::BankType)
                            .string_len(20)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::CashFee)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::CouponFee)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::TransactionId)
                            .string_len(50)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::TimeEnd)
                            .string_len(20)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatPayInfo::TradeStateDesc)
                            .string_len(256)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_wechat_pay_info_out_trade_no")
                    .table(TyWechatPayInfo::Table)
                    .col(TyWechatPayInfo::OutTradeNo)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyWechatPayInfo::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyWechatPayInfo {
    #[iden = "ty_wechat_pay_info"]
    Table,
    Id,
    #[iden = "app_id"]
    AppId,
    #[iden = "mch_id"]
    MchId,
    #[iden = "device_info"]
    DeviceInfo,
    #[iden = "open_id"]
    OpenId,
    #[iden = "nonce_str"]
    NonceStr,
    Sign,
    #[iden = "sign_type"]
    SignType,
    Body,
    Detail,
    Attach,
    #[iden = "out_trade_no"]
    OutTradeNo,
    #[iden = "fee_type"]
    FeeType,
    #[iden = "total_fee"]
    TotalFee,
    #[iden = "spbill_create_ip"]
    SpbillCreateIp,
    #[iden = "time_start"]
    TimeStart,
    #[iden = "time_expire"]
    TimeExpire,
    #[iden = "notify_url"]
    NotifyUrl,
    #[iden = "trade_type"]
    TradeType,
    #[iden = "product_id"]
    ProductId,
    #[iden = "scene_info"]
    SceneInfo,
    #[iden = "err_code"]
    ErrCode,
    #[iden = "prepay_id"]
    PrepayId,
    #[iden = "code_url"]
    CodeUrl,
    #[iden = "is_subscribe"]
    IsSubscribe,
    #[iden = "trade_state"]
    TradeState,
    #[iden = "bank_type"]
    BankType,
    #[iden = "cash_fee"]
    CashFee,
    #[iden = "coupon_fee"]
    CouponFee,
    #[iden = "transaction_id"]
    TransactionId,
    #[iden = "time_end"]
    TimeEnd,
    #[iden = "trade_state_desc"]
    TradeStateDesc,
}
