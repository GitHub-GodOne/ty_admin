use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyUserTag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyUserTag::Id)
                            .small_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyUserTag::Name)
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
            .drop_table(Table::drop().table(TyUserTag::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyUserTag {
    #[iden = "ty_user_tag"]
    Table,
    Id,
    Name,
}
