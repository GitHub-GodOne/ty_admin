use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreProductRule::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreProductRule::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductRule::RuleName)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreProductRule::RuleValue)
                            .text()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreProductRule::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreProductRule {
    #[iden = "ty_store_product_rule"]
    Table,
    Id,
    #[iden = "rule_name"]
    RuleName,
    #[iden = "rule_value"]
    RuleValue,
}
