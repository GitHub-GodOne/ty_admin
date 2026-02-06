use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TySystemNotification::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TySystemNotification::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TySystemNotification::Mark)
                            .string_len(50)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemNotification::Type)
                            .string_len(50)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemNotification::Description)
                            .string_len(100)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemNotification::IsWechat)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TySystemNotification::WechatId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TySystemNotification::IsRoutine)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TySystemNotification::RoutineId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TySystemNotification::IsSms)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TySystemNotification::SmsId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TySystemNotification::SendType)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TySystemNotification::CreateTime)
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
                    .name("idx_ty_system_notification_mark")
                    .table(TySystemNotification::Table)
                    .col(TySystemNotification::Mark)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TySystemNotification::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TySystemNotification {
    #[iden = "ty_system_notification"]
    Table,
    Id,
    Mark,
    Type,
    Description,
    #[iden = "is_wechat"]
    IsWechat,
    #[iden = "wechat_id"]
    WechatId,
    #[iden = "is_routine"]
    IsRoutine,
    #[iden = "routine_id"]
    RoutineId,
    #[iden = "is_sms"]
    IsSms,
    #[iden = "sms_id"]
    SmsId,
    #[iden = "send_type"]
    SendType,
    #[iden = "create_time"]
    CreateTime,
}
