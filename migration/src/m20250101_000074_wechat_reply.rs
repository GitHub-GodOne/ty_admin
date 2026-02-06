use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyWechatReply::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyWechatReply::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyWechatReply::Keywords)
                            .string_len(64)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyWechatReply::Type)
                            .string_len(32)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyWechatReply::Data)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatReply::Status)
                            .tiny_unsigned()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyWechatReply::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyWechatReply::UpdateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_wechat_reply_type")
                    .table(TyWechatReply::Table)
                    .col(TyWechatReply::Type)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_wechat_reply_status")
                    .table(TyWechatReply::Table)
                    .col(TyWechatReply::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_wechat_reply_key")
                    .table(TyWechatReply::Table)
                    .col(TyWechatReply::Keywords)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyWechatReply::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyWechatReply {
    #[iden = "ty_wechat_reply"]
    Table,
    Id,
    Keywords,
    Type,
    Data,
    Status,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
