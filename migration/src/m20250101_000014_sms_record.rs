use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TySmsRecord::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TySmsRecord::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TySmsRecord::Uid)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TySmsRecord::Phone)
                            .char_len(11)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TySmsRecord::Content)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TySmsRecord::AddIp)
                            .string_len(30)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TySmsRecord::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TySmsRecord::Template)
                            .string_len(255)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySmsRecord::Resultcode)
                            .unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TySmsRecord::RecordId)
                            .unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TySmsRecord::Memo)
                            .text()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TySmsRecord::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TySmsRecord {
    #[iden = "ty_sms_record"]
    Table,
    Id,
    Uid,
    Phone,
    Content,
    #[iden = "add_ip"]
    AddIp,
    #[iden = "create_time"]
    CreateTime,
    Template,
    Resultcode,
    #[iden = "record_id"]
    RecordId,
    Memo,
}
