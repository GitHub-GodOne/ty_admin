use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyUserIntegralRecord::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyUserIntegralRecord::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyUserIntegralRecord::Uid)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserIntegralRecord::LinkId)
                            .string_len(32)
                            .not_null()
                            .default("0"),
                    )
                    .col(
                        ColumnDef::new(TyUserIntegralRecord::LinkType)
                            .string_len(32)
                            .not_null()
                            .default("ORDER"),
                    )
                    .col(
                        ColumnDef::new(TyUserIntegralRecord::Type)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyUserIntegralRecord::Title)
                            .string_len(64)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserIntegralRecord::Integral)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserIntegralRecord::Balance)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserIntegralRecord::Mark)
                            .string_len(512)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserIntegralRecord::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyUserIntegralRecord::FrozenTime)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserIntegralRecord::ThawTime)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserIntegralRecord::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyUserIntegralRecord::UpdateTime)
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
                    .name("idx_ty_user_integral_record_openid")
                    .table(TyUserIntegralRecord::Table)
                    .col(TyUserIntegralRecord::Uid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_integral_record_status")
                    .table(TyUserIntegralRecord::Table)
                    .col(TyUserIntegralRecord::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_integral_record_add_time")
                    .table(TyUserIntegralRecord::Table)
                    .col(TyUserIntegralRecord::CreateTime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_integral_record_type")
                    .table(TyUserIntegralRecord::Table)
                    .col(TyUserIntegralRecord::Type)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_integral_record_type_link")
                    .table(TyUserIntegralRecord::Table)
                    .col(TyUserIntegralRecord::Type)
                    .col(TyUserIntegralRecord::LinkId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyUserIntegralRecord::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyUserIntegralRecord {
    #[iden = "ty_user_integral_record"]
    Table,
    Id,
    Uid,
    #[iden = "link_id"]
    LinkId,
    #[iden = "link_type"]
    LinkType,
    Type,
    Title,
    Integral,
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
}
