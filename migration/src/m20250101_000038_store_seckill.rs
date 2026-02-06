use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreSeckill::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreSeckill::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::ProductId)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::Image)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::Images)
                            .string_len(2000)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::Title)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::Info)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::Price)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::Cost)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::OtPrice)
                            .decimal_len(10, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::GiveIntegral)
                            .decimal_len(10, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::Sort)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::Stock)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::Sales)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::UnitName)
                            .string_len(16)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::Postage)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::Description)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::StartTime)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::StopTime)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::CreateTime)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::Status)
                            .tiny_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::IsPostage)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::IsDel)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::Num)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::IsShow)
                            .tiny_unsigned()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::TimeId)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::TempId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::Weight)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::Volume)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::Quota)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::QuotaShow)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckill::SpecType)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_seckill_product_id")
                    .table(TyStoreSeckill::Table)
                    .col(TyStoreSeckill::ProductId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_seckill_start_time")
                    .table(TyStoreSeckill::Table)
                    .col(TyStoreSeckill::StartTime)
                    .col(TyStoreSeckill::StopTime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_seckill_is_del")
                    .table(TyStoreSeckill::Table)
                    .col(TyStoreSeckill::IsDel)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_seckill_is_show")
                    .table(TyStoreSeckill::Table)
                    .col(TyStoreSeckill::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_seckill_add_time")
                    .table(TyStoreSeckill::Table)
                    .col(TyStoreSeckill::CreateTime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_seckill_sort")
                    .table(TyStoreSeckill::Table)
                    .col(TyStoreSeckill::Sort)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_seckill_is_postage")
                    .table(TyStoreSeckill::Table)
                    .col(TyStoreSeckill::IsPostage)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreSeckill::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreSeckill {
    #[iden = "ty_store_seckill"]
    Table,
    Id,
    #[iden = "product_id"]
    ProductId,
    Image,
    Images,
    Title,
    Info,
    Price,
    Cost,
    #[iden = "ot_price"]
    OtPrice,
    #[iden = "give_integral"]
    GiveIntegral,
    Sort,
    Stock,
    Sales,
    #[iden = "unit_name"]
    UnitName,
    Postage,
    Description,
    #[iden = "start_time"]
    StartTime,
    #[iden = "stop_time"]
    StopTime,
    #[iden = "create_time"]
    CreateTime,
    Status,
    #[iden = "is_postage"]
    IsPostage,
    #[iden = "is_del"]
    IsDel,
    Num,
    #[iden = "is_show"]
    IsShow,
    #[iden = "time_id"]
    TimeId,
    #[iden = "temp_id"]
    TempId,
    Weight,
    Volume,
    Quota,
    #[iden = "quota_show"]
    QuotaShow,
    #[iden = "spec_type"]
    SpecType,
}
