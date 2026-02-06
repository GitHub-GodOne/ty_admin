use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TySystemRole::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TySystemRole::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TySystemRole::RoleName)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TySystemRole::Rules)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TySystemRole::Level)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TySystemRole::Status)
                            .tiny_unsigned()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TySystemRole::CreateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TySystemRole::UpdateTime)
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
                    .name("idx_ty_system_role_status")
                    .table(TySystemRole::Table)
                    .col(TySystemRole::Status)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TySystemRole::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TySystemRole {
    #[iden = "ty_system_role"]
    Table,
    Id,
    #[iden = "role_name"]
    RoleName,
    Rules,
    Level,
    Status,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
