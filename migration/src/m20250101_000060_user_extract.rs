use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyUserExtract::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyUserExtract::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyUserExtract::Uid)
                            .unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUserExtract::RealName)
                            .string_len(64)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUserExtract::ExtractType)
                            .string_len(32)
                            .null()
                            .default("BANK"),
                    )
                    .col(
                        ColumnDef::new(TyUserExtract::BankCode)
                            .string_len(32)
                            .null()
                            .default("0"),
                    )
                    .col(
                        ColumnDef::new(TyUserExtract::BankAddress)
                            .string_len(256)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserExtract::AlipayCode)
                            .string_len(64)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserExtract::ExtractPrice)
                            .decimal_len(8, 2)
                            .null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyUserExtract::Mark)
                            .string_len(512)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUserExtract::Balance)
                            .decimal_len(8, 2)
                            .null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyUserExtract::FailMsg)
                            .string_len(128)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUserExtract::Status)
                            .tiny_integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserExtract::Wechat)
                            .string_len(15)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUserExtract::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyUserExtract::UpdateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyUserExtract::FailTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyUserExtract::BankName)
                            .string_len(512)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUserExtract::QrcodeUrl)
                            .string_len(512)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_extract_extract_type")
                    .table(TyUserExtract::Table)
                    .col(TyUserExtract::ExtractType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_extract_status")
                    .table(TyUserExtract::Table)
                    .col(TyUserExtract::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_extract_openid")
                    .table(TyUserExtract::Table)
                    .col(TyUserExtract::Uid)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyUserExtract::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyUserExtract {
    #[iden = "ty_user_extract"]
    Table,
    Id,
    Uid,
    #[iden = "real_name"]
    RealName,
    #[iden = "extract_type"]
    ExtractType,
    #[iden = "bank_code"]
    BankCode,
    #[iden = "bank_address"]
    BankAddress,
    #[iden = "alipay_code"]
    AlipayCode,
    #[iden = "extract_price"]
    ExtractPrice,
    Mark,
    Balance,
    #[iden = "fail_msg"]
    FailMsg,
    Status,
    Wechat,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
    #[iden = "fail_time"]
    FailTime,
    #[iden = "bank_name"]
    BankName,
    #[iden = "qrcode_url"]
    QrcodeUrl,
}
