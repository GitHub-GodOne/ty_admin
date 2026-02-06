use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreProduct::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreProduct::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::MerId)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::Image)
                            .string_len(256)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::SliderImage)
                            .string_len(2000)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::StoreName)
                            .string_len(128)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::StoreInfo)
                            .string_len(256)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::Keyword)
                            .string_len(256)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::BarCode)
                            .string_len(15)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::CateId)
                            .string_len(64)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::Price)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::VipPrice)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::OtPrice)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::Postage)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::UnitName)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::Sort)
                            .small_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::Sales)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::Stock)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::IsShow)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::IsHot)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::IsBenefit)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::IsBest)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::IsNew)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::AddTime)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::IsPostage)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::IsDel)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::MerUse)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::GiveIntegral)
                            .integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::Cost)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::IsSeckill)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::IsBargain)
                            .tiny_unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::IsGood)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::IsSub)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::Ficti)
                            .integer()
                            .null()
                            .default(100),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::Browse)
                            .integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::CodePath)
                            .string_len(64)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::SoureLink)
                            .string_len(255)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::VideoLink)
                            .string_len(200)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::TempId)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::SpecType)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::Activity)
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::FlatPattern)
                            .string_len(1000)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::IsRecycle)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProduct::Version)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_cate_id")
                    .table(TyStoreProduct::Table)
                    .col(TyStoreProduct::CateId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_is_hot")
                    .table(TyStoreProduct::Table)
                    .col(TyStoreProduct::IsHot)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_is_benefit")
                    .table(TyStoreProduct::Table)
                    .col(TyStoreProduct::IsBenefit)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_is_best")
                    .table(TyStoreProduct::Table)
                    .col(TyStoreProduct::IsBest)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_is_new")
                    .table(TyStoreProduct::Table)
                    .col(TyStoreProduct::IsNew)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_toggle_on_sale, is_del")
                    .table(TyStoreProduct::Table)
                    .col(TyStoreProduct::IsDel)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_price")
                    .table(TyStoreProduct::Table)
                    .col(TyStoreProduct::Price)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_is_show")
                    .table(TyStoreProduct::Table)
                    .col(TyStoreProduct::IsShow)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_sort")
                    .table(TyStoreProduct::Table)
                    .col(TyStoreProduct::Sort)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_sales")
                    .table(TyStoreProduct::Table)
                    .col(TyStoreProduct::Sales)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_add_time")
                    .table(TyStoreProduct::Table)
                    .col(TyStoreProduct::AddTime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_is_postage")
                    .table(TyStoreProduct::Table)
                    .col(TyStoreProduct::IsPostage)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreProduct::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreProduct {
    #[iden = "ty_store_product"]
    Table,
    Id,
    #[iden = "mer_id"]
    MerId,
    Image,
    #[iden = "slider_image"]
    SliderImage,
    #[iden = "store_name"]
    StoreName,
    #[iden = "store_info"]
    StoreInfo,
    Keyword,
    #[iden = "bar_code"]
    BarCode,
    #[iden = "cate_id"]
    CateId,
    Price,
    #[iden = "vip_price"]
    VipPrice,
    #[iden = "ot_price"]
    OtPrice,
    Postage,
    #[iden = "unit_name"]
    UnitName,
    Sort,
    Sales,
    Stock,
    #[iden = "is_show"]
    IsShow,
    #[iden = "is_hot"]
    IsHot,
    #[iden = "is_benefit"]
    IsBenefit,
    #[iden = "is_best"]
    IsBest,
    #[iden = "is_new"]
    IsNew,
    #[iden = "add_time"]
    AddTime,
    #[iden = "is_postage"]
    IsPostage,
    #[iden = "is_del"]
    IsDel,
    #[iden = "mer_use"]
    MerUse,
    #[iden = "give_integral"]
    GiveIntegral,
    Cost,
    #[iden = "is_seckill"]
    IsSeckill,
    #[iden = "is_bargain"]
    IsBargain,
    #[iden = "is_good"]
    IsGood,
    #[iden = "is_sub"]
    IsSub,
    Ficti,
    Browse,
    #[iden = "code_path"]
    CodePath,
    #[iden = "soure_link"]
    SoureLink,
    #[iden = "video_link"]
    VideoLink,
    #[iden = "temp_id"]
    TempId,
    #[iden = "spec_type"]
    SpecType,
    Activity,
    #[iden = "flat_pattern"]
    FlatPattern,
    #[iden = "is_recycle"]
    IsRecycle,
    Version,
}
