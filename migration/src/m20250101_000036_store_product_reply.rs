use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreProductReply::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreProductReply::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductReply::Uid)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductReply::Oid)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductReply::Unique)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductReply::ProductId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductReply::ReplyType)
                            .string_len(32)
                            .not_null()
                            .default("PRODUCT"),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductReply::ProductScore)
                            .tiny_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductReply::ServiceScore)
                            .tiny_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductReply::Comment)
                            .string_len(512)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductReply::Pics)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductReply::MerchantReplyContent)
                            .string_len(300)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductReply::MerchantReplyTime)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductReply::IsDel)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductReply::IsReply)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductReply::Nickname)
                            .string_len(64)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductReply::Avatar)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductReply::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductReply::UpdateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductReply::Sku)
                            .string_len(128)
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_reply_parent_id")
                    .table(TyStoreProductReply::Table)
                    .col(TyStoreProductReply::ReplyType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_reply_is_del")
                    .table(TyStoreProductReply::Table)
                    .col(TyStoreProductReply::IsDel)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_reply_product_score")
                    .table(TyStoreProductReply::Table)
                    .col(TyStoreProductReply::ProductScore)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_reply_service_score")
                    .table(TyStoreProductReply::Table)
                    .col(TyStoreProductReply::ServiceScore)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_reply_uid+oid")
                    .table(TyStoreProductReply::Table)
                    .col(TyStoreProductReply::Uid)
                    .col(TyStoreProductReply::Oid)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreProductReply::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreProductReply {
    #[iden = "ty_store_product_reply"]
    Table,
    Id,
    Uid,
    Oid,
    Unique,
    #[iden = "product_id"]
    ProductId,
    #[iden = "reply_type"]
    ReplyType,
    #[iden = "product_score"]
    ProductScore,
    #[iden = "service_score"]
    ServiceScore,
    Comment,
    Pics,
    #[iden = "merchant_reply_content"]
    MerchantReplyContent,
    #[iden = "merchant_reply_time"]
    MerchantReplyTime,
    #[iden = "is_del"]
    IsDel,
    #[iden = "is_reply"]
    IsReply,
    Nickname,
    Avatar,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
    Sku,
}
