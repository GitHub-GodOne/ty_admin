use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreProductAttrValue::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::ProductId)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::Suk)
                            .string_len(128)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::Stock)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::Sales)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::Price)
                            .decimal_len(8, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::Image)
                            .string_len(1000)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::Unique)
                            .char_len(8)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::Cost)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::BarCode)
                            .string_len(50)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::OtPrice)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::Weight)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::Volume)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::Brokerage)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::BrokerageTwo)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::Type)
                            .tiny_integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::Quota)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::QuotaShow)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::AttrValue)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::IsDel)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductAttrValue::Version)
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
                    .name("idx_ty_store_product_attr_value_unique")
                    .table(TyStoreProductAttrValue::Table)
                    .col(TyStoreProductAttrValue::Unique)
                    .col(TyStoreProductAttrValue::Suk)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_attr_value_store_id")
                    .table(TyStoreProductAttrValue::Table)
                    .col(TyStoreProductAttrValue::ProductId)
                    .col(TyStoreProductAttrValue::Suk)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreProductAttrValue::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreProductAttrValue {
    #[iden = "ty_store_product_attr_value"]
    Table,
    Id,
    #[iden = "product_id"]
    ProductId,
    Suk,
    Stock,
    Sales,
    Price,
    Image,
    Unique,
    Cost,
    #[iden = "bar_code"]
    BarCode,
    #[iden = "ot_price"]
    OtPrice,
    Weight,
    Volume,
    Brokerage,
    #[iden = "brokerage_two"]
    BrokerageTwo,
    Type,
    Quota,
    #[iden = "quota_show"]
    QuotaShow,
    #[iden = "attr_value"]
    AttrValue,
    #[iden = "is_del"]
    IsDel,
    Version,
}
