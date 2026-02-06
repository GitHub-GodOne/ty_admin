use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreBargainUserHelp::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreBargainUserHelp::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargainUserHelp::Uid)
                            .unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargainUserHelp::BargainId)
                            .unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargainUserHelp::BargainUserId)
                            .unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargainUserHelp::Price)
                            .decimal_len(8, 2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreBargainUserHelp::AddTime)
                            .big_unsigned()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreBargainUserHelp::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreBargainUserHelp {
    #[iden = "ty_store_bargain_user_help"]
    Table,
    Id,
    Uid,
    #[iden = "bargain_id"]
    BargainId,
    #[iden = "bargain_user_id"]
    BargainUserId,
    Price,
    #[iden = "add_time"]
    AddTime,
}
