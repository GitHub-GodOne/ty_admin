use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreCouponUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreCouponUser::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCouponUser::CouponId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCouponUser::Cid)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCouponUser::Uid)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCouponUser::Name)
                            .string_len(32)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyStoreCouponUser::Money)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreCouponUser::MinPrice)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyStoreCouponUser::Type)
                            .string_len(32)
                            .not_null()
                            .default("SEND"),
                    )
                    .col(
                        ColumnDef::new(TyStoreCouponUser::Status)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCouponUser::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyStoreCouponUser::UpdateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyStoreCouponUser::StartTime)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCouponUser::EndTime)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCouponUser::UseTime)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCouponUser::UseType)
                            .tiny_integer()
                            .null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyStoreCouponUser::PrimaryKey)
                            .string_len(255)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_coupon_user_cid")
                    .table(TyStoreCouponUser::Table)
                    .col(TyStoreCouponUser::Cid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_coupon_user_uid")
                    .table(TyStoreCouponUser::Table)
                    .col(TyStoreCouponUser::Uid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_coupon_user_end_time")
                    .table(TyStoreCouponUser::Table)
                    .col(TyStoreCouponUser::EndTime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_coupon_user_status")
                    .table(TyStoreCouponUser::Table)
                    .col(TyStoreCouponUser::Status)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreCouponUser::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreCouponUser {
    #[iden = "ty_store_coupon_user"]
    Table,
    Id,
    #[iden = "coupon_id"]
    CouponId,
    Cid,
    Uid,
    Name,
    Money,
    #[iden = "min_price"]
    MinPrice,
    Type,
    Status,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
    #[iden = "start_time"]
    StartTime,
    #[iden = "end_time"]
    EndTime,
    #[iden = "use_time"]
    UseTime,
    #[iden = "use_type"]
    UseType,
    #[iden = "primary_key"]
    PrimaryKey,
}
