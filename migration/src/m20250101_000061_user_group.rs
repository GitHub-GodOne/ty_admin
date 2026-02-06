use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyUserGroup::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyUserGroup::Id)
                            .small_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyUserGroup::GroupName)
                            .string_len(64)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyUserGroup::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyUserGroup {
    #[iden = "ty_user_group"]
    Table,
    Id,
    #[iden = "group_name"]
    GroupName,
}
