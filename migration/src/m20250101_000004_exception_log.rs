use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyExceptionLog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyExceptionLog::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyExceptionLog::ExpUrl)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyExceptionLog::ExpParams)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyExceptionLog::ExpType)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyExceptionLog::ExpController)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyExceptionLog::ExpMethod)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyExceptionLog::ExpDetail)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyExceptionLog::CreateTime)
                            .date_time()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyExceptionLog::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyExceptionLog {
    #[iden = "ty_exception_log"]
    Table,
    Id,
    #[iden = "exp_url"]
    ExpUrl,
    #[iden = "exp_params"]
    ExpParams,
    #[iden = "exp_type"]
    ExpType,
    #[iden = "exp_controller"]
    ExpController,
    #[iden = "exp_method"]
    ExpMethod,
    #[iden = "exp_detail"]
    ExpDetail,
    #[iden = "create_time"]
    CreateTime,
}
