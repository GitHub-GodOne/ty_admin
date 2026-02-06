use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyScheduleJob::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyScheduleJob::JobId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyScheduleJob::BeanName)
                            .string_len(200)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyScheduleJob::MethodName)
                            .string_len(100)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyScheduleJob::Params)
                            .string_len(2000)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyScheduleJob::CronExpression)
                            .string_len(100)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyScheduleJob::Status)
                            .tiny_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyScheduleJob::Remark)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyScheduleJob::IsDelte)
                            .tiny_unsigned()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyScheduleJob::CreateTime)
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
            .drop_table(Table::drop().table(TyScheduleJob::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyScheduleJob {
    #[iden = "ty_schedule_job"]
    Table,
    #[iden = "job_id"]
    JobId,
    #[iden = "bean_name"]
    BeanName,
    #[iden = "method_name"]
    MethodName,
    Params,
    #[iden = "cron_expression"]
    CronExpression,
    Status,
    Remark,
    #[iden = "is_delte"]
    IsDelte,
    #[iden = "create_time"]
    CreateTime,
}
