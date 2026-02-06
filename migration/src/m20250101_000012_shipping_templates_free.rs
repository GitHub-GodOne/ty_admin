use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyShippingTemplatesFree::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyShippingTemplatesFree::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesFree::TempId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesFree::CityId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesFree::Title)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesFree::Number)
                            .decimal_len(10, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesFree::Price)
                            .decimal_len(10, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesFree::Type)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesFree::Uniqid)
                            .string_len(32)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesFree::Status)
                            .tiny_integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesFree::CreateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyShippingTemplatesFree::UpdateTime)
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
            .drop_table(Table::drop().table(TyShippingTemplatesFree::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyShippingTemplatesFree {
    #[iden = "ty_shipping_templates_free"]
    Table,
    Id,
    #[iden = "temp_id"]
    TempId,
    #[iden = "city_id"]
    CityId,
    Title,
    Number,
    Price,
    Type,
    Uniqid,
    Status,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
