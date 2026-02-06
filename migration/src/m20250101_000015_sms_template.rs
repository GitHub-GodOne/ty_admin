use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TySmsTemplate::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TySmsTemplate::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TySmsTemplate::TempId)
                            .string_len(20)
                            .not_null()
                            .default("0"),
                    )
                    .col(
                        ColumnDef::new(TySmsTemplate::TempType)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TySmsTemplate::Title)
                            .string_len(100)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySmsTemplate::Type)
                            .string_len(20)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySmsTemplate::TempKey)
                            .string_len(50)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySmsTemplate::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TySmsTemplate::Content)
                            .string_len(500)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TySmsTemplate::CreateTime)
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
            .drop_table(Table::drop().table(TySmsTemplate::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TySmsTemplate {
    #[iden = "ty_sms_template"]
    Table,
    Id,
    #[iden = "temp_id"]
    TempId,
    #[iden = "temp_type"]
    TempType,
    Title,
    Type,
    #[iden = "temp_key"]
    TempKey,
    Status,
    Content,
    #[iden = "create_time"]
    CreateTime,
}
