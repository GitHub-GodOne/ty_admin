use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TySystemConfig::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TySystemConfig::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TySystemConfig::Name)
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemConfig::Title)
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemConfig::FormId)
                            .integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TySystemConfig::Value)
                            .string_len(5000)
                            .null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemConfig::Status)
                            .tiny_integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TySystemConfig::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TySystemConfig::UpdateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_system_config_status+name")
                    .table(TySystemConfig::Table)
                    .col(TySystemConfig::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_system_config_name")
                    .table(TySystemConfig::Table)
                    .col(TySystemConfig::Name)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TySystemConfig::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TySystemConfig {
    #[iden = "ty_system_config"]
    Table,
    Id,
    Name,
    Title,
    #[iden = "form_id"]
    FormId,
    Value,
    Status,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
