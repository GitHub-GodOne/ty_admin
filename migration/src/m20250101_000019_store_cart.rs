use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreCart::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreCart::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCart::Uid)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCart::Type)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCart::ProductId)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCart::ProductAttrUnique)
                            .string_len(16)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyStoreCart::CartNum)
                            .small_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCart::IsNew)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCart::CombinationId)
                            .unsigned()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCart::SeckillId)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCart::BargainId)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyStoreCart::CreateTime)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyStoreCart::UpdateTime)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreCart::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_cart_user_id")
                    .table(TyStoreCart::Table)
                    .col(TyStoreCart::Uid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_cart_goods_id")
                    .table(TyStoreCart::Table)
                    .col(TyStoreCart::ProductId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_cart_uid")
                    .table(TyStoreCart::Table)
                    .col(TyStoreCart::Uid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_cart_uid_2")
                    .table(TyStoreCart::Table)
                    .col(TyStoreCart::Uid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_cart_uid_3")
                    .table(TyStoreCart::Table)
                    .col(TyStoreCart::Uid)
                    .col(TyStoreCart::IsNew)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_cart_type")
                    .table(TyStoreCart::Table)
                    .col(TyStoreCart::Type)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreCart::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreCart {
    #[iden = "ty_store_cart"]
    Table,
    Id,
    Uid,
    Type,
    #[iden = "product_id"]
    ProductId,
    #[iden = "product_attr_unique"]
    ProductAttrUnique,
    #[iden = "cart_num"]
    CartNum,
    #[iden = "is_new"]
    IsNew,
    #[iden = "combination_id"]
    CombinationId,
    #[iden = "seckill_id"]
    SeckillId,
    #[iden = "bargain_id"]
    BargainId,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
    Status,
}
