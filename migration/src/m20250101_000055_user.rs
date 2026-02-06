use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyUser::Uid)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyUser::Account)
                            .string_len(32)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUser::Pwd)
                            .string_len(32)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUser::RealName)
                            .string_len(25)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUser::Birthday)
                            .string_len(32)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUser::CardId)
                            .string_len(20)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUser::Mark)
                            .string_len(255)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUser::PartnerId)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUser::GroupId)
                            .string_len(255)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUser::TagId)
                            .string_len(255)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUser::Nickname)
                            .string_len(255)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUser::Avatar)
                            .string_len(256)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUser::Phone)
                            .char_len(15)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUser::AddIp)
                            .string_len(16)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUser::LastIp)
                            .string_len(16)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUser::NowMoney)
                            .decimal_len(16, 2)
                            .null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyUser::BrokeragePrice)
                            .decimal_len(8, 2)
                            .null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyUser::Integral)
                            .integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUser::Experience)
                            .unsigned()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUser::SignNum)
                            .integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUser::Status)
                            .tiny_integer()
                            .null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyUser::Level)
                            .tiny_unsigned()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUser::SpreadUid)
                            .unsigned()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUser::SpreadTime)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUser::UserType)
                            .string_len(32)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUser::IsPromoter)
                            .tiny_unsigned()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUser::PayCount)
                            .unsigned()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUser::SpreadCount)
                            .integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUser::Addres)
                            .string_len(255)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUser::Adminid)
                            .unsigned()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUser::LoginType)
                            .string_len(36)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUser::CreateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyUser::UpdateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyUser::LastLoginTime)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUser::CleanTime)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUser::Path)
                            .string_len(255)
                            .not_null()
                            .default("/0/"),
                    )
                    .col(
                        ColumnDef::new(TyUser::Subscribe)
                            .tiny_integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUser::SubscribeTime)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUser::Sex)
                            .tiny_integer()
                            .null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyUser::Country)
                            .string_len(20)
                            .null()
                            .default("CN"),
                    )
                    .col(
                        ColumnDef::new(TyUser::PromoterTime)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUser::IsLogoff)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUser::LogoffTime)
                            .timestamp()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_spreaduid")
                    .table(TyUser::Table)
                    .col(TyUser::SpreadUid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_level")
                    .table(TyUser::Table)
                    .col(TyUser::Level)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_status")
                    .table(TyUser::Table)
                    .col(TyUser::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_is_promoter")
                    .table(TyUser::Table)
                    .col(TyUser::IsPromoter)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyUser::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyUser {
    #[iden = "ty_user"]
    Table,
    Uid,
    Account,
    Pwd,
    #[iden = "real_name"]
    RealName,
    Birthday,
    #[iden = "card_id"]
    CardId,
    Mark,
    #[iden = "partner_id"]
    PartnerId,
    #[iden = "group_id"]
    GroupId,
    #[iden = "tag_id"]
    TagId,
    Nickname,
    Avatar,
    Phone,
    #[iden = "add_ip"]
    AddIp,
    #[iden = "last_ip"]
    LastIp,
    #[iden = "now_money"]
    NowMoney,
    #[iden = "brokerage_price"]
    BrokeragePrice,
    Integral,
    Experience,
    #[iden = "sign_num"]
    SignNum,
    Status,
    Level,
    #[iden = "spread_uid"]
    SpreadUid,
    #[iden = "spread_time"]
    SpreadTime,
    #[iden = "user_type"]
    UserType,
    #[iden = "is_promoter"]
    IsPromoter,
    #[iden = "pay_count"]
    PayCount,
    #[iden = "spread_count"]
    SpreadCount,
    Addres,
    Adminid,
    #[iden = "login_type"]
    LoginType,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
    #[iden = "last_login_time"]
    LastLoginTime,
    #[iden = "clean_time"]
    CleanTime,
    Path,
    Subscribe,
    #[iden = "subscribe_time"]
    SubscribeTime,
    Sex,
    Country,
    #[iden = "promoter_time"]
    PromoterTime,
    #[iden = "is_logoff"]
    IsLogoff,
    #[iden = "logoff_time"]
    LogoffTime,
}
