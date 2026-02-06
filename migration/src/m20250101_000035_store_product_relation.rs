use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreProductRelation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreProductRelation::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductRelation::Uid)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductRelation::ProductId)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductRelation::Type)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductRelation::Category)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductRelation::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductRelation::UpdateTime)
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
                    .name("idx_ty_store_product_relation_type")
                    .table(TyStoreProductRelation::Table)
                    .col(TyStoreProductRelation::Type)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_relation_category")
                    .table(TyStoreProductRelation::Table)
                    .col(TyStoreProductRelation::Category)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_product_relation_uid")
                    .table(TyStoreProductRelation::Table)
                    .col(TyStoreProductRelation::Uid)
                    .col(TyStoreProductRelation::ProductId)
                    .col(TyStoreProductRelation::Type)
                    .col(TyStoreProductRelation::Category)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreProductRelation::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreProductRelation {
    #[iden = "ty_store_product_relation"]
    Table,
    Id,
    Uid,
    #[iden = "product_id"]
    ProductId,
    Type,
    Category,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
