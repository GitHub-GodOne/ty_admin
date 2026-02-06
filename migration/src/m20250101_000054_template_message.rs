use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyTemplateMessage::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyTemplateMessage::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyTemplateMessage::Type)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyTemplateMessage::TempKey)
                            .char_len(50)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyTemplateMessage::Name)
                            .char_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyTemplateMessage::Content)
                            .string_len(1000)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyTemplateMessage::TempId)
                            .char_len(100)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyTemplateMessage::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyTemplateMessage::CreateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyTemplateMessage::UpdateTime)
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
            .drop_table(Table::drop().table(TyTemplateMessage::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyTemplateMessage {
    #[iden = "ty_template_message"]
    Table,
    Id,
    Type,
    #[iden = "temp_key"]
    TempKey,
    Name,
    Content,
    #[iden = "temp_id"]
    TempId,
    Status,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
