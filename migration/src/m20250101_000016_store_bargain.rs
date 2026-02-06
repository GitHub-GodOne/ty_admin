use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreBargain::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreBargain::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::ProductId)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::Title)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::Image)
                            .string_len(150)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::UnitName)
                            .string_len(16)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::Stock)
                            .unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::Sales)
                            .unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::Images)
                            .string_len(2000)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::StartTime)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::StopTime)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::StoreName)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::Price)
                            .decimal_len(8, 2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::MinPrice)
                            .decimal_len(8, 2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::Num)
                            .unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::BargainMaxPrice)
                            .decimal_len(8, 2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::BargainMinPrice)
                            .decimal_len(8, 2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::BargainNum)
                            .unsigned()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::Status)
                            .tiny_unsigned()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::GiveIntegral)
                            .integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::Info)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::Cost)
                            .decimal_len(8, 2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::Sort)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::IsHot)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::IsDel)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::AddTime)
                            .big_unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::IsPostage)
                            .tiny_unsigned()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::Postage)
                            .decimal_len(10, 2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::Rule)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::Look)
                            .unsigned()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::Share)
                            .unsigned()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::TempId)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::Weight)
                            .decimal_len(8, 2)
                            .null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::Volume)
                            .decimal_len(8, 2)
                            .null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::Quota)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::QuotaShow)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargain::PeopleNum)
                            .integer()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreBargain::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreBargain {
    #[iden = "ty_store_bargain"]
    Table,
    Id,
    #[iden = "product_id"]
    ProductId,
    Title,
    Image,
    #[iden = "unit_name"]
    UnitName,
    Stock,
    Sales,
    Images,
    #[iden = "start_time"]
    StartTime,
    #[iden = "stop_time"]
    StopTime,
    #[iden = "store_name"]
    StoreName,
    Price,
    #[iden = "min_price"]
    MinPrice,
    Num,
    #[iden = "bargain_max_price"]
    BargainMaxPrice,
    #[iden = "bargain_min_price"]
    BargainMinPrice,
    #[iden = "bargain_num"]
    BargainNum,
    Status,
    #[iden = "give_integral"]
    GiveIntegral,
    Info,
    Cost,
    Sort,
    #[iden = "is_hot"]
    IsHot,
    #[iden = "is_del"]
    IsDel,
    #[iden = "add_time"]
    AddTime,
    #[iden = "is_postage"]
    IsPostage,
    Postage,
    Rule,
    Look,
    Share,
    #[iden = "temp_id"]
    TempId,
    Weight,
    Volume,
    Quota,
    #[iden = "quota_show"]
    QuotaShow,
    #[iden = "people_num"]
    PeopleNum,
}
