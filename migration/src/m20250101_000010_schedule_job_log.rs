use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyScheduleJobLog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyScheduleJobLog::LogId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyScheduleJobLog::JobId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyScheduleJobLog::BeanName)
                            .string_len(200)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyScheduleJobLog::MethodName)
                            .string_len(100)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyScheduleJobLog::Params)
                            .string_len(2000)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyScheduleJobLog::Status)
                            .tiny_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyScheduleJobLog::Error)
                            .string_len(2000)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyScheduleJobLog::Times)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyScheduleJobLog::CreateTime)
                            .date_time()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyScheduleJobLog::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyScheduleJobLog {
    #[iden = "ty_schedule_job_log"]
    Table,
    #[iden = "log_id"]
    LogId,
    #[iden = "job_id"]
    JobId,
    #[iden = "bean_name"]
    BeanName,
    #[iden = "method_name"]
    MethodName,
    Params,
    Status,
    Error,
    Times,
    #[iden = "create_time"]
    CreateTime,
}
