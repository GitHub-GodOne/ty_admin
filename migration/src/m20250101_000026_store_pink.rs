use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStorePink::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStorePink::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStorePink::Uid)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStorePink::OrderId)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStorePink::OrderIdKey)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStorePink::TotalNum)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStorePink::TotalPrice)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStorePink::Cid)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStorePink::Pid)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStorePink::People)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStorePink::Price)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStorePink::AddTime)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStorePink::StopTime)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStorePink::KId)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStorePink::IsTpl)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStorePink::IsRefund)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStorePink::Status)
                            .tiny_unsigned()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyStorePink::IsVirtual)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStorePink::Nickname)
                            .string_len(64)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyStorePink::Avatar)
                            .string_len(256)
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStorePink::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStorePink {
    #[iden = "ty_store_pink"]
    Table,
    Id,
    Uid,
    #[iden = "order_id"]
    OrderId,
    #[iden = "order_id_key"]
    OrderIdKey,
    #[iden = "total_num"]
    TotalNum,
    #[iden = "total_price"]
    TotalPrice,
    Cid,
    Pid,
    People,
    Price,
    #[iden = "add_time"]
    AddTime,
    #[iden = "stop_time"]
    StopTime,
    #[iden = "k_id"]
    KId,
    #[iden = "is_tpl"]
    IsTpl,
    #[iden = "is_refund"]
    IsRefund,
    Status,
    #[iden = "is_virtual"]
    IsVirtual,
    Nickname,
    Avatar,
}
