use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreCombination::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreCombination::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::ProductId)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::MerId)
                            .unsigned()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::Image)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::Images)
                            .string_len(2000)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::Title)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::Attr)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::People)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::Info)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::Price)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::Sort)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::Sales)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::Stock)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::AddTime)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::IsHost)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::IsShow)
                            .tiny_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::IsDel)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::Combination)
                            .tiny_unsigned()
                            .null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::MerUse)
                            .tiny_unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::IsPostage)
                            .tiny_unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::Postage)
                            .decimal_len(10, 2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::StartTime)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::StopTime)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::EffectiveTime)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::Cost)
                            .decimal_len(10, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::Browse)
                            .integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::UnitName)
                            .string_len(32)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::TempId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::Weight)
                            .decimal_len(8, 2)
                            .null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::Volume)
                            .decimal_len(8, 2)
                            .null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::Num)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::Quota)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::QuotaShow)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::OtPrice)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::OnceNum)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCombination::VirtualRation)
                            .integer()
                            .not_null()
                            .default(100),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreCombination::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreCombination {
    #[iden = "ty_store_combination"]
    Table,
    Id,
    #[iden = "product_id"]
    ProductId,
    #[iden = "mer_id"]
    MerId,
    Image,
    Images,
    Title,
    Attr,
    People,
    Info,
    Price,
    Sort,
    Sales,
    Stock,
    #[iden = "add_time"]
    AddTime,
    #[iden = "is_host"]
    IsHost,
    #[iden = "is_show"]
    IsShow,
    #[iden = "is_del"]
    IsDel,
    Combination,
    #[iden = "mer_use"]
    MerUse,
    #[iden = "is_postage"]
    IsPostage,
    Postage,
    #[iden = "start_time"]
    StartTime,
    #[iden = "stop_time"]
    StopTime,
    #[iden = "effective_time"]
    EffectiveTime,
    Cost,
    Browse,
    #[iden = "unit_name"]
    UnitName,
    #[iden = "temp_id"]
    TempId,
    Weight,
    Volume,
    Num,
    Quota,
    #[iden = "quota_show"]
    QuotaShow,
    #[iden = "ot_price"]
    OtPrice,
    #[iden = "once_num"]
    OnceNum,
    #[iden = "virtual_ration"]
    VirtualRation,
}
