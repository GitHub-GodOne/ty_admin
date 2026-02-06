use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyWechatExceptions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyWechatExceptions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyWechatExceptions::Errcode)
                            .string_len(64)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyWechatExceptions::Errmsg)
                            .string_len(255)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyWechatExceptions::Data)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatExceptions::Remark)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatExceptions::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyWechatExceptions::UpdateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyWechatExceptions::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyWechatExceptions {
    #[iden = "ty_wechat_exceptions"]
    Table,
    Id,
    Errcode,
    Errmsg,
    Data,
    Remark,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
