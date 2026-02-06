use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TySystemAdmin::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TySystemAdmin::Id)
                            .small_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TySystemAdmin::Account)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TySystemAdmin::Pwd)
                            .char_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TySystemAdmin::RealName)
                            .string_len(16)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TySystemAdmin::Roles)
                            .string_len(128)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TySystemAdmin::LastIp)
                            .string_len(16)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TySystemAdmin::UpdateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TySystemAdmin::CreateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TySystemAdmin::LoginCount)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TySystemAdmin::Level)
                            .tiny_unsigned()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TySystemAdmin::Status)
                            .tiny_unsigned()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TySystemAdmin::IsDel)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TySystemAdmin::Phone)
                            .string_len(15)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TySystemAdmin::IsSms)
                            .tiny_unsigned()
                            .null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_system_admin_account")
                    .table(TySystemAdmin::Table)
                    .col(TySystemAdmin::Account)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_system_admin_status")
                    .table(TySystemAdmin::Table)
                    .col(TySystemAdmin::Status)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TySystemAdmin::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TySystemAdmin {
    #[iden = "ty_system_admin"]
    Table,
    Id,
    Account,
    Pwd,
    #[iden = "real_name"]
    RealName,
    Roles,
    #[iden = "last_ip"]
    LastIp,
    #[iden = "update_time"]
    UpdateTime,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "login_count"]
    LoginCount,
    Level,
    Status,
    #[iden = "is_del"]
    IsDel,
    Phone,
    #[iden = "is_sms"]
    IsSms,
}
