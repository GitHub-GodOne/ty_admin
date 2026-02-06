use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyArticle::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyArticle::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyArticle::Cid)
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyArticle::Title)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyArticle::Author)
                            .string_len(255)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyArticle::ImageInput)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyArticle::Synopsis)
                            .string_len(255)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyArticle::ShareTitle)
                            .string_len(255)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyArticle::ShareSynopsis)
                            .string_len(255)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyArticle::Visit)
                            .string_len(255)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyArticle::Sort)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyArticle::Url)
                            .string_len(255)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyArticle::MediaId)
                            .string_len(100)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyArticle::Status)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyArticle::Hide)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyArticle::AdminId)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyArticle::MerId)
                            .unsigned()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyArticle::ProductId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyArticle::IsHot)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyArticle::IsBanner)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyArticle::Content)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyArticle::CreateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyArticle::UpdateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyArticle::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyArticle {
    #[iden = "ty_article"]
    Table,
    Id,
    Cid,
    Title,
    Author,
    #[iden = "image_input"]
    ImageInput,
    Synopsis,
    #[iden = "share_title"]
    ShareTitle,
    #[iden = "share_synopsis"]
    ShareSynopsis,
    Visit,
    Sort,
    Url,
    #[iden = "media_id"]
    MediaId,
    Status,
    Hide,
    #[iden = "admin_id"]
    AdminId,
    #[iden = "mer_id"]
    MerId,
    #[iden = "product_id"]
    ProductId,
    #[iden = "is_hot"]
    IsHot,
    #[iden = "is_banner"]
    IsBanner,
    Content,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
