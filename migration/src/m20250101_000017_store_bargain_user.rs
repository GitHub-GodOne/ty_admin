use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreBargainUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreBargainUser::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargainUser::Uid)
                            .unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargainUser::BargainId)
                            .unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargainUser::BargainPriceMin)
                            .decimal_len(8, 2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargainUser::BargainPrice)
                            .decimal_len(8, 2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargainUser::Price)
                            .decimal_len(8, 2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargainUser::Status)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargainUser::AddTime)
                            .big_unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargainUser::IsDel)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreBargainUser::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreBargainUser {
    #[iden = "ty_store_bargain_user"]
    Table,
    Id,
    Uid,
    #[iden = "bargain_id"]
    BargainId,
    #[iden = "bargain_price_min"]
    BargainPriceMin,
    #[iden = "bargain_price"]
    BargainPrice,
    Price,
    Status,
    #[iden = "add_time"]
    AddTime,
    #[iden = "is_del"]
    IsDel,
}
