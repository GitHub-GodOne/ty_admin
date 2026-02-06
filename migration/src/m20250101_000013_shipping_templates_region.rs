use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyShippingTemplatesRegion::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyShippingTemplatesRegion::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesRegion::TempId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesRegion::CityId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesRegion::Title)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesRegion::First)
                            .decimal_len(10, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesRegion::FirstPrice)
                            .decimal_len(10, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesRegion::Renewal)
                            .decimal_len(10, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesRegion::RenewalPrice)
                            .decimal_len(10, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesRegion::Type)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesRegion::Uniqid)
                            .string_len(32)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesRegion::Status)
                            .tiny_integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesRegion::CreateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesRegion::UpdateTime)
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
            .drop_table(Table::drop().table(TyShippingTemplatesRegion::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyShippingTemplatesRegion {
    #[iden = "ty_shipping_templates_region"]
    Table,
    Id,
    #[iden = "temp_id"]
    TempId,
    #[iden = "city_id"]
    CityId,
    Title,
    First,
    #[iden = "first_price"]
    FirstPrice,
    Renewal,
    #[iden = "renewal_price"]
    RenewalPrice,
    Type,
    Uniqid,
    Status,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
