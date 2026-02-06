use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TySystemStoreStaff::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TySystemStoreStaff::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TySystemStoreStaff::Uid)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TySystemStoreStaff::Avatar)
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemStoreStaff::StoreId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TySystemStoreStaff::StaffName)
                            .string_len(64)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemStoreStaff::Phone)
                            .char_len(15)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TySystemStoreStaff::VerifyStatus)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TySystemStoreStaff::Status)
                            .tiny_integer()
                            .null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TySystemStoreStaff::CreateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TySystemStoreStaff::UpdateTime)
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
            .drop_table(Table::drop().table(TySystemStoreStaff::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TySystemStoreStaff {
    #[iden = "ty_system_store_staff"]
    Table,
    Id,
    Uid,
    Avatar,
    #[iden = "store_id"]
    StoreId,
    #[iden = "staff_name"]
    StaffName,
    Phone,
    #[iden = "verify_status"]
    VerifyStatus,
    Status,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
