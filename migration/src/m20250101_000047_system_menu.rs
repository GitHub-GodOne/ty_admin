use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TySystemMenu::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TySystemMenu::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TySystemMenu::Pid)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TySystemMenu::Name)
                            .string_len(100)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TySystemMenu::Icon)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TySystemMenu::Perms)
                            .string_len(200)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TySystemMenu::Component)
                            .string_len(200)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TySystemMenu::MenuType)
                            .string_len(2)
                            .null()
                            .default("M"),
                    )
                    .col(
                        ColumnDef::new(TySystemMenu::Sort)
                            .integer()
                            .not_null()
                            .default(99999),
                    )
                    .col(
                        ColumnDef::new(TySystemMenu::IsShow)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TySystemMenu::IsDelte)
                            .tiny_unsigned()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TySystemMenu::CreateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TySystemMenu::UpdateTime)
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
            .drop_table(Table::drop().table(TySystemMenu::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TySystemMenu {
    #[iden = "ty_system_menu"]
    Table,
    Id,
    Pid,
    Name,
    Icon,
    Perms,
    Component,
    #[iden = "menu_type"]
    MenuType,
    Sort,
    #[iden = "is_show"]
    IsShow,
    #[iden = "is_delte"]
    IsDelte,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
