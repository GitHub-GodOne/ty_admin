use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreCoupon::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreCoupon::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::Name)
                            .string_len(64)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::Money)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::IsLimited)
                            .tiny_integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::Total)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::LastTotal)
                            .integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::UseType)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::PrimaryKey)
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::MinPrice)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::ReceiveStartTime)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::ReceiveEndTime)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::IsFixedTime)
                            .tiny_integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::UseStartTime)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::UseEndTime)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::Day)
                            .integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::Type)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::Sort)
                            .unsigned()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::Status)
                            .tiny_unsigned()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::IsDel)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyStoreCoupon::UpdateTime)
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
                    .name("idx_ty_store_coupon_state")
                    .table(TyStoreCoupon::Table)
                    .col(TyStoreCoupon::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_coupon_is_del")
                    .table(TyStoreCoupon::Table)
                    .col(TyStoreCoupon::IsDel)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreCoupon::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreCoupon {
    #[iden = "ty_store_coupon"]
    Table,
    Id,
    Name,
    Money,
    #[iden = "is_limited"]
    IsLimited,
    Total,
    #[iden = "last_total"]
    LastTotal,
    #[iden = "use_type"]
    UseType,
    #[iden = "primary_key"]
    PrimaryKey,
    #[iden = "min_price"]
    MinPrice,
    #[iden = "receive_start_time"]
    ReceiveStartTime,
    #[iden = "receive_end_time"]
    ReceiveEndTime,
    #[iden = "is_fixed_time"]
    IsFixedTime,
    #[iden = "use_start_time"]
    UseStartTime,
    #[iden = "use_end_time"]
    UseEndTime,
    Day,
    Type,
    Sort,
    Status,
    #[iden = "is_del"]
    IsDel,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
