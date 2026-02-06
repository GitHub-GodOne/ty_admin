use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreOrderStatus::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreOrderStatus::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderStatus::Oid)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderStatus::ChangeType)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderStatus::ChangeMessage)
                            .string_len(256)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreOrderStatus::CreateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_order_status_oid")
                    .table(TyStoreOrderStatus::Table)
                    .col(TyStoreOrderStatus::Oid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_store_order_status_change_type")
                    .table(TyStoreOrderStatus::Table)
                    .col(TyStoreOrderStatus::ChangeType)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreOrderStatus::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreOrderStatus {
    #[iden = "ty_store_order_status"]
    Table,
    Id,
    Oid,
    #[iden = "change_type"]
    ChangeType,
    #[iden = "change_message"]
    ChangeMessage,
    #[iden = "create_time"]
    CreateTime,
}
