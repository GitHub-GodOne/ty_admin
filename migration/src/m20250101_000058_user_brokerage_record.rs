use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyUserBrokerageRecord::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyUserBrokerageRecord::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyUserBrokerageRecord::Uid)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserBrokerageRecord::LinkId)
                            .string_len(32)
                            .not_null()
                            .default("0"),
                    )
                    .col(
                        ColumnDef::new(TyUserBrokerageRecord::LinkType)
                            .string_len(32)
                            .not_null()
                            .default("0"),
                    )
                    .col(
                        ColumnDef::new(TyUserBrokerageRecord::Type)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyUserBrokerageRecord::Title)
                            .string_len(64)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserBrokerageRecord::Price)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyUserBrokerageRecord::Balance)
                            .decimal_len(16, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyUserBrokerageRecord::Mark)
                            .string_len(512)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserBrokerageRecord::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyUserBrokerageRecord::FrozenTime)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserBrokerageRecord::ThawTime)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserBrokerageRecord::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyUserBrokerageRecord::UpdateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyUserBrokerageRecord::BrokerageLevel)
                            .integer()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_brokerage_record_openid")
                    .table(TyUserBrokerageRecord::Table)
                    .col(TyUserBrokerageRecord::Uid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_brokerage_record_status")
                    .table(TyUserBrokerageRecord::Table)
                    .col(TyUserBrokerageRecord::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_brokerage_record_add_time")
                    .table(TyUserBrokerageRecord::Table)
                    .col(TyUserBrokerageRecord::CreateTime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_brokerage_record_type")
                    .table(TyUserBrokerageRecord::Table)
                    .col(TyUserBrokerageRecord::Type)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_brokerage_record_type_link")
                    .table(TyUserBrokerageRecord::Table)
                    .col(TyUserBrokerageRecord::Type)
                    .col(TyUserBrokerageRecord::LinkId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyUserBrokerageRecord::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyUserBrokerageRecord {
    #[iden = "ty_user_brokerage_record"]
    Table,
    Id,
    Uid,
    #[iden = "link_id"]
    LinkId,
    #[iden = "link_type"]
    LinkType,
    Type,
    Title,
    Price,
    Balance,
    Mark,
    Status,
    #[iden = "frozen_time"]
    FrozenTime,
    #[iden = "thaw_time"]
    ThawTime,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
    #[iden = "brokerage_level"]
    BrokerageLevel,
}
