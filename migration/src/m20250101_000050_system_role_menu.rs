use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TySystemRoleMenu::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TySystemRoleMenu::Rid)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TySystemRoleMenu::MenuId)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TySystemRoleMenu::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TySystemRoleMenu {
    #[iden = "ty_system_role_menu"]
    Table,
    Rid,
    #[iden = "menu_id"]
    MenuId,
}
