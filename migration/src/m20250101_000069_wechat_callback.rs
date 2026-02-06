use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyWechatCallback::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyWechatCallback::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyWechatCallback::ToUserName)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatCallback::FromUserName)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatCallback::CreateTime)
                            .big_unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatCallback::MsgType)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatCallback::Event)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatCallback::Content)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatCallback::AddTime)
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
            .drop_table(Table::drop().table(TyWechatCallback::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyWechatCallback {
    #[iden = "ty_wechat_callback"]
    Table,
    Id,
    #[iden = "to_user_name"]
    ToUserName,
    #[iden = "from_user_name"]
    FromUserName,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "msg_type"]
    MsgType,
    Event,
    Content,
    #[iden = "add_time"]
    AddTime,
}
