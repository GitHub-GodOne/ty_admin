use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TySystemFormTemp::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TySystemFormTemp::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TySystemFormTemp::Name)
                            .string_len(500)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemFormTemp::Info)
                            .string_len(500)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemFormTemp::Content)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TySystemFormTemp::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TySystemFormTemp::UpdateTime)
                            .timestamp()
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
            .drop_table(Table::drop().table(TySystemFormTemp::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TySystemFormTemp {
    #[iden = "ty_system_form_temp"]
    Table,
    Id,
    Name,
    Info,
    Content,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
